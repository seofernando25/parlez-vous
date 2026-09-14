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
    let mut output = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for part in parts {
        for line in part.lines() {
            let value = line.trim();
            if value.is_empty() || is_generic_filler(value) { continue; }
            let key = comparison_key(value);
            if key.is_empty() || !seen.insert(key) { continue; }
            output.push(value.to_string());
            if output.len() == 5 { return output; }
        }
    }
    output
}

fn comparison_key(value: &str) -> String {
    value.chars()
        .filter(|character| character.is_alphanumeric() || character.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_generic_filler(value: &str) -> bool {
    if value.split_whitespace().count() > 7 { return false; }
    let key = comparison_key(value);
    [
        "good job", "great job", "well done", "great progress", "nice work",
        "keep it up", "excellent work", "youre doing great",
    ].iter().any(|phrase| key == *phrase || key.starts_with(&format!("{phrase} ")))
}

#[cfg(test)]
mod tests {
    use super::normalize_parts;

    #[test]
    fn chat_bubbles_drop_duplicates_and_generic_filler() {
        let parts = vec![
            "Good job!".to_string(),
            "Tu vas bien ?".to_string(),
            "tu vas bien".to_string(),
            "On continue ?\nUn seul pas à la fois.".to_string(),
        ];
        assert_eq!(
            normalize_parts(parts),
            vec!["Tu vas bien ?", "On continue ?", "Un seul pas à la fois."]
        );
    }

    #[test]
    fn chat_bubbles_never_exceed_five() {
        let parts = (1..=7).map(|value| value.to_string()).collect();
        assert_eq!(normalize_parts(parts).len(), 5);
    }
}
