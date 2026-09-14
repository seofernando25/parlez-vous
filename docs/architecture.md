# System Architecture

**Parlez-vous?** is built as a local-first application using a hybrid framework combining web technology with a native systems language. 

The architecture is divided into two primary execution environments:
1. **Frontend (User Interface Layer)**: Built with SvelteKit, TypeScript, and Vite.
2. **Backend (Systems & Security Layer)**: Built with Rust and Tauri.

```mermaid
graph TD
    A[SvelteKit Frontend] <-->|Tauri IPC Invoke / Events| B[Tauri Rust Backend]
    B -->|rusqlite / sqlite-vec| C[(SQLite Database)]
    B -->|ONNX Runtime / Burn| D[Hangul Handwriting Model]
    B -->|OpenAI-compatible HTTP| E[Remote AI: Ollama / LM Studio / OpenRouter / OpenAI / Custom]
    B -->|Local HTTP API| F[Whisper ASR / Qwen3-TTS Servers]
    B -->|LiteRT C API| G[On-Device Gemma Model (Android)]
```

---

## 1. Frontend & Backend Communication (IPC Bridge)

The frontend communicates with the backend via Tauri's Inter-Process Communication (IPC) bridge. It invokes Rust functions registered as commands and listens to system events emitted by Rust.

### Key Commands Registered in `lib.rs`:
- **`generate_journal`**: Requests LLM prompt generation based on mood, weather, and current tier. Emits extracted vocabulary to the frontend.
- **`chat_with_avatar`**: Orchestrates character conversational AI, combining chat history, local PDF textbook context, active tier theme, and multi-modal (audio/image) parameters.
- **`infer_character`**: Accepts a canvas pixel array representing handwritten Korean Hangul, invokes the ONNX Jamo model, and returns the recognized character.
- **`generate_tts_audio`**: Dispatches text output to OpenAI-compatible TTS APIs or the local Supertonic engine.
- **`upload_and_ingest_textbook`**: Initiates PDF extraction, text chunking, embedding generation, and vector insertion.

---

## 2. Database Schema & Migration Manager

The database is built on **SQLite**, accessed via the `rusqlite` crate in Rust. It utilizes the `sqlite-vec` extension to enable fast, local vector searches directly inside SQLite.

### Database Tables:
- **`vocabulary`**: Stores target words, translations, and word categories (character vs. word).
- **`srs_state`**: Spaced repetition tracking for flashcards.
- **`journal_entries`**: History of generated prompts, responses, grading, and dates.
- **`conjugation_history`**: Tracking stats on verb conjugations to calculate user weaknesses.
- **`language_curriculum`**: Tracks cumulative XP, active seconds, and current tier for each target language.
- **`document_chunks` & `vec_chunks`**: Store extracted PDF texts and their respective vector embeddings for RAG.

Migrations are managed linearly via `PRAGMA user_version` inside `db/mod.rs`. New tables and columns are incrementally upgraded on application boot up.

---

## 3. Local RAG (Retrieval-Augmented Generation)

To provide highly specific reference material during chat sessions (e.g. referencing textbook contents), the application includes a local RAG pipeline:

1. **PDF Text Extraction**: Extracted using the `pdf-extract` crate.
2. **Chunking**: Text is split into overlapping chunks of ~1000 characters, respecting character boundaries and paragraph markers.
3. **Embedding Generation**: Chunks use the configured OpenAI-compatible `/v1/embeddings` endpoint. The current sqlite-vec index is fixed at 768 dimensions, so the embedding model must support a 768-dimensional response.
4. **Vector Storage**: Inserted as byte slices into the virtual `vec_chunks` table utilizing the `sqlite-vec` index.
5. **Context Querying**: During character chat, the user's latest prompt is embedded, and a cosine-similarity query returns the most relevant textbook page chunks to inject into the LLM system prompt.

---

## 4. AI Provider Boundary

Remote inference has one implementation: `OpenAiCompatibleAdapter`. It targets the broadly-supported OpenAI-compatible surface (`/v1/models`, `/v1/chat/completions`, and `/v1/embeddings`) over `reqwest`. Provider names in Settings are presets for endpoint/auth defaults rather than separate backend implementations.

- **Ollama**: `http://localhost:11434/v1`
- **LM Studio**: `http://localhost:1234/v1`
- **OpenRouter**: `https://openrouter.ai/api/v1`
- **OpenAI**: `https://api.openai.com/v1`
- **OpenAI-compatible**: arbitrary compatible endpoint (vLLM, llama.cpp servers, ModelScope-served endpoints, and similar gateways)

Android LiteRT remains a separate provider because it is a native on-device runtime rather than an HTTP API. A provider-specific implementation should only be added when the product needs behavior that cannot be represented by the compatibility layer, such as OAuth, a proprietary protocol, or a provider-specific catalog.
