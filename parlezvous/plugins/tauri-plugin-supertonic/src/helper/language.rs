pub const AVAILABLE_LANGS: &[&str] = &["en", "ko", "ja", "ar", "bg", "cs", "da", "de", "el", "es", "et", "fi", "fr", "hi", "hr", "hu", "id", "it", "lt", "lv", "nl", "pl", "pt", "ro", "ru", "sk", "sl", "sv", "tr", "uk", "vi", "na"];

pub fn is_valid_lang(lang: &str) -> bool {
    AVAILABLE_LANGS.contains(&lang)
}

pub fn normalize_lang_code(lang: &str) -> String {
    let lower = lang.trim().to_lowercase();
    match lower.as_str() {
        "english" => "en",
        "korean" => "ko",
        "japanese" => "ja",
        "arabic" => "ar",
        "bulgarian" => "bg",
        "czech" => "cs",
        "danish" => "da",
        "german" => "de",
        "greek" => "el",
        "spanish" => "es",
        "estonian" => "et",
        "finnish" => "fi",
        "french" => "fr",
        "hindi" => "hi",
        "croatian" => "hr",
        "hungarian" => "hu",
        "indonesian" => "id",
        "italian" => "it",
        "lithuanian" => "lt",
        "latvian" => "lv",
        "dutch" => "nl",
        "polish" => "pl",
        "portuguese" => "pt",
        "romanian" => "ro",
        "russian" => "ru",
        "slovak" => "sk",
        "slovenian" => "sl",
        "swedish" => "sv",
        "turkish" => "tr",
        "ukrainian" => "uk",
        "vietnamese" => "vi",
        _ => lower.as_str(),
    }.to_string()
}
