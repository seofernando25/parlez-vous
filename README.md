# Parlez-vous? 💬

Language learning is a wonderful way to expand the reachable cities of our cognitive highways, so when some fall out of repair you can get to your destination. Learn a language today using AI supports! Live chat encourages active production helping you converse. Privacy focused language learning has never been easier!
---

## 🌐 Project Links & Sites
* **Official Website:** [parlezvous.ca](https://parlezvous.ca)

---

## 🛠️ Built With

![Tauri](https://img.shields.io/badge/Tauri-%2324C8DB?style=for-the-badge&logo=tauri&logoColor=white)
![SvelteKit](https://img.shields.io/badge/Svelte-%23FF3E00?style=for-the-badge&logo=svelte&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white)
![Kotlin](https://img.shields.io/badge/Kotlin-%237F52FF?style=for-the-badge&logo=kotlin&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-%23007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Android](https://img.shields.io/badge/Android-%233DDC84?style=for-the-badge&logo=android&logoColor=white)
![PyTorch](https://img.shields.io/badge/PyTorch-%23EE4C2C?style=for-the-badge&logo=pytorch&logoColor=white)
![ONNX](https://img.shields.io/badge/ONNX-%23005C99?style=for-the-badge&logo=onnx&logoColor=white)
![SQLite](https://img.shields.io/badge/SQLite-%2307405E?style=for-the-badge&logo=sqlite&logoColor=white)

---

## 🚀 Quick Start Installation

See the [Installation Guide](docs/installation.md) for the complete setup. A clean checkout now builds without training the optional handwriting model first.

### 1. Client prerequisites
- Install [Bun](https://bun.sh/) and [Rust](https://www.rust-lang.org/).
- Clone the repository and install the app dependencies:
  ```bash
  cd parlezvous
  bun install
  bun run verify
  ```
  `bun run verify` includes the source-organization gate: authored source files stay at or below 300 LOC.
- Run the desktop app:
  ```bash
  bun tauri dev
  ```

`uv` is only required for optional Python model/services. On desktop, Parlez-vous offers an **Express** private local-AI setup and a focused **Advanced** path for connecting an existing OpenAI-compatible endpoint such as Ollama, LM Studio, OpenRouter, OpenAI, vLLM, or llama.cpp.

### 2. Handwriting models (optional)

The application can launch without the Hangul ONNX model; only Hangul handwriting recognition is disabled. To enable it:

```bash
cd hangulnist
uv sync
uv run main.py
```

The Tauri build automatically detects the generated `hangulnist/character_model.onnx` and its external data file. The tracked Cyrillic model is detected the same way.

### 3. Android development

Install Android Studio plus API 36, Platform Tools, Build Tools, command-line tools, and NDK `30.0.14904198`. Add the Android Rust targets:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

On macOS, the current Gradle wrapper should be run with JDK 21 rather than Android Studio's newer JBR 25:

```bash
export JAVA_HOME="/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home"
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export NDK_HOME="$ANDROID_HOME/ndk/30.0.14904198"
export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
```

With an AVD running, start the app with its AVD name, for example:

```bash
bun tauri android dev medium_phone --no-watch
```

For emulator development, `adb reverse` can make the existing localhost defaults reach services on the Mac:

```bash
adb reverse tcp:11434 tcp:11434 # example local AI endpoint (Ollama preset)
adb reverse tcp:8000 tcp:8000   # Whisper ASR
adb reverse tcp:5050 tcp:5050   # server TTS
```

The Qualcomm runtime fetch script is only needed when targeting the corresponding proprietary physical-device acceleration path; it is not required to boot the standard ARM64 emulator.

### 4. iOS development

The repository includes the generated Tauri/Xcode project plus minimal Swift bridges for the custom plugins. Install the Apple host tools and Rust targets:

```bash
brew install xcodegen libimobiledevice cocoapods
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

Build an unsigned Apple-silicon simulator bundle with:

```bash
bun tauri ios build --debug --target aarch64-sim --no-sign --ci
```

iOS currently uses server-backed OpenAI-compatible AI/ASR/TTS. LiteRT-LM and Supertonic on-device inference remain Android-only; their iOS bridges report those capabilities as unavailable without blocking the rest of the app.

### 5. Optional service helpers
- **Whisper ASR (speech-to-text):** run `docker compose up` inside `whisper/`.
- **Qwen3 TTS (server text-to-speech):**
  ```bash
  cd Qwen3-TTS
  uv run qwen_tts/cli/openai_server.py --port 5050 --checkpoint Qwen3-TTS-12Hz-0.6B-CustomVoice/ --no-flash-attn
  ```
- **On-device Android:** LiteRT handles supported local LLM workloads and Supertonic handles local TTS when their models are installed from Settings.

---

## 🔒 Post Installation Firewall
Allow connection from desired IP addresses to the specific ports:
```bash
sudo ufw allow from <IP_ADDRESS> proto tcp to any port <PORT_NUMBER>
```

---

# License

This project is licensed under the [MIT License](LICENSE).

## Models, Assets & Licenses

Parlez-vous integrates external machine learning models and 3D assets, which are subject to their own respective licensing terms. Users and developers of this application must comply with these terms:

1. **Qwen3.5 / Qwen3 Embeddings** (`Qwen3.5-4B`, `Qwen3-Embedding-0.6B`):
   - **License**: [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
   - Used by the managed desktop Express setup for language tutoring and textbook retrieval.

2. **MiniCPM-V 4.6** (optional Vision capability):
   - **License**: [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)

3. **Gemma 4** (`gemma-4-E2B-it`, Android LiteRT path):
   - **License**: [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)

4. **Supertonic TTS** (`supertonic-3`):
   - **License**: [BigScience Open RAIL-M License](https://huggingface.co/spaces/bigscience/license)
   - A responsible AI license designed for open access while enforcing ethical downstream use restrictions.
   - **Important Use Restrictions**: The model must not be used for unlawful acts, harm to minors, generating disinformation, harassment/impersonation, automated legal decision-making, social scoring or discrimination, medical advice/interpretation, or certain law enforcement activities. Any redistribution or derivation of the model weights must carry these same use-based restrictions.

5. **3D Assets** (`avatar.vrm`, `VRMA_*.vrma`):
   - **License**: Proprietary terms dictated by pixiv Inc.'s VRoid Project.
   - You must include the following attribution in derivatives: "キャラクターアニメーション: ピクシブ株式会社 VRoidプロジェクト" (Character animation credits to pixiv Inc.'s VRoid Project).
   - Commercial use is allowed with credit. See VRoid Hub for full terms.
