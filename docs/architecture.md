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
    B -->|Managed localhost API| E[Bundled llama.cpp desktop runtime]
    B -->|OpenAI-compatible HTTP| F[External AI: Ollama / LM Studio / OpenRouter / OpenAI / Custom]
    B -->|Local HTTP API| G[Whisper ASR / Qwen3-TTS Servers]
    B -->|LiteRT C API| H[On-Device Gemma Model (Android)]
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

## 2. Canonical Database Schema

The database is built on **SQLite**, accessed via the `rusqlite` crate in Rust. It utilizes the `sqlite-vec` extension to enable fast, local vector searches directly inside SQLite.

### Database Tables:
- **`vocabulary`**: Stores target words, translations, and word categories (character vs. word).
- **`srs_state`**: Spaced repetition tracking for flashcards.
- **`journal_entries`**: History of generated prompts, responses, grading, and dates.
- **`conjugation_history`**: Tracking stats on verb conjugations to calculate user weaknesses.
- **`language_curriculum`**: Tracks cumulative XP, active seconds, and current tier for each target language.
- **`document_chunks`**: Stores extracted textbook chunks plus the embedding BLOB, model id, and vector dimension used for that row.

The project is intentionally greenfield: `schema.rs` defines one canonical schema instead of preserving a historical migration ladder. A database created by the pre-greenfield builds is renamed to `sqlite.legacy-<timestamp>.db` once and a fresh canonical database is created. This keeps the schema understandable while retaining the old file as a manual backup.

---

## 3. Local RAG (Retrieval-Augmented Generation)

To provide highly specific reference material during chat sessions (e.g. referencing textbook contents), the application includes a local RAG pipeline:

1. **PDF Text Extraction**: Extracted using the `pdf-extract` crate.
2. **Chunking**: Text is split into overlapping chunks of ~1000 characters, respecting character boundaries and paragraph markers.
3. **Embedding Generation**: Chunks use the selected embedding model through the OpenAI-compatible `/v1/embeddings` capability. Managed desktop AI uses Qwen3.5-4B for language generation and Qwen3-Embedding-0.6B for retrieval behind stable aliases. MiniCPM-V is an optional Vision capability rather than part of Express setup.
4. **Vector Storage**: Each chunk stores its raw float-vector BLOB together with `embedding_dimensions` and `embedding_model`. The database is therefore not coupled to a global vector width.
5. **Context Querying**: `sqlite-vec`'s scalar cosine-distance function ranks rows produced by the same embedding model, so 768-, 1024-, or other-dimensional embedding families can coexist safely.

---

## 4. AI Provider Boundary

Generation and embeddings have one HTTP implementation: `OpenAiCompatibleAdapter`. It targets `/v1/models`, `/v1/chat/completions`, and `/v1/embeddings` over `reqwest`. The default desktop experience is a managed llama.cpp runtime installed into the app data directory and exposed only on localhost. The same adapter also supports external endpoints, so local and hosted AI do not create parallel inference stacks.

Normal users only see whether Local AI is set up and ready. Endpoint, API-key, and model-id controls live under Advanced settings.

- **On this device**: managed llama.cpp at `http://127.0.0.1:11435/v1`
- **Ollama**: `http://localhost:11434/v1`
- **LM Studio**: `http://localhost:1234/v1`
- **OpenRouter**: `https://openrouter.ai/api/v1`
- **OpenAI**: `https://api.openai.com/v1`
- **OpenAI-compatible**: arbitrary compatible endpoint (vLLM, llama.cpp servers, ModelScope-served endpoints, and similar gateways)

Android LiteRT remains a separate provider because it is a native on-device runtime rather than an HTTP API. A provider-specific implementation should only be added when the product needs behavior that cannot be represented by the compatibility layer, such as OAuth, a proprietary protocol, or a provider-specific catalog.

### Tutor orchestration

Tutor turns are intentionally decomposed instead of asking one model to do everything in one prompt. A narrow turn-analysis call identifies the immediate task; exact language/reference work runs at low temperature; conversation uses a separate short-bubble contract; correction can run concurrently when needed. Deterministic helpers handle literal multi-bubble requests and curated false-friend drills so the model is not asked to invent facts that the application can know directly. Responses are emitted as 1–5 short parts and TTS speaks each part separately.

The managed teacher is acceptance-tested with `bun run eval:tutor`. The gate covers translation, multilingual morphology, contextual correction, register, false-premise handling, short-message cadence, exact five-bubble output, and Portuguese false-friend behavior.
