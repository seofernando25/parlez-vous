use anyhow::{Context, Result};
use ndarray::Array3;
use ort::session::{builder::GraphOptimizationLevel, Session};
use std::{fs::File, io::BufReader};
#[cfg(feature = "xnnpack")]
use ort::execution_providers::{CPUExecutionProvider, XNNPACKExecutionProvider};
use super::{config::{load_cfgs, VoiceStyleData}, synthesis::{Style, TextToSpeech}, text::UnicodeProcessor};

// ============================================================================
// Component Loading Functions
// ============================================================================

/// Load voice style from JSON files
pub fn load_voice_style(voice_style_paths: &[String], verbose: bool) -> Result<Style> {
    let bsz = voice_style_paths.len();

    // Read first file to get dimensions
    let first_file = File::open(&voice_style_paths[0])
        .context("Failed to open voice style file")?;
    let first_reader = BufReader::new(first_file);
    let first_data: VoiceStyleData = serde_json::from_reader(first_reader)?;

    let ttl_dims = &first_data.style_ttl.dims;
    let dp_dims = &first_data.style_dp.dims;

    let ttl_dim1 = ttl_dims[1];
    let ttl_dim2 = ttl_dims[2];
    let dp_dim1 = dp_dims[1];
    let dp_dim2 = dp_dims[2];

    // Pre-allocate arrays with full batch size
    let ttl_size = bsz * ttl_dim1 * ttl_dim2;
    let dp_size = bsz * dp_dim1 * dp_dim2;
    let mut ttl_flat = vec![0.0f32; ttl_size];
    let mut dp_flat = vec![0.0f32; dp_size];

    // Fill in the data
    for (i, path) in voice_style_paths.iter().enumerate() {
        let file = File::open(path).context("Failed to open voice style file")?;
        let reader = BufReader::new(file);
        let data: VoiceStyleData = serde_json::from_reader(reader)?;

        // Flatten TTL data
        let ttl_offset = i * ttl_dim1 * ttl_dim2;
        let mut idx = 0;
        for batch in &data.style_ttl.data {
            for row in batch {
                for &val in row {
                    ttl_flat[ttl_offset + idx] = val;
                    idx += 1;
                }
            }
        }

        // Flatten DP data
        let dp_offset = i * dp_dim1 * dp_dim2;
        idx = 0;
        for batch in &data.style_dp.data {
            for row in batch {
                for &val in row {
                    dp_flat[dp_offset + idx] = val;
                    idx += 1;
                }
            }
        }
    }

    let ttl_style = Array3::from_shape_vec((bsz, ttl_dim1, ttl_dim2), ttl_flat)?;
    let dp_style = Array3::from_shape_vec((bsz, dp_dim1, dp_dim2), dp_flat)?;

    if verbose {
        println!("Loaded {} voice styles\n", bsz);
    }

    Ok(Style {
        ttl: ttl_style,
        dp: dp_style,
    })
}

/// Load and mix two voice styles
pub fn load_and_mix_voice_styles(path1: &str, path2: &str, alpha: f32) -> Result<Style> {
    let s1 = load_voice_style(&[path1.to_string()], false)?;
    let s2 = load_voice_style(&[path2.to_string()], false)?;

    if s1.ttl.dim() != s2.ttl.dim() || s1.dp.dim() != s2.dp.dim() {
        anyhow::bail!("Voice style dimensions mismatch");
    }

    let ttl = &s1.ttl * (1.0 - alpha) + &s2.ttl * alpha;
    let dp = &s1.dp * (1.0 - alpha) + &s2.dp * alpha;

    Ok(Style { ttl, dp })
}

/// Create an ONNX session with the specified execution providers
fn create_session(model_path: &str, use_gpu: bool, use_xnnpack: bool, ort_threads: usize, _xnn_threads: usize) -> Result<Session> {
    #[allow(unused_mut)]
    let mut builder = Session::builder().map_err(|e| anyhow::anyhow!("{}", e))?
        .with_optimization_level(GraphOptimizationLevel::Level3).map_err(|e| anyhow::anyhow!("{}", e))?
        .with_config_entry("session.intra_op.allow_spinning", "0").map_err(|e| anyhow::anyhow!("{}", e))?
        .with_config_entry("session.inter_op.allow_spinning", "0").map_err(|e| anyhow::anyhow!("{}", e))?
        .with_config_entry("session.inter_op.allow_spinning", "0").map_err(|e| anyhow::anyhow!("{}", e))?
        .with_intra_threads(ort_threads).map_err(|e| anyhow::anyhow!("{}", e))?;

    if use_gpu {
        #[cfg(target_os = "android")]
        {
            builder = builder.with_execution_providers([
                ort::execution_providers::NNAPIExecutionProvider::default().build(),
                ort::execution_providers::CPUExecutionProvider::default().build(),
            ]).map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        #[cfg(not(target_os = "android"))]
        {
            builder = builder.with_execution_providers([
                ort::execution_providers::CoreMLExecutionProvider::default().build(),
                ort::execution_providers::CPUExecutionProvider::default().build(),
            ]).map_err(|e| anyhow::anyhow!("{}", e))?;
        }
    } else if use_xnnpack {
        #[cfg(feature = "xnnpack")]
        {
            if let Some(xnn_threads_nz) = std::num::NonZeroUsize::new(_xnn_threads) {
                builder = builder.with_execution_providers([
                    XNNPACKExecutionProvider::default()
                        .with_intra_op_num_threads(xnn_threads_nz)
                        .build(),
                    CPUExecutionProvider::default().build(),
                ]).map_err(|e| anyhow::anyhow!("{}", e))?;
            } else {
                builder = builder.with_execution_providers([
                    XNNPACKExecutionProvider::default()
                        .with_intra_op_num_threads(std::num::NonZeroUsize::MIN)
                        .build(),
                    CPUExecutionProvider::default().build(),
                ]).map_err(|e| anyhow::anyhow!("{}", e))?;
            }
        }
    }

    builder.commit_from_file(model_path).context(format!("Failed to load model: {}", model_path))
}

/// Load TTS components
pub fn load_text_to_speech(onnx_dir: &str, use_gpu: bool, use_xnnpack: bool, ort_threads: usize, xnn_threads: usize) -> Result<TextToSpeech> {
    if use_gpu {
        log::info!("Using GPU Execution Provider with ORT ({}) threads", ort_threads);
    } else if use_xnnpack {
        log::info!("Using XNNPACK ({}) with ORT ({}) threads", xnn_threads, ort_threads);
    } else {
        log::info!("Using CPU for inference with {} threads", ort_threads);
    }

    let cfgs = load_cfgs(onnx_dir)?;

    let dp_path = format!("{}/duration_predictor.onnx", onnx_dir);
    let text_enc_path = format!("{}/text_encoder.onnx", onnx_dir);
    let vector_est_path = format!("{}/vector_estimator.onnx", onnx_dir);
    let vocoder_path = format!("{}/vocoder.onnx", onnx_dir);

    let dp_ort = create_session(&dp_path, use_gpu, use_xnnpack, ort_threads, xnn_threads)?;
    let text_enc_ort = create_session(&text_enc_path, use_gpu, use_xnnpack, ort_threads, xnn_threads)?;
    let vector_est_ort = create_session(&vector_est_path, use_gpu, use_xnnpack, ort_threads, xnn_threads)?;
    let vocoder_ort = create_session(&vocoder_path, use_gpu, use_xnnpack, ort_threads, xnn_threads)?;

    let unicode_indexer_path = format!("{}/unicode_indexer.json", onnx_dir);
    let text_processor = UnicodeProcessor::new(&unicode_indexer_path)?;

    Ok(TextToSpeech::new(
        cfgs,
        text_processor,
        dp_ort,
        text_enc_ort,
        vector_est_ort,
        vocoder_ort,
    ))
}
