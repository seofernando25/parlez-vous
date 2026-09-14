pub const MANAGED_PORT: u16 = 11435;
pub const MANAGED_BASE_URL: &str = "http://127.0.0.1:11435/v1";
pub const CHAT_ALIAS: &str = "parlezvous-chat";
pub const EMBEDDING_ALIAS: &str = "parlezvous-embed";
pub const VISION_ALIAS: &str = "parlezvous-vision";
pub const CHAT_REPO: &str = "bartowski/Qwen_Qwen3.5-4B-GGUF";
pub const EMBEDDING_REPO: &str = "Qwen/Qwen3-Embedding-0.6B-GGUF";
pub const VISION_REPO: &str = "ggml-org/MiniCPM-V-4.6-GGUF";

pub fn runtime_asset_fragment() -> Option<&'static str> {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    { return Some("bin-macos-arm64.tar.gz"); }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    { return Some("bin-macos-x64.tar.gz"); }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    { return Some("bin-ubuntu-arm64.tar.gz"); }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    { return Some("bin-ubuntu-x64.tar.gz"); }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    { return Some("bin-win-cpu-x64.zip"); }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    { return Some("bin-win-cpu-arm64.zip"); }
    #[allow(unreachable_code)]
    None
}

pub fn server_binary_name() -> &'static str {
    if cfg!(target_os = "windows") { "llama-server.exe" } else { "llama-server" }
}
