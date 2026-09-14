pub fn build_chat_system_prompt(
    language: &str,
    skill_level: &str,
    context: &str,
    active_theme: &Option<String>,
    active_subtheme: &Option<String>,
    use_json: bool,
    use_expression_tags: bool,
    is_vision_judge: bool,
) -> String {
    let mut prompt = String::with_capacity(2048);

    // move to own prompt, not sure why here.
    if is_vision_judge {
        prompt.push_str(&format!(
            "# ROLE\n\
            You are a helpful language learning vision judge for {language}. \
            Your job is to describe the attached image and grade the user's description of it.\n\
            Point out anything they missed, and correct their grammar based on their {skill_level} level.\n\n",
            language=language, skill_level=skill_level
        ));
    } else {
        prompt.push_str(&format!(
            "Role: {} language partner. Keep conversation flowing naturally. Adapt to user's level. Ignore minor errors. End with a short follow-up question.\n\n\
            Rules:\n\
            - Max 3 sentences.\n\
            - No random greetings mid-chat.\n\
            - You control an avatar. Include exact animation tags: [anim:shrug], [anim:greet], [anim:peace], [anim:shoot], [anim:spin], [anim:pose], [anim:squat], [anim:full]. No other tags.\n",
            language
        ));

        if use_expression_tags {
            prompt.push_str(
                "- Expression tags (use exactly as shown):\n\
                  * <laugh> : END of sentence (x3)\n\
                  * <breath> : START of sentence (x3)\n\
                  * <sad> : BOTH ends of sentence (x3)\n"
            );
        }

        if !cfg!(target_os = "android") && !context.trim().is_empty() {
            prompt.push_str("Textbook Context (use to guide conversation):\n");
            prompt.push_str(context);
            prompt.push_str("\n\n");
        }

        if let Some(theme) = active_theme {
            prompt.push_str(&format!("Theme: {}. Steer chat towards this and use related words.\n", theme));
            if let Some(subtheme) = active_subtheme {
                prompt.push_str(&format!("Specific Context/Subtheme: {}.\n", subtheme));
            }
            prompt.push_str("\n");
        }
    }

    if skill_level.trim().eq_ignore_ascii_case("beginner") {
        prompt.push_str(&format!(
            "User level: Beginner. Act as strict tutor. Speak mostly English. Introduce 1-2 new {} words/phrases per turn with meaning. Use native script for {} words.\n",
            language, language
        ));
    } else {
        prompt.push_str(&format!(
            "User level: {}. Speak almost entirely in {}. Adapt vocabulary to their level.\n",
            skill_level, language
        ));
    }

    prompt.push_str(
        "No romanization/phonetics. Use perfect orthography/punctuation in native script. No parentheses.\n\n"
    );

    // 9. Output Formatting Constraint (Crucial this goes at the end)
    prompt.push_str("# OUTPUT FORMAT\n");
    if use_json {
        prompt.push_str(
            "Output STRICTLY valid JSON with the following structure, nothing else:\n\
            {\n  \
                \"response\": \"Your conversational response here (including your animation/expression tags)\",\n  \
                \"idealized_correction\": \"If the user's last message had grammar errors, provide the correction here. If perfect, return null.\"\n\
            }\n"
        );
    } else {
        prompt.push_str("Do not use JSON. Provide your response directly.\n");
    }

    prompt
}
