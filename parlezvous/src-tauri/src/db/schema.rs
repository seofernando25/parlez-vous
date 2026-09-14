pub const SCHEMA_VERSION: i64 = 2;

pub const SCHEMA_CURRENT: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE app_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
INSERT INTO app_meta (key, value) VALUES ('schema_version', '2');

CREATE TABLE vocabulary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    language_code TEXT NOT NULL,
    target_text TEXT NOT NULL,
    native_text TEXT NOT NULL,
    is_character BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE srs_state (
    vocab_id INTEGER PRIMARY KEY,
    review_level INTEGER NOT NULL DEFAULT 0,
    next_review_date DATETIME,
    ease_factor REAL NOT NULL DEFAULT 2.5,
    interval_days INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(vocab_id) REFERENCES vocabulary(id) ON DELETE CASCADE
);

CREATE TABLE journal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    language_code TEXT NOT NULL,
    date DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    mood_input TEXT,
    weather_input TEXT,
    activity_input TEXT,
    generated_target_text TEXT,
    native_translation TEXT
);

CREATE TABLE journal_vocabulary (
    journal_id INTEGER NOT NULL,
    vocab_id INTEGER NOT NULL,
    PRIMARY KEY (journal_id, vocab_id),
    FOREIGN KEY(journal_id) REFERENCES journal_entries(id) ON DELETE CASCADE,
    FOREIGN KEY(vocab_id) REFERENCES vocabulary(id) ON DELETE CASCADE
);

CREATE TABLE settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    target_language TEXT NOT NULL DEFAULT 'French',
    tts_server_url TEXT NOT NULL DEFAULT 'http://localhost:5050/v1/audio/speech',
    asr_server_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:8000/v1/audio/transcriptions',
    ai_provider TEXT NOT NULL DEFAULT 'managed',
    ai_base_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:11435/v1',
    ai_api_key TEXT NOT NULL DEFAULT '',
    embedding_model TEXT NOT NULL DEFAULT 'parlezvous-embed',
    active_model TEXT NOT NULL DEFAULT 'parlezvous-chat',
    huggingface_token TEXT DEFAULT '',
    litert_accelerator TEXT NOT NULL DEFAULT 'Auto',
    litert_max_tokens INTEGER NOT NULL DEFAULT 1024,
    target_programming_language TEXT NOT NULL DEFAULT 'python',
    coding_theme_category TEXT NOT NULL DEFAULT 'All',
    active_vrm TEXT NOT NULL DEFAULT 'avatar.vrm',
    supertonic_voice_style TEXT NOT NULL DEFAULT 'voice_styles/F1.json',
    tts_provider TEXT NOT NULL DEFAULT 'auto',
    tutor_tone TEXT NOT NULL DEFAULT 'balanced'
);
INSERT INTO settings (id) VALUES (1);

CREATE TABLE user_profile (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    skill_level TEXT NOT NULL DEFAULT 'Beginner',
    tier INTEGER NOT NULL DEFAULT 1,
    active_seconds INTEGER NOT NULL DEFAULT 0,
    display_name TEXT NOT NULL DEFAULT ''
);
INSERT INTO user_profile (id) VALUES (1);

CREATE TABLE document_chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_name TEXT NOT NULL,
    chunk_text TEXT NOT NULL,
    page_number INTEGER NOT NULL DEFAULT 1,
    embedding BLOB,
    embedding_dimensions INTEGER,
    embedding_model TEXT
);
CREATE INDEX document_chunks_document_idx ON document_chunks(document_name);

CREATE TABLE conjugation_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    language_code TEXT NOT NULL,
    verb TEXT NOT NULL,
    tense TEXT NOT NULL,
    subject TEXT NOT NULL,
    answer TEXT NOT NULL,
    translation TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    answered_correct BOOLEAN,
    sentence TEXT NOT NULL DEFAULT ''
);

CREATE TABLE language_curriculum (
    language TEXT PRIMARY KEY,
    current_tier INTEGER NOT NULL DEFAULT 1,
    active_theme_id TEXT NOT NULL,
    total_xp INTEGER NOT NULL DEFAULT 0,
    active_seconds INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_practiced DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE coding_questions_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_type TEXT NOT NULL,
    question_data TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE coding_questions_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_type TEXT NOT NULL,
    question_data TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE language_questions_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_type TEXT NOT NULL,
    question_data TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE language_questions_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_type TEXT NOT NULL,
    question_data TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE avatar_chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    correction TEXT,
    audio_base64 TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
"#;
