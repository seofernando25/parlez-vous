# Parlez-vous? Documentation

Welcome to the technical documentation for **Parlez-vous?**, a next-generation local first language learning application designed to build strong cognitive highways through active learning, conversational roleplaying, Spaced Repetition, and native handwriting support.

## Project Vision

Unlike traditional, gamified, and inductive language learning platforms (such as Duolingo) which lock features behind paywalls and focus primarily on passive pattern recognition, **Parlez-vous?** is built on the philosophy of **actionable, active learning**. 

The application integrates cutting-edge AI technologies and cognitive science to help users practice production-level skills (writing, speaking, and structure construction) in a self-hosted, private, and free environment.

---

## Documentation Sections

Explore the documentation pages using the sidebar navigation or index links below:

### 1. [System Architecture](architecture.md)
Learn about the dual-process architecture combining a Rust Tauri backend with a SvelteKit frontend, IPC (Inter-Process Communication) bridge commands, the canonical SQLite schema, managed desktop AI, and local vector RAG setup.

### 2. [Application Features](features.md)
Detailed functional details of core modules including the Spaced Repetition System (SRS) algorithm, AI-guided Journaling & Grading, interactive 3D VRM Avatar, and ONNX-based Hangul Handwriting Canvas.

### 3. [Installation & Setup](installation.md)
Complete guide to building and running the Tauri desktop app, generating the Android APK package, and deploying helper containers like Whisper ASR and Qwen3-TTS.

### 4. [Development Guide](development.md)
Guidelines for extending the application, updating the database schema, modifying the learning curriculum, and training or exporting the Hangul Jamo CNN model using the PyTorch-to-ONNX pipeline.
