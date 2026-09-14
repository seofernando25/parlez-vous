#[cfg(has_character_model)]
pub mod character_model {
    include!(concat!(env!("OUT_DIR"), "/model/character_model.rs"));
}

#[cfg(has_character_model)]
use burn::{backend::NdArray, prelude::*, tensor::Bytes};
#[cfg(has_character_model)]
type Backend = NdArray<f32>;

#[cfg(has_character_model)]
/// Maps a class index (0–29) to the corresponding Hangul jamo character.
pub fn get_jamo_from_index(index: usize) -> &'static str {
    match index {
        0 => "ㅏ",
        1 => "ㅐ",
        2 => "ㅂ",
        3 => "ㅃ",
        4 => "ㅊ",
        5 => "ㄷ",
        6 => "ㅔ",
        7 => "ㅓ",
        8 => "ㅡ",
        9 => "ㄱ",
        10 => "ㄲ",
        11 => "ㅎ",
        12 => "ㅣ",
        13 => "ㅈ",
        14 => "ㅋ",
        15 => "ㅁ",
        16 => "ㄴ",
        17 => "ㅇ",
        18 => "ㅗ",
        19 => "ㅍ",
        20 => "ㄹ",
        21 => "ㅅ",
        22 => "ㅆ",
        23 => "ㅌ",
        24 => "ㅜ",
        25 => "ㅑ",
        26 => "ㅒ",
        27 => "ㅖ",
        28 => "ㅛ",
        29 => "ㅠ",
        _ => "?",
    }
}

/// Maps a Hangul jamo character back to the romanized label.
pub fn get_romanization(jamo: &str) -> &'static str {
    match jamo {
        "ㅏ" => "a",
        "ㅐ" => "ae",
        "ㅂ" => "b",
        "ㅃ" => "bb",
        "ㅊ" => "ch",
        "ㄷ" => "d",
        "ㅔ" => "e",
        "ㅓ" => "eo",
        "ㅡ" => "eu",
        "ㄱ" => "g",
        "ㄲ" => "gg",
        "ㅎ" => "h",
        "ㅣ" => "i",
        "ㅈ" => "j",
        "ㅋ" => "k",
        "ㅁ" => "m",
        "ㄴ" => "n",
        "ㅇ" => "ng",
        "ㅗ" => "o",
        "ㅍ" => "p",
        "ㄹ" => "r",
        "ㅅ" => "s",
        "ㅆ" => "ss",
        "ㅌ" => "t",
        "ㅜ" => "u",
        "ㅑ" => "ya",
        "ㅒ" => "yae",
        "ㅖ" => "ye",
        "ㅛ" => "yo",
        "ㅠ" => "yu",
        _ => "?",
    }
}

/// All 30 jamo characters in class-index order, for the frontend to cycle through.
pub const ALL_JAMO: [&str; 30] = [
    "ㅏ", "ㅐ", "ㅂ", "ㅃ", "ㅊ", "ㄷ", "ㅔ", "ㅓ", "ㅡ", "ㄱ", "ㄲ", "ㅎ", "ㅣ", "ㅈ", "ㅋ", "ㅁ",
    "ㄴ", "ㅇ", "ㅗ", "ㅍ", "ㄹ", "ㅅ", "ㅆ", "ㅌ", "ㅜ", "ㅑ", "ㅒ", "ㅖ", "ㅛ", "ㅠ",
];

/// Run inference on a 28×28 pixel array (0–255 u8 values, ink=255, bg=0).
/// Returns (predicted_jamo, confidence) tuple.
#[cfg(has_character_model)]
pub fn infer(pixels: &[u8]) -> Result<(String, f32), String> {
   if pixels.len() != 784 {
        return Err(format!("Expected 784 pixels, got {}", pixels.len()));
    }

    let device = Default::default();
    let tensor_data: Vec<f32> = pixels.iter().map(|&p| p as f32 / 255.0).collect();

    let input = Tensor::<Backend, 1>::from_floats(tensor_data.as_slice(), &device)
        .reshape([1, 1, 28, 28]);

    static MODEL_WEIGHTS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model/character_model.bpk"));

    // 2. Convert raw slice -> Vec<u8> -> Burn Bytes
    let burn_bytes = Bytes::from_bytes_vec(MODEL_WEIGHTS.to_vec());

    // 3. Load safely from RAM
    let model = character_model::Model::<Backend>::from_bytes(
        burn_bytes,
        &device
    );

    let output = model.forward(input);
    let probabilities = burn::tensor::activation::softmax(output, 1);

    let argmax = probabilities.clone().argmax(1);
    let pred_index = argmax.into_scalar() as usize;

    let probs_data = probabilities.to_data();
    let confidence = probs_data.iter::<f32>().nth(pred_index).unwrap_or(0.0);
    let jamo = get_jamo_from_index(pred_index).to_string();
    Ok((jamo, confidence))
}

#[cfg(not(has_character_model))]
pub fn infer(pixels: &[u8]) -> Result<(String, f32), String> {
    if pixels.len() != 784 {
        return Err(format!("Expected 784 pixels, got {}", pixels.len()));
    }
    Err("Hangul handwriting model is not installed. Train it in ../hangulnist and copy character_model.onnx plus character_model.onnx.data into src-tauri/models/.".into())
}
