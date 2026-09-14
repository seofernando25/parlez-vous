#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JournalVariables {
    pub mood: String,
    pub weather: String,
    pub activity: String,
    pub model: String,
    pub language: String,
    pub skill_level: String,
    pub active_theme: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GradingVariables {
    pub entry: String,
    pub model: String,
    pub language: String,
    pub skill_level: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct JournalResponse {
    pub generated_target_text: String,
    pub native_translation: String,
    pub vocabulary: Vec<VocabItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct VocabItem {
    pub target_text: String,
    pub native_text: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(default)]
    pub audio_base64: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, schemars::JsonSchema)]
pub struct ChatResponse {
    pub response: String,
    #[serde(default)]
    pub response_parts: Vec<String>,
    pub idealized_correction: Option<String>,
    #[serde(default)]
    pub context_summary: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct ConjugationExercise {
    pub subject: String,
    pub tense: String,
    pub sentence: String,
    pub verb: String,
    pub answer: String,
    pub translation: String,
}

/// Per-tense accuracy stats derived from the last N answered exercises.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct TenseStat {
    pub tense: String,
    pub total: u32,
    pub correct: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct KeystonePuzzle {
    pub language: String,
    pub code_with_blank: String,
    pub exact_answer: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct SpeedrunPuzzle {
    pub language: String,
    pub code: String,
    pub correct_answer: String,
    pub distractors: Vec<String>,
}
