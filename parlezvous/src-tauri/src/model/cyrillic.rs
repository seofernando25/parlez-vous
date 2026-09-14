#[cfg(has_cyrillic_model)]
pub mod cyrillic_model {
    include!(concat!(env!("OUT_DIR"), "/model/cyrillic_model.rs"));
}

#[cfg(has_cyrillic_model)]
use burn::{backend::NdArray, prelude::*, tensor::Bytes};
#[cfg(has_cyrillic_model)]
type Backend = NdArray<f32>;

/// All 37 Cyrillic characters in class-index order (matching cyrillic_classes.json).
pub const ALL_CYRILLIC: [&str; 37] = [
    "а", "б", "в", "г", "д", "е", "ж", "з", "и", "й", "к", "л", "м", "н", "о", "п",
    "р", "с", "т", "у", "ф", "х", "ц", "ч", "ш", "щ", "ъ", "ы", "ь", "э", "ю", "я",
    "ё", "є", "і", "ї", "ґ",
];

/// Maps a class index (0–36) to the corresponding Cyrillic character.
pub fn get_cyrillic_from_index(index: usize) -> &'static str {
    match index {
        0 => "а",
        1 => "б",
        2 => "в",
        3 => "г",
        4 => "д",
        5 => "е",
        6 => "ж",
        7 => "з",
        8 => "и",
        9 => "й",
        10 => "к",
        11 => "л",
        12 => "м",
        13 => "н",
        14 => "о",
        15 => "п",
        16 => "р",
        17 => "с",
        18 => "т",
        19 => "у",
        20 => "ф",
        21 => "х",
        22 => "ц",
        23 => "ч",
        24 => "ш",
        25 => "щ",
        26 => "ъ",
        27 => "ы",
        28 => "ь",
        29 => "э",
        30 => "ю",
        31 => "я",
        32 => "ё",
        33 => "є",
        34 => "і",
        35 => "ї",
        36 => "ґ",
        _ => "?",
    }
}

/// Run inference on a 28×28 pixel array (0–255 u8 values, ink=255, bg=0).
/// Returns (predicted_character, confidence) tuple using the compiled Cyrillic Burn model.
#[cfg(has_cyrillic_model)]
pub fn infer_cyrillic(pixels: &[u8]) -> Result<(String, f32), String> {
    if pixels.len() != 784 {
        return Err(format!("Expected 784 pixels, got {}", pixels.len()));
    }

    let device = Default::default();
    let tensor_data: Vec<f32> = pixels.iter().map(|&p| p as f32 / 255.0).collect();

    let input = Tensor::<Backend, 1>::from_floats(tensor_data.as_slice(), &device)
        .reshape([1, 1, 28, 28]);

    static CYRILLIC_WEIGHTS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model/cyrillic_model.bpk"));

    let burn_bytes = Bytes::from_bytes_vec(CYRILLIC_WEIGHTS.to_vec());
    let model = cyrillic_model::Model::<Backend>::from_bytes(
        burn_bytes,
        &device
    );

    let output = model.forward(input);
    let probabilities = burn::tensor::activation::softmax(output, 1);

    let argmax = probabilities.clone().argmax(1);
    let pred_index = argmax.into_scalar() as usize;

    let probs_data = probabilities.to_data();
    let confidence = probs_data.iter::<f32>().nth(pred_index).unwrap_or(0.0);
    let letter = get_cyrillic_from_index(pred_index).to_string();
    Ok((letter, confidence))
}

#[cfg(not(has_cyrillic_model))]
pub fn infer_cyrillic(pixels: &[u8]) -> Result<(String, f32), String> {
    if pixels.len() != 784 {
        return Err(format!("Expected 784 pixels, got {}", pixels.len()));
    }
    Err("Cyrillic handwriting model is not installed. Place cyrillic_model.onnx plus cyrillic_model.onnx.data in src-tauri/models/.".into())
}
