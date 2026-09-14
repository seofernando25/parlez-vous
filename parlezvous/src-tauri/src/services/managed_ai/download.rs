use super::{manifest, paths::ManagedAiPaths, ManagedAiProgress};
use flate2::read::GzDecoder;
use serde::Deserialize;
use std::{fs::File, io::Write, path::{Path, PathBuf}};
use tauri::Emitter;

#[derive(Deserialize)]
struct GitHubRelease { assets: Vec<GitHubAsset> }
#[derive(Deserialize)]
struct GitHubAsset { name: String, browser_download_url: String }
#[derive(Deserialize)]
struct HfModel { siblings: Vec<HfFile> }
#[derive(Deserialize)]
struct HfFile { rfilename: String }

pub async fn install(app: &tauri::AppHandle, paths: &ManagedAiPaths) -> Result<(), String> {
    paths.ensure_dirs()?;
    let client = client()?;
    if !paths.server.exists() { install_runtime(app, &client, paths).await?; }
    if !paths.chat.exists() {
        install_model(app, &client, manifest::CHAT_REPO, "q4_k_m", &paths.chat, "Language model").await?;
    }
    if !paths.embedding.exists() {
        install_model(app, &client, manifest::EMBEDDING_REPO, "q8_0", &paths.embedding, "Search model").await?;
    }
    write_presets(paths)?;
    emit(app, "Ready", 1, 1)
}

pub async fn install_vision(app: &tauri::AppHandle, paths: &ManagedAiPaths) -> Result<(), String> {
    paths.ensure_dirs()?;
    let client = client()?;
    if !paths.vision.exists() {
        install_model(app, &client, manifest::VISION_REPO, "q4_k_m", &paths.vision, "Vision model").await?;
    }
    if !paths.vision_mmproj.exists() {
        let file = resolve_hf_file(&client, manifest::VISION_REPO, |name| {
            let lower = name.to_ascii_lowercase();
            lower.contains("mmproj") && lower.contains("q8_0")
        }).await?;
        download_file(app, &client, &hf_download_url(manifest::VISION_REPO, &file), &paths.vision_mmproj, "Vision projector").await?;
    }
    write_presets(paths)?;
    emit(app, "Vision ready", 1, 1)
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder().user_agent("ParlezVous/0.1").build().map_err(|e| e.to_string())
}

async fn install_runtime(app: &tauri::AppHandle, client: &reqwest::Client, paths: &ManagedAiPaths) -> Result<(), String> {
    let fragment = manifest::runtime_asset_fragment().ok_or("Managed AI is not packaged for this desktop platform yet")?;
    let releases: Vec<GitHubRelease> = client.get("https://api.github.com/repos/ggml-org/llama.cpp/releases?per_page=20")
        .send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;
    let asset = releases.into_iter().flat_map(|release| release.assets).find(|asset| asset.name.contains(fragment))
        .ok_or_else(|| format!("No recent llama.cpp release asset matched {fragment}"))?;
    let archive = paths.root.join(if asset.name.ends_with(".zip") { "runtime.zip" } else { "runtime.tar.gz" });
    download_file(app, client, &asset.browser_download_url, &archive, "AI runtime").await?;
    let unpack = paths.root.join("runtime-unpack");
    let _ = std::fs::remove_dir_all(&unpack);
    std::fs::create_dir_all(&unpack).map_err(|e| e.to_string())?;
    unpack_runtime(&archive, &unpack)?;
    let binary = find_file(&unpack, manifest::server_binary_name()).ok_or("llama-server was missing from the release archive")?;
    let bundle_dir = binary.parent().ok_or("Invalid llama.cpp release layout")?;
    copy_dir_contents(bundle_dir, &paths.runtime)?;
    #[cfg(unix)] {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&paths.server, std::fs::Permissions::from_mode(0o755)).map_err(|e| e.to_string())?;
    }
    let _ = std::fs::remove_file(archive);
    let _ = std::fs::remove_dir_all(unpack);
    Ok(())
}

async fn install_model(app: &tauri::AppHandle, client: &reqwest::Client, repo: &str, needle: &str, dest: &Path, label: &str) -> Result<(), String> {
    let file = resolve_hf_file(client, repo, |name| {
        let lower = name.to_ascii_lowercase();
        lower.ends_with(".gguf") && lower.contains(needle) && !lower.contains("mmproj")
    }).await?;
    download_file(app, client, &hf_download_url(repo, &file), dest, label).await
}

async fn resolve_hf_file<F: Fn(&str) -> bool>(client: &reqwest::Client, repo: &str, predicate: F) -> Result<String, String> {
    let metadata: HfModel = client.get(format!("https://huggingface.co/api/models/{repo}"))
        .send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;
    metadata.siblings.into_iter().map(|file| file.rfilename).find(|name| predicate(name))
        .ok_or_else(|| format!("No compatible GGUF found in {repo}"))
}

fn hf_download_url(repo: &str, file: &str) -> String { format!("https://huggingface.co/{repo}/resolve/main/{file}?download=true") }

async fn download_file(app: &tauri::AppHandle, client: &reqwest::Client, url: &str, dest: &Path, label: &str) -> Result<(), String> {
    let partial = dest.with_extension("part");
    let mut response = client.get(url).send().await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?;
    let total = response.content_length().unwrap_or(0);
    let mut file = File::create(&partial).map_err(|e| e.to_string())?;
    let mut downloaded = 0u64;
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        emit(app, label, downloaded, total)?;
    }
    file.flush().map_err(|e| e.to_string())?;
    std::fs::rename(partial, dest).map_err(|e| e.to_string())?;
    Ok(())
}

pub(super) fn write_presets(paths: &ManagedAiPaths) -> Result<(), String> {
    let mut preset = format!(
        "version = 1\n\n[*]\nc = 4096\n\n[{}]\nmodel = {}\njinja = true\ntemp = 0.2\ntop-p = 0.9\ntop-k = 40\nn = 1024\n\n[{}]\nmodel = {}\nembedding = true\npooling = last\n",
        manifest::CHAT_ALIAS, paths.chat.display(), manifest::EMBEDDING_ALIAS, paths.embedding.display()
    );
    if paths.vision.exists() && paths.vision_mmproj.exists() {
        preset.push_str(&format!(
            "\n[{}]\nmodel = {}\nmmproj = {}\njinja = true\nreasoning = off\ntemp = 0.7\ntop-p = 0.8\ntop-k = 100\nn = 768\n",
            manifest::VISION_ALIAS, paths.vision.display(), paths.vision_mmproj.display()
        ));
    }
    std::fs::write(&paths.presets, preset).map_err(|e| e.to_string())
}

fn unpack_runtime(archive: &Path, destination: &Path) -> Result<(), String> {
    if archive.extension().and_then(|value| value.to_str()) == Some("zip") {
        #[cfg(target_os = "windows")]
        {
            let file = File::open(archive).map_err(|e| e.to_string())?;
            let mut zip = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
            zip.extract(destination).map_err(|e| e.to_string())?;
            return Ok(());
        }
        #[cfg(not(target_os = "windows"))]
        return Err("ZIP runtime archive is only expected on Windows".into());
    }
    let decoder = GzDecoder::new(File::open(archive).map_err(|e| e.to_string())?);
    tar::Archive::new(decoder).unpack(destination).map_err(|e| e.to_string())
}

fn copy_dir_contents(source: &Path, destination: &Path) -> Result<(), String> {
    std::fs::create_dir_all(destination).map_err(|e| e.to_string())?;
    for entry in std::fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let from = entry.path();
        let to = destination.join(entry.file_name());
        if from.is_dir() { copy_dir_contents(&from, &to)?; }
        else { std::fs::copy(&from, &to).map_err(|e| e.to_string())?; }
    }
    Ok(())
}

fn find_file(root: &Path, name: &str) -> Option<PathBuf> {
    for entry in std::fs::read_dir(root).ok()?.flatten() {
        let path = entry.path();
        if path.is_dir() { if let Some(found) = find_file(&path, name) { return Some(found); } }
        else if path.file_name().and_then(|value| value.to_str()) == Some(name) { return Some(path); }
    }
    None
}

fn emit(app: &tauri::AppHandle, stage: &str, downloaded: u64, total: u64) -> Result<(), String> {
    app.emit("managed_ai_progress", ManagedAiProgress { stage: stage.to_string(), downloaded, total }).map_err(|e| e.to_string())
}
