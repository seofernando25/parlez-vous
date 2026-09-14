use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TurnAnalysis {
    pub intent: String,
    pub task: String,
    #[serde(default)]
    pub factual: bool,
    #[serde(default)]
    pub check_correction: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskSolution {
    pub answer: String,
    #[serde(default)]
    pub correction: Option<String>,
    #[serde(default = "default_confidence")]
    pub confidence: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TutorDraft {
    pub response_parts: Vec<String>,
    #[serde(default)]
    pub idealized_correction: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorrectionCheck {
    #[serde(default)]
    pub correction: Option<String>,
}

fn default_confidence() -> String { "medium".to_string() }

pub fn normalize_parts(parts: Vec<String>) -> Vec<String> {
    parts.into_iter()
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .take(5)
        .collect()
}
