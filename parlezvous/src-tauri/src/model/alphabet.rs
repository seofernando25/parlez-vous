use super::hangul::{get_romanization, ALL_JAMO};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AlphabetLetter {
    pub char: String,
    pub uppercase: String,
    pub lowercase: String,
    pub name: String,
    pub romanization: String,
    pub pronunciation: String,
    pub script: String,
    pub is_vowel: bool,
}

pub fn get_alphabet_letters(script: &str) -> Vec<AlphabetLetter> {
    match script.to_lowercase().as_str() {
        "russian" | "cyrillic" => vec![
            AlphabetLetter { char: "А".into(), uppercase: "А".into(), lowercase: "а".into(), name: "а".into(), romanization: "a".into(), pronunciation: "[a]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Б".into(), uppercase: "Б".into(), lowercase: "б".into(), name: "бэ".into(), romanization: "b".into(), pronunciation: "[b]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "В".into(), uppercase: "В".into(), lowercase: "в".into(), name: "вэ".into(), romanization: "v".into(), pronunciation: "[v]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Г".into(), uppercase: "Г".into(), lowercase: "г".into(), name: "гэ".into(), romanization: "g".into(), pronunciation: "[ɡ]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Д".into(), uppercase: "Д".into(), lowercase: "д".into(), name: "дэ".into(), romanization: "d".into(), pronunciation: "[d]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Е".into(), uppercase: "Е".into(), lowercase: "е".into(), name: "е".into(), romanization: "ye".into(), pronunciation: "[je]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Ё".into(), uppercase: "Ё".into(), lowercase: "ё".into(), name: "ё".into(), romanization: "yo".into(), pronunciation: "[jo]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Ж".into(), uppercase: "Ж".into(), lowercase: "ж".into(), name: "жэ".into(), romanization: "zh".into(), pronunciation: "[ʐ]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "З".into(), uppercase: "З".into(), lowercase: "з".into(), name: "зэ".into(), romanization: "z".into(), pronunciation: "[z]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "И".into(), uppercase: "И".into(), lowercase: "и".into(), name: "и".into(), romanization: "i".into(), pronunciation: "[i]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Й".into(), uppercase: "Й".into(), lowercase: "й".into(), name: "и краткое".into(), romanization: "y".into(), pronunciation: "[j]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "К".into(), uppercase: "К".into(), lowercase: "к".into(), name: "ка".into(), romanization: "k".into(), pronunciation: "[k]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Л".into(), uppercase: "Л".into(), lowercase: "л".into(), name: "эль".into(), romanization: "l".into(), pronunciation: "[l]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "М".into(), uppercase: "М".into(), lowercase: "м".into(), name: "эм".into(), romanization: "m".into(), pronunciation: "[m]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Н".into(), uppercase: "Н".into(), lowercase: "н".into(), name: "эн".into(), romanization: "n".into(), pronunciation: "[n]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "О".into(), uppercase: "О".into(), lowercase: "о".into(), name: "о".into(), romanization: "o".into(), pronunciation: "[o]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "П".into(), uppercase: "П".into(), lowercase: "п".into(), name: "пэ".into(), romanization: "p".into(), pronunciation: "[p]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Р".into(), uppercase: "Р".into(), lowercase: "р".into(), name: "эр".into(), romanization: "r".into(), pronunciation: "[r]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "С".into(), uppercase: "С".into(), lowercase: "с".into(), name: "эс".into(), romanization: "s".into(), pronunciation: "[s]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Т".into(), uppercase: "Т".into(), lowercase: "т".into(), name: "тэ".into(), romanization: "t".into(), pronunciation: "[t]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "У".into(), uppercase: "У".into(), lowercase: "у".into(), name: "у".into(), romanization: "u".into(), pronunciation: "[u]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Ф".into(), uppercase: "Ф".into(), lowercase: "ф".into(), name: "эф".into(), romanization: "f".into(), pronunciation: "[f]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Х".into(), uppercase: "Х".into(), lowercase: "х".into(), name: "ха".into(), romanization: "kh".into(), pronunciation: "[x]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Ц".into(), uppercase: "Ц".into(), lowercase: "ц".into(), name: "цэ".into(), romanization: "ts".into(), pronunciation: "[ts]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Ч".into(), uppercase: "Ч".into(), lowercase: "ч".into(), name: "че".into(), romanization: "ch".into(), pronunciation: "[tɕ]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Ш".into(), uppercase: "Ш".into(), lowercase: "ш".into(), name: "ша".into(), romanization: "sh".into(), pronunciation: "[ʂ]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Щ".into(), uppercase: "Щ".into(), lowercase: "щ".into(), name: "ща".into(), romanization: "shch".into(), pronunciation: "[ɕː]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Ъ".into(), uppercase: "Ъ".into(), lowercase: "ъ".into(), name: "твёрдый знак".into(), romanization: "ʺ".into(), pronunciation: "[silent]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Ы".into(), uppercase: "Ы".into(), lowercase: "ы".into(), name: "ы".into(), romanization: "y".into(), pronunciation: "[ɨ]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Ь".into(), uppercase: "Ь".into(), lowercase: "ь".into(), name: "мягкий знак".into(), romanization: "ʹ".into(), pronunciation: "[palatal]".into(), script: "cyrillic".into(), is_vowel: false },
            AlphabetLetter { char: "Э".into(), uppercase: "Э".into(), lowercase: "э".into(), name: "э".into(), romanization: "e".into(), pronunciation: "[ɛ]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Ю".into(), uppercase: "Ю".into(), lowercase: "ю".into(), name: "ю".into(), romanization: "yu".into(), pronunciation: "[ju]".into(), script: "cyrillic".into(), is_vowel: true },
            AlphabetLetter { char: "Я".into(), uppercase: "Я".into(), lowercase: "я".into(), name: "я".into(), romanization: "ya".into(), pronunciation: "[ja]".into(), script: "cyrillic".into(), is_vowel: true },
        ],
        "ukrainian" => vec![
            AlphabetLetter { char: "А".into(), uppercase: "А".into(), lowercase: "а".into(), name: "а".into(), romanization: "a".into(), pronunciation: "[a]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Б".into(), uppercase: "Б".into(), lowercase: "б".into(), name: "бе".into(), romanization: "b".into(), pronunciation: "[b]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "В".into(), uppercase: "В".into(), lowercase: "в".into(), name: "ве".into(), romanization: "v".into(), pronunciation: "[w/v]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Г".into(), uppercase: "Г".into(), lowercase: "г".into(), name: "ге".into(), romanization: "h".into(), pronunciation: "[ɦ]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Ґ".into(), uppercase: "Ґ".into(), lowercase: "ґ".into(), name: "ґе".into(), romanization: "g".into(), pronunciation: "[ɡ]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Д".into(), uppercase: "Д".into(), lowercase: "д".into(), name: "де".into(), romanization: "d".into(), pronunciation: "[d]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Е".into(), uppercase: "Е".into(), lowercase: "е".into(), name: "е".into(), romanization: "e".into(), pronunciation: "[ɛ]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Є".into(), uppercase: "Є".into(), lowercase: "є".into(), name: "є".into(), romanization: "ye".into(), pronunciation: "[je]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Ж".into(), uppercase: "Ж".into(), lowercase: "ж".into(), name: "же".into(), romanization: "zh".into(), pronunciation: "[ʒ]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "З".into(), uppercase: "З".into(), lowercase: "з".into(), name: "зе".into(), romanization: "z".into(), pronunciation: "[z]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "И".into(), uppercase: "И".into(), lowercase: "и".into(), name: "и".into(), romanization: "y".into(), pronunciation: "[ɪ]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "І".into(), uppercase: "І".into(), lowercase: "і".into(), name: "і".into(), romanization: "i".into(), pronunciation: "[i]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Ї".into(), uppercase: "Ї".into(), lowercase: "ї".into(), name: "ї".into(), romanization: "yi".into(), pronunciation: "[ji]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Й".into(), uppercase: "Й".into(), lowercase: "й".into(), name: "йот".into(), romanization: "y".into(), pronunciation: "[j]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "К".into(), uppercase: "К".into(), lowercase: "к".into(), name: "ка".into(), romanization: "k".into(), pronunciation: "[k]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Л".into(), uppercase: "Л".into(), lowercase: "л".into(), name: "ел".into(), romanization: "l".into(), pronunciation: "[l]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "М".into(), uppercase: "М".into(), lowercase: "м".into(), name: "ем".into(), romanization: "m".into(), pronunciation: "[m]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Н".into(), uppercase: "Н".into(), lowercase: "н".into(), name: "ен".into(), romanization: "n".into(), pronunciation: "[n]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "О".into(), uppercase: "О".into(), lowercase: "о".into(), name: "о".into(), romanization: "o".into(), pronunciation: "[ɔ]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "П".into(), uppercase: "П".into(), lowercase: "п".into(), name: "пе".into(), romanization: "p".into(), pronunciation: "[p]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Р".into(), uppercase: "Р".into(), lowercase: "р".into(), name: "ер".into(), romanization: "r".into(), pronunciation: "[r]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "С".into(), uppercase: "С".into(), lowercase: "с".into(), name: "ес".into(), romanization: "s".into(), pronunciation: "[s]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Т".into(), uppercase: "Т".into(), lowercase: "т".into(), name: "те".into(), romanization: "t".into(), pronunciation: "[t]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "У".into(), uppercase: "У".into(), lowercase: "у".into(), name: "у".into(), romanization: "u".into(), pronunciation: "[u]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Ф".into(), uppercase: "Ф".into(), lowercase: "ф".into(), name: "еф".into(), romanization: "f".into(), pronunciation: "[f]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Х".into(), uppercase: "Х".into(), lowercase: "х".into(), name: "ха".into(), romanization: "kh".into(), pronunciation: "[x]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Ц".into(), uppercase: "Ц".into(), lowercase: "ц".into(), name: "це".into(), romanization: "ts".into(), pronunciation: "[ts]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Ч".into(), uppercase: "Ч".into(), lowercase: "ч".into(), name: "че".into(), romanization: "ch".into(), pronunciation: "[tʃ]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Ш".into(), uppercase: "Ш".into(), lowercase: "ш".into(), name: "ша".into(), romanization: "sh".into(), pronunciation: "[ʃ]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Щ".into(), uppercase: "Щ".into(), lowercase: "щ".into(), name: "ща".into(), romanization: "shch".into(), pronunciation: "[ʃtʃ]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Ь".into(), uppercase: "Ь".into(), lowercase: "ь".into(), name: "м'який знак".into(), romanization: "ʹ".into(), pronunciation: "[palatal]".into(), script: "ukrainian".into(), is_vowel: false },
            AlphabetLetter { char: "Ю".into(), uppercase: "Ю".into(), lowercase: "ю".into(), name: "ю".into(), romanization: "yu".into(), pronunciation: "[ju]".into(), script: "ukrainian".into(), is_vowel: true },
            AlphabetLetter { char: "Я".into(), uppercase: "Я".into(), lowercase: "я".into(), name: "я".into(), romanization: "ya".into(), pronunciation: "[ja]".into(), script: "ukrainian".into(), is_vowel: true },
        ],
        "korean" | "hangul" => ALL_JAMO.iter().map(|&j| {
            let rom = get_romanization(j);
            let is_vowel = ["a", "ae", "ya", "yae", "eo", "e", "yeo", "ye", "o", "yo", "u", "yu", "eu", "i"].contains(&rom);
            AlphabetLetter {
                char: j.to_string(),
                uppercase: j.to_string(),
                lowercase: j.to_string(),
                name: j.to_string(),
                romanization: rom.to_string(),
                pronunciation: format!("[{}]", rom),
                script: "hangul".to_string(),
                is_vowel,
            }
        }).collect(),
        "latin" => ('A'..='Z').map(|c| {
            let lower = c.to_ascii_lowercase();
            let is_vowel = matches!(c, 'A' | 'E' | 'I' | 'O' | 'U');
            AlphabetLetter {
                char: c.to_string(),
                uppercase: c.to_string(),
                lowercase: lower.to_string(),
                name: c.to_string(),
                romanization: lower.to_string(),
                pronunciation: format!("[{}]", lower),
                script: "latin".to_string(),
                is_vowel,
            }
        }).collect(),
        _ => get_alphabet_letters("cyrillic"),
    }
}

