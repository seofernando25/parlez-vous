use crate::ai::{ConjugationExercise, TenseStat};

pub fn build_conjugation_prompt(
    language: &str,
    previously_used: &[String],
    _tense_stats: &[TenseStat],
    active_theme: &str,
) -> String {
    let lenses = [
        "an emotional perspective", "a historical or time-based context", "a technical or analytical angle",
        "a visual or descriptive focus", "a social or relational viewpoint", "an action-oriented or energetic tone",
        "a mysterious or curious angle", "a humorous or lighthearted perspective", "a formal or professional tone",
        "a mundane or everyday slice-of-life angle"
    ];
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let lens = lenses[(time as usize) % lenses.len()];

    let mut prompt = format!(
        "Generate a conjugation exercise for a student learning {lang}. \
        First, pick a completely random, natural, and unique infinitive verb. \
        Pick a specific subject, and construct a natural sentence around it. \
        CRITICAL: The sentence MUST strictly relate to the theme of '{theme}', specifically focusing on '{lens}'. \
        CRITICAL: You MUST conjugate your chosen verb DIRECTLY. DO NOT use modal or auxiliary verbs (like pouvoir, vouloir, devoir, aller) followed by the infinitive of your chosen verb. Your chosen verb must be the primary conjugated action in the sentence. \
        CRITICAL: In languages like French, object pronouns (like 'vous', 'te', 'me') often sit right before the verb (e.g. 'Je vous accueille'). Do NOT mistake the object for the subject. The 'subject' field MUST accurately reflect the true entity performing the action, and the verb MUST be conjugated to match that subject. \
        Output STRICTLY valid JSON with the following structure, nothing else, IN THIS EXACT ORDER: \n\
        {{ \n  \
            \"verb\": \"Pick a random infinitive verb (e.g. manger)\", \n  \
            \"subject\": \"Pick a random subject pronoun or noun (the true doer of the action)\", \n  \
            \"sentence\": \"Write a complete, natural sentence in {lang} using your chosen verb and subject directly conjugated.\", \n  \
            \"tense\": \"Identify the exact tense of the primary conjugated verb you used in the sentence (e.g. passé composé)\", \n  \
            \"answer\": \"Extract just the exact conjugated verb from the sentence.\", \n  \
            \"translation\": \"english translation of the infinitive verb\" \n\
        }}",
        lang=language,
        theme=active_theme,
        lens=lens
    );


    if !previously_used.is_empty() {
        prompt.push_str("\n\nFORBIDDEN VERBS: The student has recently practiced the following verbs. \
            CRITICAL INSTRUCTION: You MUST select a completely NEW and DIFFERENT VERB. Under NO CIRCUMSTANCES may your sentence use any of these verbs as the primary conjugated verb. If you do, the system will crash:\n");
        for entry in previously_used {
            prompt.push_str(&format!("- {}\n", entry));
        }
    }

    prompt
}

pub fn build_conjugation_verification_prompt(
    language: &str,
    exercise: &ConjugationExercise,
) -> String {
    format!(
        "Please verify the following conjugation exercise for a student learning {lang}. \n\
        Subject: {subject} \n\
        Tense: {tense} \n\
        Sentence: {sentence} \n\
        Verb (infinitive): {verb} \n\
        Current Answer: {answer} \n\
        Translation of Infinitive: {translation} \n\n\
        If the current answer and sentence are completely correct, output the same JSON exactly. \
        If there is any error in the conjugation (e.g. wrong tense, wrong subject agreement, misspelled), correct the 'answer', 'subject', and/or 'sentence' field and output the fixed JSON. \
        CRITICAL: The 'answer' MUST be the conjugated form of the exact verb '{verb}'. Do NOT change the answer to a completely different verb.\n\
        CRITICAL BUG FIX RULE 1: If the sentence uses a modal or auxiliary verb (like pouvoir, vouloir, devoir, aller) followed by the infinitive '{verb}', THIS IS WRONG. You MUST completely rewrite the sentence so that '{verb}' itself is directly conjugated as the primary verb of the sentence, and then update the 'answer' to match it.\n\
        CRITICAL BUG FIX RULE 2: Object pronouns (e.g. 'vous', 'te', 'me') placed before the verb do NOT determine conjugation. The verb MUST match the true subject. If the 'subject' field does not match the actual grammatical subject of the sentence, correct the 'subject' field and ensure the 'answer' matches the true subject.\n\
        Output STRICTLY valid JSON with the following structure, nothing else, IN THIS EXACT ORDER: \n\
        {{\n  \"verb\": \"{verb}\",\n  \"subject\": \"{subject}\",\n  \"sentence\": \"{sentence}\",\n  \"tense\": \"{tense}\",\n  \"answer\": \"corrected or original conjugated form\",\n  \"translation\": \"{translation}\"\n}}",
        lang=language, subject=exercise.subject, tense=exercise.tense, sentence=exercise.sentence, verb=exercise.verb, answer=exercise.answer, translation=exercise.translation
    )
}
