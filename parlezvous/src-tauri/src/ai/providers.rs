use async_trait::async_trait;
use super::types::*;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    async fn check_health(&self) -> bool;
    async fn list_models(&self) -> Result<Vec<String>, String>;
}

#[async_trait]
pub trait JournalProvider: Send + Sync {
    async fn generate_guided_journal(&self, variables: JournalVariables) -> Result<JournalResponse, String>;
    async fn grade_custom_journal(&self, variables: GradingVariables) -> Result<JournalResponse, String>;
}

#[async_trait]
pub trait ChatProvider: Send + Sync {
    async fn generate_chat_response(
        &self,
        history: Vec<ChatMessage>,
        model: String,
        language: String,
        skill_level: String,
        context: String,
        active_theme: Option<String>,
        active_subtheme: Option<String>,
        audio_base64: Option<String>,
        image_uri: Option<String>,
    ) -> Result<ChatResponse, String>;
}

#[async_trait]
pub trait ConjugationProvider: Send + Sync {
    async fn generate_conjugation_exercise(
        &self,
        language: String,
        model: String,
        previously_used: Vec<String>,
        tense_stats: Vec<TenseStat>,
        active_theme: String,
    ) -> Result<ConjugationExercise, String>;
}

#[async_trait]
pub trait PuzzleProvider: Send + Sync {
    async fn generate_coding_puzzle(
        &self,
        language: String,
        model: String,
        theme: String,
        puzzle_type: String,
        previously_used: Vec<String>,
    ) -> Result<String, String>;
    async fn generate_language_puzzle(
        &self,
        language: String,
        model: String,
        skill_level: String,
        active_theme: String,
        active_subtheme: String,
        puzzle_type: String,
        previously_used: Vec<String>,
    ) -> Result<String, String>;
}

pub trait LlmProvider:
    ModelProvider + JournalProvider + ChatProvider + ConjugationProvider + PuzzleProvider + Send + Sync
{}
impl<T> LlmProvider for T where
    T: ModelProvider + JournalProvider + ChatProvider + ConjugationProvider + PuzzleProvider + Send + Sync
{}

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn generate_embedding(&self, text: String, model: String) -> Result<Vec<f64>, String>;
}
