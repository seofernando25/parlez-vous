use crate::ai::ChatMessage;
use tokenizers::Tokenizer;

pub fn truncate_history_by_tokens(
    system_prompt: &str,
    history: &[ChatMessage],
    max_tokens_budget: usize,
) -> Vec<ChatMessage> {
    // FIX 1: Use .chars().count() to prevent multi-byte characters (like Hangul/Kanji)
    // from artificially inflating the token estimate.
    let estimate_tokens = |text: &str| -> usize {
        (text.chars().count() as f64 / 3.5).ceil() as usize
    };

    let sys_tokens = estimate_tokens(system_prompt);

    // FIX 2: Bump the reserved output budget to 250
    let safe_budget = if max_tokens_budget > sys_tokens + 250 {
        max_tokens_budget - sys_tokens - 250
    } else {
        0
    };

    let mut current_budget = 0;
    let mut keep_count = 0;

    // Traverse from newest to oldest
    for (i, msg) in history.iter().rev().enumerate() {
        let mut msg_tokens = estimate_tokens(&msg.content);
        if msg.content.contains("🎤 [Audio Message]") {
            msg_tokens += 1200; // Native audio footprint penalty
        }
        let total_msg_tokens = msg_tokens + 5;

        // FIX 3: Guarantee we ALWAYS keep the very last message (i == 0)
        if i > 0 && current_budget + total_msg_tokens > safe_budget {
            break;
        }

        current_budget += total_msg_tokens;
        keep_count += 1;
    }

    let start_idx = history.len().saturating_sub(keep_count);
    history[start_idx..].to_vec()
}

pub fn truncate_history_exact(
    system_prompt: &str,
    history: &[ChatMessage],
    max_tokens_budget: usize,
    tokenizer: &Tokenizer,
) -> Vec<ChatMessage> {
    let sys_tokens = tokenizer.encode(system_prompt, true).unwrap_or_default().get_ids().len();

    let safe_budget = max_tokens_budget.saturating_sub(sys_tokens + 250);

    let mut current_budget = 0;
    let mut keep_count = 0;

    for (i, msg) in history.iter().rev().enumerate() {
        let mut msg_tokens = tokenizer.encode(msg.content.clone(), true).unwrap_or_default().get_ids().len();
        if msg.content.contains("🎤 [Audio Message]") {
            msg_tokens += 1200; // Native audio footprint penalty
        }
        let total_msg_tokens = msg_tokens + 5; // chat template overhead

        if i > 0 && current_budget + total_msg_tokens > safe_budget {
            break;
        }

        current_budget += total_msg_tokens;
        keep_count += 1;
    }

    let start_idx = history.len().saturating_sub(keep_count);
    history[start_idx..].to_vec()
}
