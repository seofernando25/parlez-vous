mod chat;
mod conjugation;
mod journal;
mod puzzles;

pub use chat::build_chat_system_prompt;
pub use conjugation::{build_conjugation_prompt, build_conjugation_verification_prompt};
pub use journal::build_journal_prompt;
pub use puzzles::{build_coding_puzzle_prompt, build_language_puzzle_prompt};
