struct FalseFriend {
    term: &'static str,
    meaning: &'static str,
}

const PORTUGUESE: &[FalseFriend] = &[
    FalseFriend { term: "pretender", meaning: "to intend or plan" },
    FalseFriend { term: "atualmente", meaning: "currently" },
    FalseFriend { term: "parentes", meaning: "relatives" },
    FalseFriend { term: "esquisito", meaning: "strange or odd" },
    FalseFriend { term: "livraria", meaning: "bookstore" },
    FalseFriend { term: "oficina", meaning: "workshop or repair shop" },
];

const FRENCH: &[FalseFriend] = &[
    FalseFriend { term: "actuellement", meaning: "currently" },
    FalseFriend { term: "librairie", meaning: "bookstore" },
    FalseFriend { term: "assister", meaning: "to attend" },
    FalseFriend { term: "attendre", meaning: "to wait" },
    FalseFriend { term: "sensible", meaning: "sensitive" },
];

const SPANISH: &[FalseFriend] = &[
    FalseFriend { term: "actualmente", meaning: "currently" },
    FalseFriend { term: "embarazada", meaning: "pregnant" },
    FalseFriend { term: "asistir", meaning: "to attend" },
    FalseFriend { term: "librería", meaning: "bookstore" },
];

const ITALIAN: &[FalseFriend] = &[
    FalseFriend { term: "attualmente", meaning: "currently" },
    FalseFriend { term: "libreria", meaning: "bookstore" },
    FalseFriend { term: "camera", meaning: "room" },
    FalseFriend { term: "parenti", meaning: "relatives" },
];

const GERMAN: &[FalseFriend] = &[
    FalseFriend { term: "bekommen", meaning: "to receive" },
    FalseFriend { term: "Gift", meaning: "poison" },
    FalseFriend { term: "eventuell", meaning: "possibly" },
    FalseFriend { term: "aktuell", meaning: "current" },
];

pub fn false_friend_challenge(language: &str, recent: &str) -> Option<String> {
    let entries = entries_for(language)?;
    let context = recent.to_lowercase();
    let item = entries.iter().find(|item| !context.contains(&item.term.to_lowercase())).unwrap_or(&entries[0]);
    Some(format!("What does “{}” mean in {}?", item.term, language))
}


pub fn false_friend_feedback(language: &str, recent: &str, answer: &str) -> Option<Vec<String>> {
    let entries = entries_for(language)?;
    let context = recent.to_lowercase();
    let item = entries.iter().find(|item| {
        let term = item.term.to_lowercase();
        context.contains(&format!("assistant: what does “{term}” mean"))
            || context.contains(&format!("assistant: what does \"{term}\" mean"))
    })?;
    let correct = meaning_matches(answer, item.meaning);
    let first = if correct {
        format!("Exactly — “{}” means {}.", item.term, item.meaning)
    } else {
        format!("Not quite — “{}” means {}.", item.term, item.meaning)
    };
    let next = false_friend_challenge(language, recent);
    Some(match next {
        Some(challenge) if !challenge.to_lowercase().contains(&item.term.to_lowercase()) => vec![first, challenge],
        _ => vec![first],
    })
}

fn entries_for(language: &str) -> Option<&'static [FalseFriend]> {
    match language.trim().to_ascii_lowercase().as_str() {
        "portuguese" => Some(PORTUGUESE),
        "french" => Some(FRENCH),
        "spanish" => Some(SPANISH),
        "italian" => Some(ITALIAN),
        "german" => Some(GERMAN),
        _ => None,
    }
}

fn meaning_matches(answer: &str, meaning: &str) -> bool {
    let answer = answer.to_ascii_lowercase();
    meaning.to_ascii_lowercase().split(" or ").any(|candidate| {
        let candidate = candidate.trim().trim_start_matches("to ");
        answer.contains(candidate)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_away_from_recent_false_friends() {
        let first = false_friend_challenge("Portuguese", "").unwrap();
        let second = false_friend_challenge("Portuguese", &first).unwrap();
        assert!(first.contains("pretender"));
        assert!(second.contains("atualmente"));
        assert_ne!(PORTUGUESE[0].meaning, PORTUGUESE[1].meaning);
    }

    #[test]
    fn grades_a_recent_false_friend_without_an_llm() {
        let recent = "assistant: What does “atualmente” mean in Portuguese?\nuser: currently!";
        let parts = false_friend_feedback("Portuguese", recent, "currently!").unwrap();
        assert!(parts[0].starts_with("Exactly"));
        assert!(parts[0].contains("currently"));
    }
}
