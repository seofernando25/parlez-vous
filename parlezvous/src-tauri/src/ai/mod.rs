pub mod openai_compat_adapter;
mod openai_compat;
#[cfg(target_os = "android")]
pub mod litert_adapter;
#[cfg(target_os = "android")]
mod litert;
pub mod router;
pub mod tts_adapter;
pub mod tutor;

mod context;
mod prompts;
mod providers;
mod types;

pub use context::{truncate_history_by_tokens, truncate_history_exact};
pub use prompts::*;
pub use providers::{ChatProvider, ConjugationProvider, EmbeddingProvider, JournalProvider, LlmProvider, ModelProvider, PuzzleProvider};
pub use types::*;
