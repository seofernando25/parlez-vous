use super::super::{openai_compat_adapter::OpenAiCompatibleAdapter, ChatMessage, ChatProvider};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

fn adapter(tone: &str) -> (OpenAiCompatibleAdapter, String) {
    let base_url = std::env::var("PARLEZVOUS_EVAL_URL").unwrap_or_else(|_| "http://127.0.0.1:11435/v1".into());
    let model = std::env::var("PARLEZVOUS_EVAL_MODEL").unwrap_or_else(|_| "parlezvous-chat".into());
    let provider = std::env::var("PARLEZVOUS_EVAL_PROVIDER").unwrap_or_else(|_| if base_url.contains("11435") { "managed".into() } else { "custom".into() });
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(crate::db::SCHEMA_CURRENT).unwrap();
    conn.execute(
        "UPDATE settings SET tutor_tone=?1, ai_provider=?2, ai_base_url=?3, active_model=?4 WHERE id=1",
        rusqlite::params![tone, provider, base_url, model],
    ).unwrap();
    (OpenAiCompatibleAdapter::new(Arc::new(Mutex::new(conn))), model)
}

async fn ask(
    ai: &OpenAiCompatibleAdapter,
    language: &str,
    skill: &str,
    model: &str,
    history: Vec<(&str, &str)>,
) -> crate::ai::ChatResponse {
    let messages = history.into_iter().map(|(role, content)| ChatMessage {
        role: role.to_string(), content: content.to_string(), audio_base64: None,
    }).collect();
    ai.generate_chat_response(
        messages,
        model.into(),
        language.into(),
        skill.into(),
        String::new(),
        None,
        None,
        None,
        None,
    ).await.unwrap()
}

fn joined(response: &crate::ai::ChatResponse) -> String {
    response.response_parts.join(" ").to_lowercase()
}

fn assert_chat_shape(label: &str, response: &crate::ai::ChatResponse) {
    assert!(!response.response_parts.is_empty(), "{label}: no bubbles");
    assert!(response.response_parts.len() <= 5, "{label}: too many bubbles: {:?}", response.response_parts);
    for part in &response.response_parts {
        let words = part.split_whitespace().count();
        assert!(words <= 36, "{label}: bubble too long ({words} words): {part}");
    }
    let total = response.response_parts.iter().map(|part| part.split_whitespace().count()).sum::<usize>();
    assert!(total <= 90, "{label}: response too long ({total} words): {:?}", response.response_parts);
}

#[tokio::test]
#[ignore = "requires the managed local tutor runtime on 127.0.0.1:11435"]
async fn local_teacher_quality_gate() {
    let (ai, model) = adapter("balanced");

    let coffee = ask(&ai, "French", "Beginner", &model, vec![("user", "How do I say 'I would like a coffee' in French?")]).await;
    println!("coffee: {:?}", coffee.response_parts);
    assert_chat_shape("coffee", &coffee);
    let text = joined(&coffee);
    assert!(text.contains("je voudrais") || text.contains("j’aimerais") || text.contains("j'aimerais"), "coffee: {text}");

    let aller = ask(&ai, "French", "Beginner", &model, vec![("user", "Conjugate aller in the present tense for all six persons.")]).await;
    println!("aller: {:?}", aller.response_parts);
    assert_chat_shape("aller", &aller);
    let text = joined(&aller);
    for form in ["vais", "vas", "va", "allons", "allez", "vont"] { assert!(text.contains(form), "aller missing {form}: {text}"); }


    let portuguese_conjugation = ask(&ai, "Portuguese", "Beginner", &model, vec![("user", "Conjugate dançar in the present tense for eu, tu, ele/ela, nós, vós, eles/elas.")]).await;
    println!("dançar: {:?}", portuguese_conjugation.response_parts);
    assert_chat_shape("dançar", &portuguese_conjugation);
    let text = joined(&portuguese_conjugation);
    for form in ["eu danço", "tu danças", "ele/ela dança", "nós dançamos", "vós dançais", "eles/elas dançam"] { assert!(text.contains(form), "dançar missing {form}: {text}"); }

    let spanish_conjugation = ask(&ai, "Spanish", "Beginner", &model, vec![("user", "Conjugate bailar in the present tense for yo, tú, él/ella/usted, nosotros, vosotros, ellos/ellas/ustedes.")]).await;
    println!("bailar: {:?}", spanish_conjugation.response_parts);
    assert_chat_shape("bailar", &spanish_conjugation);
    let text = joined(&spanish_conjugation);
    for form in ["yo bailo", "tú bailas", "él/ella/usted baila", "nosotros", "bailamos", "vosotros", "bailáis", "ellos/ellas/ustedes bailan"] { assert!(text.contains(form), "bailar missing {form}: {text}"); }

    let italian_conjugation = ask(&ai, "Italian", "Beginner", &model, vec![("user", "Conjugate andare in the present tense for io, tu, lui/lei, noi, voi, loro.")]).await;
    println!("andare: {:?}", italian_conjugation.response_parts);
    assert_chat_shape("andare", &italian_conjugation);
    let text = joined(&italian_conjugation);
    for form in ["io vado", "tu vai", "lui/lei va", "noi andiamo", "voi andate", "loro vanno"] { assert!(text.contains(form), "andare missing {form}: {text}"); }

    let german_conjugation = ask(&ai, "German", "Beginner", &model, vec![("user", "Conjugate gehen in the present tense for ich, du, er/sie/es, wir, ihr, sie/Sie.")]).await;
    println!("gehen: {:?}", german_conjugation.response_parts);
    assert_chat_shape("gehen", &german_conjugation);
    let text = joined(&german_conjugation);
    for form in ["ich gehe", "du gehst", "er/sie/es geht", "wir gehen", "ihr geht", "sie/sie gehen"] { assert!(text.contains(form), "gehen missing {form}: {text}"); }

    let correction = ask(&ai, "French", "Intermediate", &model, vec![("user", "Je suis allé au magasin hier et j’achète du pain.")]).await;
    println!("correction: {:?} / {:?}", correction.response_parts, correction.idealized_correction);
    assert_chat_shape("correction", &correction);
    let text = format!("{} {}", joined(&correction), correction.idealized_correction.clone().unwrap_or_default().to_lowercase());
    assert!(text.contains("j’ai acheté") || text.contains("j'ai acheté"), "correction: {text}");

    let premise = ask(&ai, "French", "Intermediate", &model, vec![("user", "Did Napoleon intentionally make French nasal?")]).await;
    println!("premise: {:?}", premise.response_parts);
    assert_chat_shape("premise", &premise);
    let text = joined(&premise);
    assert!(text.contains("no") || text.contains("not") || text.contains("didn't") || text.contains("didn’t") || text.contains("non"), "false premise not rejected: {text}");
    for invented in ["norman", "revolution", "academy", "xi century", "11th century"] {
        assert!(!text.contains(invented), "false-premise answer invented unsupported detail ({invented}): {text}");
    }

    let register = ask(&ai, "French", "Beginner", &model, vec![("user", "What's the difference between tu and vous?")]).await;
    println!("register: {:?}", register.response_parts);
    assert_chat_shape("register", &register);
    let text = joined(&register);
    assert!(text.contains("tu") && text.contains("vous"), "register: {text}");
    assert!(text.contains("informal") || text.contains("formal") || text.contains("friend") || text.contains("respect"), "register: {text}");

    let bubbles = ask(&ai, "French", "Beginner", &model, vec![("user", "Send exactly five separate tiny message bubbles. Say: one, two, three, four, five. One word per bubble.")]).await;
    println!("bubbles: {:?}", bubbles.response_parts);
    assert_eq!(bubbles.response_parts.len(), 5, "explicit five-bubble request was not honored");
    for (actual, expected) in bubbles.response_parts.iter().zip(["one", "two", "three", "four", "five"]) {
        assert_eq!(actual.trim().to_lowercase(), expected);
    }

    let portuguese = ask(&ai, "Portuguese", "Beginner", &model, vec![
        ("user", "I'm only learning Portuguese."),
        ("assistant", "Got it."),
        ("user", "Give me one false-friend challenge."),
    ]).await;
    println!("portuguese: {:?}", portuguese.response_parts);
    assert_chat_shape("portuguese", &portuguese);
    let text = joined(&portuguese);
    assert!(!text.contains("french"), "language leakage: {text}");
    let known_false_friends = ["pretender", "atualmente", "pasta", "parentes", "esquisito", "assistir", "livraria", "oficina"];
    assert!(known_false_friends.iter().any(|term| text.contains(term)), "not a useful Portuguese/English false-friend challenge: {text}");
    assert!(!text.contains(" means ") && !text.contains(" = "), "challenge revealed its own answer: {text}");

    let current = ask(&ai, "Portuguese", "Beginner", &model, vec![("user", "What does atualmente mean in Portuguese?")]).await;
    println!("atualmente: {:?}", current.response_parts);
    assert_chat_shape("atualmente", &current);
    let text = joined(&current);
    assert!(text.contains("currently") || text.contains("nowadays") || text.contains("presently"), "atualmente: {text}");
}
