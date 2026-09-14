mod prompts;
mod reference;
mod types;

pub use prompts::{analyze as analysis_prompt, conversation as conversation_prompt, correction as correction_prompt, solve as solver_prompt};
pub use reference::{false_friend_challenge, false_friend_feedback};
pub use types::{normalize_parts, CorrectionCheck, TaskSolution, TurnAnalysis, TutorDraft};

#[cfg(test)]
mod eval;
