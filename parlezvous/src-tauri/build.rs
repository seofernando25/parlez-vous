use burn_onnx::ModelGen;
use std::{
    env,
    path::{Path, PathBuf},
};

fn compile_optional_model(
    model_name: &str,
    cfg_name: &str,
    candidates: &[PathBuf],
    out_dir: &Path,
) {
    println!("cargo:rustc-check-cfg=cfg({cfg_name})");

    for candidate in candidates {
        println!("cargo:rerun-if-changed={}", candidate.display());
        if candidate.exists() {
            println!("cargo:rustc-cfg={cfg_name}");
            println!(
                "cargo:warning=Compiling optional {model_name} model from {}",
                candidate.display()
            );
            ModelGen::new()
                .input(candidate.to_str().expect("model path must be valid UTF-8"))
                .out_dir(out_dir.to_str().expect("output path must be valid UTF-8"))
                .run_from_script();
            return;
        }
    }

    println!(
        "cargo:warning=Optional {model_name} handwriting model not found; app will build with that recognizer disabled"
    );
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or(&manifest_dir);
    let model_dir = manifest_dir.join("models");
    let generated_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("model");

    compile_optional_model(
        "Hangul",
        "has_character_model",
        &[
            model_dir.join("character_model.onnx"),
            repo_root.join("hangulnist/character_model.onnx"),
        ],
        &generated_dir,
    );

    compile_optional_model(
        "Cyrillic",
        "has_cyrillic_model",
        &[
            model_dir.join("cyrillic_model.onnx"),
            repo_root.join("hangulnist/cyrillic_model.onnx"),
        ],
        &generated_dir,
    );

    tauri_build::build()
}
