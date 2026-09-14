use crate::ai::tts_adapter::TtsProvider;

#[tauri::command]
pub(crate) async fn generate_tts_audio<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    text: String,
    language: String,
    voice: String,
    speed: f32,
    url: String,
) -> Result<Vec<u8>, String> {
    // Strip angle brackets from translated words, keeping valid TTS tags intact.
    let re = regex::Regex::new(r"<([^>]+)>").map_err(|e| e.to_string())?;
    let cleaned_text = re.replace_all(&text, |caps: &regex::Captures| {
        let inner = &caps[1];
        if inner == "laugh" || inner == "breath" || inner == "cough" || inner == "surprise"
            || inner == "sad" || inner == "sigh" || inner.starts_with("lang:") || inner.starts_with("emotion:") {
            caps[0].to_string()
        } else {
            inner.to_string()
        }
    }).to_string();

    // 2. Strip parenthetical text (e.g., romanizations like "(annyeonghaseyo)" or translations like "(hello)")
    // so they are not spoken out loud by the TTS engine.
    let re_paren = regex::Regex::new(r"\s*\([^)]+\)").map_err(|e| e.to_string())?;
    let cleaned_text = re_paren.replace_all(&cleaned_text, "").to_string();

    let provider: Box<dyn TtsProvider> = if url == "supertonic" || url.starts_with("supertonic://") {
        Box::new(crate::ai::tts_adapter::SupertonicTtsAdapter::new(app_handle.clone()))
    } else {
        Box::new(crate::ai::tts_adapter::OpenAiTtsAdapter::new(url))
    };

    provider.generate_tts(&cleaned_text, &language, &voice, speed).await
}
