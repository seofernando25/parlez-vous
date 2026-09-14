#[cfg(target_os = "android")]
pub const SCHEMA_V1: &str = "
    CREATE TABLE vocabulary (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        language_code TEXT NOT NULL,
        target_text TEXT NOT NULL,
        native_text TEXT NOT NULL,
        is_character BOOLEAN DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE srs_state (
        vocab_id INTEGER PRIMARY KEY,
        review_level INTEGER DEFAULT 0,
        next_review_date DATETIME,
        ease_factor REAL DEFAULT 2.5,
        interval_days INTEGER DEFAULT 0,
        FOREIGN KEY(vocab_id) REFERENCES vocabulary(id) ON DELETE CASCADE
    );

    CREATE TABLE journal_entries (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        language_code TEXT NOT NULL,
        date DATE DEFAULT CURRENT_TIMESTAMP,
        mood_input TEXT,
        weather_input TEXT,
        activity_input TEXT,
        generated_target_text TEXT,
        native_translation TEXT
    );

    CREATE TABLE journal_vocabulary (
        journal_id INTEGER,
        vocab_id INTEGER,
        PRIMARY KEY (journal_id, vocab_id),
        FOREIGN KEY(journal_id) REFERENCES journal_entries(id) ON DELETE CASCADE,
        FOREIGN KEY(vocab_id) REFERENCES vocabulary(id) ON DELETE CASCADE
    );

    CREATE TABLE settings (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        target_language TEXT NOT NULL DEFAULT 'French',
        tts_server_url TEXT NOT NULL DEFAULT 'http://localhost:5050/v1/audio/speech',
        asr_server_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:8000/v1/audio/transcriptions',
        ollama_server_url TEXT NOT NULL DEFAULT 'http://localhost:11434',
        embedding_model TEXT NOT NULL DEFAULT 'nomic-embed-text',
        active_model TEXT NOT NULL DEFAULT 'gemma-4-E2B-it.litertlm',
        huggingface_token TEXT DEFAULT '',
        litert_accelerator TEXT NOT NULL DEFAULT 'Auto',
        litert_max_tokens INTEGER NOT NULL DEFAULT 5000
    );
    INSERT INTO settings (id) VALUES (1);

    CREATE TABLE user_profile (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        skill_level TEXT NOT NULL DEFAULT 'Beginner',
        tier INTEGER NOT NULL DEFAULT 1,
        active_seconds INTEGER NOT NULL DEFAULT 0
    );
    INSERT INTO user_profile (id) VALUES (1);

    CREATE TABLE document_chunks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        document_name TEXT,
        chunk_text TEXT,
        page_number INTEGER NOT NULL DEFAULT 1
    );

    CREATE VIRTUAL TABLE vec_chunks USING vec0(
        embedding float[768]
    );

    CREATE TABLE conjugation_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        language_code TEXT NOT NULL,
        verb TEXT NOT NULL,
        tense TEXT NOT NULL,
        subject TEXT NOT NULL,
        answer TEXT NOT NULL,
        translation TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        answered_correct BOOLEAN DEFAULT NULL,
        sentence TEXT NOT NULL DEFAULT ''
    );

    CREATE TABLE IF NOT EXISTS language_curriculum (
        language TEXT PRIMARY KEY,
        current_tier INTEGER DEFAULT 1,
        active_theme_id TEXT NOT NULL,
        total_xp INTEGER DEFAULT 0,
        active_seconds INTEGER DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        last_practiced DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    PRAGMA user_version = 1;
";

#[cfg(not(target_os = "android"))]
pub const SCHEMA_V1: &str = "
    CREATE TABLE vocabulary (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        language_code TEXT NOT NULL,
        target_text TEXT NOT NULL,
        native_text TEXT NOT NULL,
        is_character BOOLEAN DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE srs_state (
        vocab_id INTEGER PRIMARY KEY,
        review_level INTEGER DEFAULT 0,
        next_review_date DATETIME,
        ease_factor REAL DEFAULT 2.5,
        interval_days INTEGER DEFAULT 0,
        FOREIGN KEY(vocab_id) REFERENCES vocabulary(id) ON DELETE CASCADE
    );

    CREATE TABLE journal_entries (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        language_code TEXT NOT NULL,
        date DATE DEFAULT CURRENT_TIMESTAMP,
        mood_input TEXT,
        weather_input TEXT,
        activity_input TEXT,
        generated_target_text TEXT,
        native_translation TEXT
    );

    CREATE TABLE journal_vocabulary (
        journal_id INTEGER,
        vocab_id INTEGER,
        PRIMARY KEY (journal_id, vocab_id),
        FOREIGN KEY(journal_id) REFERENCES journal_entries(id) ON DELETE CASCADE,
        FOREIGN KEY(vocab_id) REFERENCES vocabulary(id) ON DELETE CASCADE
    );

    CREATE TABLE settings (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        target_language TEXT NOT NULL DEFAULT 'French',
        tts_server_url TEXT NOT NULL DEFAULT 'http://localhost:5050/v1/audio/speech',
        asr_server_url TEXT NOT NULL DEFAULT 'http://127.0.0.1:8000/v1/audio/transcriptions',
        ollama_server_url TEXT NOT NULL DEFAULT 'http://localhost:11434',
        embedding_model TEXT NOT NULL DEFAULT 'nomic-embed-text',
        active_model TEXT NOT NULL DEFAULT '',
        huggingface_token TEXT DEFAULT '',
        litert_accelerator TEXT NOT NULL DEFAULT 'Auto',
        litert_max_tokens INTEGER NOT NULL DEFAULT 5000
    );
    INSERT INTO settings (id) VALUES (1);

    CREATE TABLE user_profile (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        skill_level TEXT NOT NULL DEFAULT 'Beginner',
        tier INTEGER NOT NULL DEFAULT 1,
        active_seconds INTEGER NOT NULL DEFAULT 0
    );
    INSERT INTO user_profile (id) VALUES (1);

    CREATE TABLE document_chunks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        document_name TEXT,
        chunk_text TEXT,
        page_number INTEGER NOT NULL DEFAULT 1
    );

    CREATE VIRTUAL TABLE vec_chunks USING vec0(
        embedding float[768]
    );

    CREATE TABLE conjugation_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        language_code TEXT NOT NULL,
        verb TEXT NOT NULL,
        tense TEXT NOT NULL,
        subject TEXT NOT NULL,
        answer TEXT NOT NULL,
        translation TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        answered_correct BOOLEAN DEFAULT NULL,
        sentence TEXT NOT NULL DEFAULT ''
    );

    CREATE TABLE IF NOT EXISTS language_curriculum (
        language TEXT PRIMARY KEY,
        current_tier INTEGER DEFAULT 1,
        active_theme_id TEXT NOT NULL,
        total_xp INTEGER DEFAULT 0,
        active_seconds INTEGER DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        last_practiced DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    PRAGMA user_version = 1;
";

pub const SCHEMA_V2: &str = "
    ALTER TABLE settings ADD COLUMN target_programming_language TEXT NOT NULL DEFAULT 'python';

    CREATE TABLE coding_questions_queue (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        question_type TEXT NOT NULL,
        question_data TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    PRAGMA user_version = 2;
";

pub const SCHEMA_V3: &str = "
    ALTER TABLE settings ADD COLUMN coding_theme_category TEXT NOT NULL DEFAULT 'All';
    ALTER TABLE settings ADD COLUMN active_vrm TEXT NOT NULL DEFAULT 'avatar.vrm';
    ALTER TABLE settings ADD COLUMN supertonic_voice_style TEXT NOT NULL DEFAULT 'voice_styles/F1.json';
    PRAGMA user_version = 3;
";

pub const SCHEMA_V4: &str = "
    CREATE TABLE language_questions_queue (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        question_type TEXT NOT NULL,
        question_data TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    PRAGMA user_version = 4;
";

pub const SCHEMA_V5: &str = "
    CREATE TABLE coding_questions_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        question_type TEXT NOT NULL,
        question_data TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE language_questions_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        question_type TEXT NOT NULL,
        question_data TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    PRAGMA user_version = 5;
";

pub const SCHEMA_V6: &str = "
    PRAGMA user_version = 6;
";

pub const SCHEMA_V7: &str = "
    CREATE TABLE avatar_chat_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        role TEXT NOT NULL,
        content TEXT NOT NULL,
        correction TEXT,
        audio_base64 TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    PRAGMA user_version = 7;
";


pub const SCHEMA_V8: &str = "
    ALTER TABLE settings ADD COLUMN tts_provider TEXT NOT NULL DEFAULT 'auto';
    PRAGMA user_version = 8;
";

pub const SCHEMA_V9: &str = "
    ALTER TABLE user_profile ADD COLUMN display_name TEXT NOT NULL DEFAULT '';
    PRAGMA user_version = 9;
";

pub const DB_VERSION_NUM: usize = 9;
