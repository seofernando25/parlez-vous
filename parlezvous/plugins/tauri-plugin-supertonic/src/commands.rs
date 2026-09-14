use tauri::{AppHandle, command, Runtime, State, Manager};
use std::sync::Mutex;
use crate::models::*;
use crate::Result;
use crate::SupertonicExt;
use crate::helper::{load_text_to_speech, load_voice_style, TextToSpeech};

pub struct SupertonicState {
    pub tts: std::sync::Arc<Mutex<Option<TextToSpeech>>>,
}

impl Default for SupertonicState {
    fn default() -> Self {
        Self { tts: std::sync::Arc::new(Mutex::new(None)) }
    }
}

/// Wrap mono 16-bit little-endian PCM in a standard WAV container.
///
/// The frontend media pipeline expects every TTS provider to return the same
/// playable format. Keeping this conversion at the provider boundary means
/// callers do not need a Supertonic-specific playback implementation.
fn pcm16_mono_to_wav(pcm: &[u8], sample_rate: u32) -> Vec<u8> {
    let data_len = u32::try_from(pcm.len()).unwrap_or(u32::MAX);
    let mut wav = Vec::with_capacity(44 + pcm.len());

    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36u32.saturating_add(data_len)).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // PCM fmt chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&sample_rate.saturating_mul(2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(pcm);

    wav
}

#[command]
pub(crate) async fn is_supertonic_ready<R: Runtime>(
    app: AppHandle<R>,
) -> Result<IsSupertonicReadyResponse> {
    let app_data_dir = app.path().app_local_data_dir().map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string())))?;
    let req = IsSupertonicReadyRequest {
        app_data_dir: app_data_dir.to_string_lossy().to_string(),
    };
    app.supertonic().is_supertonic_ready(req)
}

#[command]
pub(crate) async fn download_supertonic_models<R: Runtime>(
    app: AppHandle<R>,
    mut payload: DownloadSupertonicRequest,
) -> Result<DownloadSupertonicResponse> {
    let app_data_dir = app.path().app_local_data_dir().map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string())))?;
    payload.model_path = app_data_dir.to_string_lossy().to_string();
    app.supertonic().download_supertonic_models(payload)
}

#[command]
pub async fn generate_supertonic_tts<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, SupertonicState>,
    payload: GenerateTtsRequest,
) -> Result<GenerateTtsResponse> {
    let tts_arc = state.tts.clone();
    let app_data_dir = app.path().app_local_data_dir().map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string())))?;
    
    tokio::task::spawn_blocking(move || {
        let mut tts_guard = tts_arc.lock().unwrap();

        let model_dir = app_data_dir.join("onnx");
        let style_path = app_data_dir.join(&payload.voice_style);

        if tts_guard.is_none() {
            let _ = ort::init().with_name("supertonic-tts").commit();

            let model_dir_str = model_dir.to_string_lossy().to_string();
            
            let tts = load_text_to_speech(&model_dir_str, true, true, 4, 1)
                .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
                
            *tts_guard = Some(tts);
        }

        let tts = tts_guard.as_mut().unwrap();
        let style = load_voice_style(&[style_path.to_string_lossy().to_string()], false)
            .map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

        let final_lang = crate::helper::normalize_lang_code(&payload.lang);
        if !crate::helper::is_valid_lang(&final_lang) {
            return Err(crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported language by Supertonic: {}", payload.lang)
            )));
        }

        let (wav_data, _dur) = tts.call(
            &payload.text,
            &final_lang,
            &style,
            payload.steps as usize,
            payload.speed,
            0.1,
            |_curr, _total, _chunk| true,
        ).map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

        let mut pcm_data = Vec::with_capacity(wav_data.len() * 2);
        for &sample in &wav_data {
            let clamped = sample.max(-1.0).min(1.0);
            let val = (clamped * 32767.0) as i16;
            pcm_data.extend_from_slice(&val.to_le_bytes());
        }

        let sample_rate = tts.sample_rate as u32;
        let audio_bytes = pcm16_mono_to_wav(&pcm_data, sample_rate);

        Ok(GenerateTtsResponse {
            audio_bytes,
            sample_rate,
        })
    }).await.unwrap_or_else(|e| Err(crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))))
}

#[command]
pub async fn purge_supertonic_models<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    let app_data_dir = app.path().app_local_data_dir().map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string())))?;
    let req = IsSupertonicReadyRequest {
        app_data_dir: app_data_dir.to_string_lossy().to_string(),
    };
    app.supertonic().purge_supertonic_models(req)
}

#[cfg(test)]
mod tests {
    use super::pcm16_mono_to_wav;

    #[test]
    fn wraps_pcm_in_valid_wav_header() {
        let pcm = [0u8, 0, 0xff, 0x7f];
        let wav = pcm16_mono_to_wav(&pcm, 22_050);

        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[36..40], b"data");
        assert_eq!(u32::from_le_bytes(wav[24..28].try_into().unwrap()), 22_050);
        assert_eq!(u32::from_le_bytes(wav[40..44].try_into().unwrap()), 4);
        assert_eq!(&wav[44..], &pcm);
    }
}
