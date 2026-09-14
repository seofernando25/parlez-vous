pub fn build_coding_puzzle_prompt(language: &str, theme: &str, puzzle_type: &str, previously_used: &[String]) -> (String, String) {
    let example_keystone_code = match language.to_lowercase().as_str() {
        "rust" => "fn do_work(a: i32, b: i32) -> i32 {\\n    return ___BLANK___;\\n}",
        "javascript" | "typescript" | "js" | "ts" => "function doWork(a, b) {\\n    return ___BLANK___;\\n}",
        "go" => "func doWork(a int, b int) int {\\n    return ___BLANK___\\n}",
        "c++" | "cpp" | "c" => "int do_work(int a, int b) {\\n    return ___BLANK___;\\n}",
        "java" => "public int doWork(int a, int b) {\\n    return ___BLANK___;\\n}",
        "kotlin" | "kt" => "fun doWork(a: Int, b: Int): Int {\\n    return ___BLANK___\\n}",
        _ => "def do_work(a, b):\\n    return ___BLANK___", // Default to python-like
    };

    let example_speedrun_code = match language.to_lowercase().as_str() {
        "rust" => "fn do_work(arr: &[i32]) -> i32 {\\n    return arr[arr.len() - 1];\\n}",
        "javascript" | "typescript" | "js" | "ts" => "function doWork(arr) {\\n    return arr[arr.length - 1];\\n}",
        "go" => "func doWork(arr []int) int {\\n    return arr[len(arr)-1]\\n}",
        "c++" | "cpp" | "c" => "int do_work(std::vector<int>& arr) {\\n    return arr.back();\\n}",
        "java" => "public int doWork(int[] arr) {\\n    return arr[arr.length - 1];\\n}",
        "kotlin" | "kt" => "fun doWork(arr: IntArray): Int {\\n    return arr.last()\\n}",
        _ => "def do_work(arr):\\n    return arr[-1]", // Default to python-like
    };

    let result = if puzzle_type == "keystone" {
        (
            format!(
                "You are a strict JSON code generator for a programming game.\nYou will be given a programming topic. Generate a concise, functional snippet of code (10-15 lines max) that solves it.\n\nCRITICAL GAMEPLAY RULES:\n1. You MUST replace the single most important line of algorithmic logic (the 'keystone' of the function) with the exact string \"___BLANK___\".\n2. The code MUST contain exactly one instance of \"___BLANK___\".\n3. Do not blank out function definitions, imports, or basic brackets. Only blank out the core logical step.\n4. CRITICAL: ALL generated code MUST be written strictly in the {language} programming language. Do NOT use any other language.\n\nCRITICAL JSON RULES:\n1. You MUST escape all internal double quotes inside your code strings (e.g., use \\\" instead of \").\n2. You MUST use \\n for newlines inside your code strings. Do NOT use actual line breaks.\n\nOutput strictly in this JSON format:\n{{\n  \"language\": \"{language}\",\n  \"code_with_blank\": \"{example_keystone_code}\",\n  \"exact_answer\": \"a + b\"\n}}"
            ),
            theme.to_string()
        )
    } else {
        (
            format!(
                "You are a strict JSON code generator for a programming game.\nYou will be given a programming topic. Generate a concise, functional snippet of code (10-15 lines max) that solves it.\n\nCRITICAL GAMEPLAY RULES:\n1. You MUST name the main function generically, exactly `doWork` or `do_work`, so the name does NOT give away what the code does.\n2. Do NOT include any comments, docstrings, or documentation in the code. It must be completely uncommented.\n3. Provide a short, 1-sentence correct description of what the code does, and 3 plausible but incorrect distractor descriptions. The distractors should sound like real programming operations.\n4. CRITICAL: ALL generated code MUST be written strictly in the {language} programming language. Do NOT use any other language.\n\nCRITICAL JSON RULES:\n1. You MUST escape all internal double quotes inside your code strings (e.g., use \\\" instead of \").\n2. You MUST use \\n for newlines inside your code strings. Do NOT use actual line breaks.\n\nOutput strictly in this JSON format:\n{{\n  \"language\": \"{language}\",\n  \"code\": \"{example_speedrun_code}\",\n  \"correct_answer\": \"Returns the last element of an array.\",\n  \"distractors\": [\"Removes the first element of an array.\", \"Reverses the array in place.\", \"Returns the length of the array.\"]\n}}"
            ),
            theme.to_string()
        )
    };

    if !previously_used.is_empty() {
        let mut appended_prompt = result.0;
        appended_prompt.push_str("\n\nFORBIDDEN PREVIOUS GAMES: The user has recently played the following games. CRITICAL INSTRUCTION: You MUST NOT generate a puzzle that matches these recent puzzles. Create something completely NEW and DIFFERENT:\n");
        for game in previously_used {
            appended_prompt.push_str(&format!("- {}\n", game));
        }
        (appended_prompt, result.1)
    } else {
        result
    }
}

pub fn build_language_puzzle_prompt(language: &str, skill_level: &str, active_theme: &str, active_subtheme: &str, puzzle_type: &str, previously_used: &[String]) -> String {

    let example_keystone = match language.to_lowercase().as_str() {
        "french" => "Je ___BLANK___ un croissant.",
        "spanish" => "Yo ___BLANK___ una manzana.",
        _ => "I ___BLANK___ a book.",
    };

    let example_speedrun = match language.to_lowercase().as_str() {
        "french" => "Je mange un croissant.",
        "spanish" => "Yo como una manzana.",
        _ => "I am reading a book.",
    };

    let prompt = if puzzle_type == "keystone" {
        format!(
            "You are a strict JSON generator for a language learning game. \
            Generate a single sentence in {language} suitable for a {skill_level} learner. \
            The sentence should be related to the theme: '{active_theme}', specifically focusing on the context of: '{active_subtheme}'. \
            \nCRITICAL GAMEPLAY RULES:\n\
            1. You MUST replace a key word in the sentence (like a conjugated verb or an important noun) with the exact string \"___BLANK___\".\n\
            2. The sentence MUST contain exactly one instance of \"___BLANK___\".\n\
            3. CRITICAL: ALL generated text MUST be grammatically correct {language}, except for the blank.\n\
            \nOutput strictly in this JSON format:\n\
            {{\n  \"language\": \"{language}\",\n  \"code_with_blank\": \"{example_keystone}\",\n  \"exact_answer\": \"mange\"\n}}"
        )
    } else {
        format!(
            "You are a strict JSON generator for a language learning game. \
            Generate a single sentence in {language} suitable for a {skill_level} learner. \
            The sentence should be related to the theme: '{active_theme}', specifically focusing on the context of: '{active_subtheme}'. \
            \nCRITICAL GAMEPLAY RULES:\n\
            1. Provide a correct English translation of the sentence, and 3 plausible but incorrect distractor English translations.\n\
            2. CRITICAL: ALL generated text MUST be grammatically correct {language}.\n\
            \nOutput strictly in this JSON format:\n\
            {{\n  \"language\": \"{language}\",\n  \"code\": \"{example_speedrun}\",\n  \"correct_answer\": \"I eat a croissant.\",\n  \"distractors\": [\"I want a croissant.\", \"I make a croissant.\", \"You eat a croissant.\"]\n}}"
        )
    };

    if !previously_used.is_empty() {
        let mut appended_prompt = prompt;
        appended_prompt.push_str("\n\nFORBIDDEN PREVIOUS GAMES: The user has recently played the following games. CRITICAL INSTRUCTION: You MUST NOT generate a puzzle that matches these recent puzzles. Create something completely NEW and DIFFERENT:\n");
        for game in previously_used {
            appended_prompt.push_str(&format!("- {}\n", game));
        }
        appended_prompt
    } else {
        prompt
    }
}
