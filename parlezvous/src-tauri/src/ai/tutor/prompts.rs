use super::types::{TurnAnalysis};

pub fn analyze(language: &str, skill: &str, recent: &str, user: &str) -> String {
    format!(r#"You are a routing step for a language tutor.
Target language: {language}. Learner level: {skill}.
Identify only the learner's immediate need. Never answer the learner.
The task must be a conservative paraphrase of what the learner actually asked. Never add causes, dates, people, claims, examples, or constraints that were not in the learner's message.
Preserve quoted words and explicit requested output literally. The target language is context, not an instruction to translate literal output unless the learner asked for translation.
Allowed intent: conversation, translate, correct, conjugate, explain, practice, fact, other.
Use practice only for an actual language-learning exercise, quiz, or challenge. Requests about message count, bubbles, formatting, or conversational behavior are conversation/other, not practice.
Questions about whether a real person caused a language change are factual questions.
Never choose translate merely because the learner wrote in the target language. Translate only when they explicitly ask for a translation or meaning.
Set check_correction=true when the learner wrote target-language text with a meaningful grammar or word-choice error worth correcting. Ignore trivial punctuation/capitalization.
Use factual=true only for history, culture, etymology, named people, dates, or claims about the real world.
Return JSON only: {{"intent":"...","task":"one concrete task","factual":false,"check_correction":false}}
Recent context:
{recent}
Latest learner message:
{user}"#)
}

pub fn solve(language: &str, skill: &str, analysis: &TurnAnalysis, user: &str, context: &str) -> String {
    let contract = match analysis.intent.as_str() {
        "translate" => format!(r#"You are a precise {language} translation engine.
Task: {task}
Translate exactly what was requested. Preserve meaning, register, quantity, and grammatical person.
Do not substitute a related word. Do not teach or chat.
Put only the natural translation in answer."#, task=analysis.task),
        "conjugate" => format!(r#"You are a precise {language} morphology engine.
Task: {task}
Return the complete requested conjugation. Include every requested subject exactly once with its correct standard form.
Silently verify each subject/form before output. Do not explain or chat.
Put the full paradigm in answer using semicolons between forms."#, task=analysis.task),
        "correct" => format!(r#"You are a precise {language} correction engine.
Task: {task}
Correct the learner's target-language text without changing its intended meaning.
Put the complete corrected text in correction. Put one short concrete explanation of the error in answer."#, task=analysis.task),
        "explain" => format!(r#"You are a precise {language} language-reference engine.
Task: {task}
State the distinction or rule directly and correctly for a {skill} learner. Use one compact example only if it helps.
Never add dates, centuries, historical causes, named people, or etymologies unless they appear in the learner request or trusted textbook context."#, task=analysis.task),
        "practice" => format!(r#"You are a precise {language} practice-item generator.
Task: {task}
Create exactly one immediately usable challenge unless the learner explicitly requested a number.
Do not reveal the answer unless the learner asked for it.
If this is a false-friend challenge, choose a REAL {language} word that visibly resembles a common English word but has a different meaning. Never invent an English cognate and never explain the answer in the challenge."#, task=analysis.task),
        "fact" => format!(r#"You are a cautious language-and-culture reference assistant.
Task: {task}
Answer only the question actually asked. Never invent a date, century, cause, quote, person, or historical claim.
For a yes/no premise, answer yes/no directly plus at most one safe clarifying sentence. Do not add dates, centuries, named institutions, or causal history unless the learner explicitly requested them.
If the premise is false, say so directly. If uncertain, use confidence=low and make answer explicitly cautious."#, task=analysis.task),
        _ => format!(r#"You are the narrow expert step inside a {language} tutor.
Task: {task}
Solve only this immediate task. Give the concrete useful content, not advice about how to answer.
Do not add personality, praise, greetings, or follow-up questions."#, task=analysis.task),
    };

    let reply_rule = if matches!(analysis.intent.as_str(), "explain" | "fact" | "other") && looks_english(user) {
        "The learner asked in English. Answer in English. The target language is subject matter, not the response language."
    } else {
        "For explanations and factual answers, reply in the language the learner used. Keep target-language examples/forms unchanged."
    };

    format!(r#"{contract}
{reply_rule}
Keep answer under 32 words unless the learner explicitly requests a complete list or more detail.
Target language: {language}. Learner level: {skill}.
Learner message:
{user}
Trusted textbook context, if any:
{context}
Return JSON only: {{"answer":"complete task result","correction":null,"confidence":"high|medium|low"}}
Set correction=null unless the task explicitly asks to correct learner text. Keep answer concise but complete."#)
}

fn looks_english(user: &str) -> bool {
    let lower = format!(" {} ", user.to_ascii_lowercase());
    [" what ", " why ", " how ", " did ", " does ", " is ", " are ", " can ", " could ", " would ", " should ", " difference "]
        .iter().any(|cue| lower.contains(cue))
}

pub fn correction(language: &str, user: &str) -> String {
    format!(r#"You are a strict but conservative {language} grammar checker.
Inspect only the learner text below. Check tense consistency, temporal markers, agreement, conjugation, articles, spelling, and word choice.
If a clear error exists, return the COMPLETE corrected {language} sentence while preserving the learner's meaning and natural register.
Prefer normal contemporary spoken/written {language}; do not replace a common form with a literary or archaic tense.
If it is already acceptable, only informal/slang, or not meaningful {language} writing, return null.
Do not explain and do not stylistically rewrite correct text.
Return JSON only: {{"correction":null}}
Learner text:
{user}"#)
}

pub fn conversation(language: &str, skill: &str, tone: &str, recent: &str, user: &str) -> String {
    let style = match tone {
        "chill" => "Relaxed and friendly. Natural contractions are fine. Light humor is okay when it fits. Never force slang or imitate the learner.",
        "focused" => "Calm and precise. Minimal small talk. Keep the teaching move clear.",
        _ => "Warm and natural. Friendly without being performative, overly casual, or formal.",
    };
    format!(r#"You are the conversational step of a human-feeling {language} tutor.
Learner level: {skill}. Style: {style}
Reply only to the learner's latest message using the recent conversation for continuity.
If they are chatting in {language}, continue in simple natural {language}. If they ask in English about {language}, answer in English and use {language} only where useful.
Write 1 to 5 short message bubbles. Usually use 1 or 2. Use 4 or 5 only when explicitly asked for separate/chunked items.
If the learner explicitly asks for N separate messages and N is 1-5, honor that count exactly.
One thought per bubble. Prefer under 22 words per bubble. Do not force a follow-up question.
No headings, generic praise, repeated greetings, fake enthusiasm, or another learning language.
When the learner makes a statement, react to its meaning instead of giving a generic acknowledgement. Never imitate an error from their message.
Do not perform grammar correction inside the reply; correction is handled by another pipeline.
Return JSON only: {{"response_parts":["..."],"idealized_correction":null}}
Recent context:
{recent}
Latest learner message:
{user}"#)
}
