# Parlez-vous Runtime & Refactor Notes

This note captures the behavior discovered and the architecture changed during the September 2026 setup/refactor pass.

## Runtime map

```text
SvelteKit UI
  ├─ Tauri IPC ──> Rust AppState
  │                ├─ AiRouter
  │                │   ├─ generation requests
  │                │   │   ├─ *.litertlm on Android -> lazy LiteRtAdapter
  │                │   │   └─ everything else -> OpenAiCompatibleAdapter
  │                │   └─ embeddings -> OpenAiCompatibleAdapter
  │                ├─ SQLite + sqlite-vec
  │                ├─ RAG: PDF -> chunks -> embeddings -> vector search
  │                └─ TtsProvider
  │                    ├─ OpenAI-compatible HTTP TTS
  │                    └─ Android Supertonic
  ├─ WebAudio microphone capture
  │   ├─ audio-capable on-device model -> WAV -> base64 -> multimodal chat
  │   └─ other models -> WAV -> Whisper-compatible ASR -> text chat
  └─ WebAudio playback -> lip-sync analyser -> VRM avatar
```

## What changed


### Codebase organization

The refactor now enforces a hard **300 LOC maximum for every authored source file** (TS/JS/Svelte/CSS/Rust/Kotlin/KTS) through `bun run check:loc`, which is part of `bun run verify`. Generated Android/Tauri output and dependencies are intentionally excluded.

Major decomposition boundaries now include:

- Tauri commands grouped by domain instead of one monolithic `lib.rs`;
- Avatar orchestration split into chat, voice capture, VRM stage/TTS, textbook, handwriting, and small view components;
- Settings split into a controller plus focused settings sections;
- shared handwriting rasterization reused by Avatar and Alphabet practice;
- AI contracts separated into provider capabilities, prompt families, context management, and transport/native implementations;
- one OpenAI-compatible remote adapter handles model discovery, chat, structured generation, multimodal images, and embeddings; LiteRT remains the Android-native provider;
- Supertonic split into text processing, chunking, synthesis, model/style loading, audio I/O, configuration, and thermal topology/policy;
- Android LiteRT split into a thin Tauri plugin bridge, runtime/session manager, model downloads, gallery handling, and argument types.

The intent is not small files for their own sake: each file should have one stable reason to change and expose the narrowest useful boundary.

### Runtime AI routing

AI generation is no longer frozen to one provider at Tauri startup. `AiRouter` routes `.litertlm` models to Android LiteRT and everything else to the saved OpenAI-compatible endpoint. Changing remote presets/endpoints takes effect without rebuilding provider code.

LiteRT instances are created lazily on Android and are recreated if the LiteRT model/accelerator/token-limit configuration changes.

Embedding generation is a separate `EmbeddingProvider` capability. RAG uses the configured OpenAI-compatible `/v1/embeddings` endpoint even when chat runs through LiteRT; LiteRT does not currently implement embeddings.

The frontend now centralizes model capability decisions in `src/lib/ai/capabilities.ts` rather than scattering `model.includes("litert")` checks across chat, audio, and puzzle generation.

### Media pipeline

Speech playback has one frontend contract: every TTS provider returns playable WAV bytes. Supertonic's raw mono PCM is wrapped into WAV at the provider boundary in Rust, so sentence chunking, playback-rate control, animation tags, error handling, and avatar lip sync share the same browser playback path. Android is local-first, with fallback to the configured server on each failed sentence chunk so a partial Supertonic failure does not drop later speech or replay chunks that already succeeded.

Voice capture owns WAV encoding in `src/lib/media/wav.ts` and ASR transport in `src/lib/media/asr.ts`. Runtime/platform detection lives in `src/lib/platform.ts`.

### Fresh-clone build

Handwriting ONNX assets are optional at build time. A clean checkout can compile and launch without training the 30-epoch Hangul CNN first. The tracked Cyrillic ONNX model under `hangulnist/` is detected automatically.

Android debug Gradle configuration no longer requires a release keystore. The Android application id/namespace are aligned so Tauri can install and launch the generated activity.

### Bun migration

Both `parlezvous/` and `website/` use Bun lockfiles. Tauri hooks, the generated Android build task, docs, and the GitHub Pages workflow use Bun instead of pnpm.

## Verified development path

The app was built, installed, and launched on an ARM64 Android API 36 emulator (`medium_phone`) on Apple silicon using:

- Bun 1.4.2
- Rust 1.98
- JDK 21
- Android SDK 36
- NDK 30.0.14904198
- Gradle 8.14.3

The first Android cross-compile is large; subsequent builds reuse the Rust/Gradle caches and are substantially faster.

The initial iOS scaffold is also checked in and verified on an ARM64 iOS simulator with Xcode 26.6. `bun tauri ios build --debug --target aarch64-sim --no-sign --ci` produces a launchable simulator bundle. iOS uses server-backed OpenAI-compatible generation and speech for now; the LiteRT and Supertonic Swift bridges expose capability-unavailable stubs until native Apple inference is implemented.

Android debug packaging also strips Rust DWARF symbols in the generated build task (opt out with `PARLEZVOUS_KEEP_ANDROID_SYMBOLS=1`). The final verified ARM64 debug APK is about 140.6 MB instead of the ~575 MB compressed payload produced when the 483 MiB unstripped Rust library was packaged; the stripped Rust library is ~73.9 MiB. Duplicate generated Cyrillic ONNX assets were removed because Burn embeds the compiled model in the Rust library. The runtime Burn dependency disables its default feature set and enables only `std` + `ndarray`; `burn-store` is likewise limited to `std` + Burnpack instead of pulling PyTorch/safetensors loaders.

## Design system

The UI now uses:

- a 4 px spatial base unit;
- a 1.2 minor-third modular type scale;
- OKLCH semantic colors for predictable perceived-lightness changes across light/dark themes;
- semantic roles (`canvas`, `surface`, `foreground`, `muted`, `accent`, `success`, `danger`, etc.) rather than direct Zinc/Yellow utilities;
- seven curriculum tier tokens on an evenly stepped hue progression;
- System, Light, and Dark appearance modes persisted locally;
- reduced-motion behavior and semantic focus/selection colors;
- a five-icon mobile primary navigation (`Learn`, `Journal`, `Tutor`, `Practice`, `Profile`) and a collapsible desktop sidebar with account/settings separated from learning destinations.

## Remaining high-value work

- **Offline RAG embeddings:** textbook ingestion/search uses the remote compatibility endpoint. Fully offline Android RAG still needs an on-device embedding provider and a separate embedding-model lifecycle. The current local vector index is fixed at 768 dimensions.
- **VRM distribution:** the two public VRM files still total roughly 47 MB. The accidental second hashed Vite copy is gone, but the remaining avatars should eventually be compressed or installed/downloaded on demand rather than shipped in every static distribution.
- **Native build debt:** the Android build is green, but upstream Tauri/Gradle code still emits deprecation notices. These should be handled during a deliberate Tauri/Gradle upgrade rather than mixed into application refactors.
- **Broader behavior coverage:** TTS provider routing, chat correction persistence, and the v7→v8 settings migration now have direct regression tests, and the Android artifact is smoke-tested. The next testing step is controller-level interaction coverage for the larger learning flows.

## Current capability boundaries

```text
AiRouter
  ├─ ModelProvider
  ├─ JournalProvider
  ├─ ChatProvider
  ├─ ConjugationProvider
  ├─ PuzzleProvider
  └─ EmbeddingProvider (OpenAI-compatible remote endpoint)

Media
  ├─ VoiceCaptureController -> Speech-to-text / multimodal audio
  ├─ TTS provider policy -> Auto / Supertonic / Server
  ├─ shared WAV playback -> animations + lip sync
  └─ shared handwriting rasterization
```

Avatar, Settings, Alphabet practice, Conjugator, Tauri commands, provider implementations, native plugins, curriculum data, notifications, DB/model code, and website presentation have all been decomposed under the 300 LOC invariant.
