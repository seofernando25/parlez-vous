pub fn build_journal_prompt(
    language: &str,
    skill_level: &str,
    mood: &str,
    weather: &str,
    activity: &str,
    active_theme: &Option<String>,
) -> String {
    let mut prompt = format!(
        "Generate a journal entry in {lang} for a {skill_level} learner based on these inputs: \
        Mood: {mood}, Weather: {weather}, Activity: {activity}. \
        Adapt your vocabulary and grammar complexity to the student's {skill_level} level.\n",
        lang=language, skill_level=skill_level, mood=mood, weather=weather, activity=activity
    );

    if let Some(theme) = active_theme {
        prompt.push_str(&format!("The user is currently studying the curriculum theme: {}. Please gently incorporate ideas or vocabulary related to this theme into the journal scaffold.\n", theme));
    }

    prompt.push_str("Also extract 3-5 useful dictionary-form vocabulary words from the entry. \
        Output STRICTLY valid JSON with the following structure, nothing else: \
        {\"generated_target_text\": \"...\", \"native_translation\": \"...\", \"vocabulary\": [{\"target_text\": \"...\", \"native_text\": \"...\"}], \"feedback\": null}");

    prompt
}
