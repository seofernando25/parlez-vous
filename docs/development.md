# Developer & Contribution Guide

Instructions on how to extend the codebase, train custom models, and modify curriculum parameters.

---


## Source organization invariant

Authored source files must stay at or below **300 lines of code**. The rule applies to TypeScript, JavaScript, Svelte, CSS, Rust, Kotlin, and Gradle Kotlin files under the app, native plugins, and website. Generated/vendor/build output is excluded.

The invariant is enforced by:

```bash
cd parlezvous
bun run check:loc
```

`bun run verify` includes this check before type diagnostics, tests, and the production build. Prefer extracting cohesive domain modules, controllers/services, or focused components rather than mechanically splitting a large file by line count. Routes and plugin entrypoints should stay thin; stateful orchestration belongs in domain controllers/services, while pure data/transforms belong in standalone modules.

---

## 1. Training & Exporting the Jamo CNN Model

The handwriting canvas utilizes a custom-trained CNN classification model built in PyTorch.

1. **Prerequisites**: Make sure you have `uv` installed. Navigate to the `hangulnist` folder:
   ```bash
   cd hangulnist
   ```
2. **Setup**: Install python dependencies:
   ```bash
   uv sync
   ```
3. **Training pipeline**:
   - `main.py` is configured to download the **Wayperwayp Hangul Characters Dataset** from Kaggle Hub.
   - It trains a 3-layer Convolutional Neural Network with Batch Normalization and Focal Loss (to focus on hard-to-distinguish stroke differences).
   - Images are resized to `28x28`, inverted, and threshold-sharpened.
   - To trigger the training:
     ```bash
     uv run main.py
     ```
4. **ONNX Export**:
   - The training script automatically compiles and exports the trained PyTorch network weights into an ONNX model file named `character_model.onnx` and its weight data file `character_model.onnx.data`.
5. **Build integration**: Keep `character_model.onnx` and `character_model.onnx.data` together in `hangulnist/`. The Tauri build detects them there automatically. The model is optional: without it, the rest of the application still compiles and only Hangul handwriting inference is disabled.

---

## 2. Modifying the Curriculum Tiers & Themes

The learning structure is defined inside `parlezvous/src/lib/curriculum.ts`:

- The curriculum maps levels 1 to 7 corresponding to CEFR levels from **A1** (The Outskirts) to **Mastery/C2** (The Horizon).
- Each tier contains a specific set of thematic modules (e.g. `greetings`, `numbers`, `family`, `shopping`, `travel`, `science`).
- To add a new theme:
  1. Add a new object inside the `themes` array of the appropriate `Tier` in `CURRICULUM_TIERS`.
  2. Define a unique `id`, a friendly `name`, and a detailed `description`.
  3. The LLM generation services will automatically query the theme database and use the description as semantic seeds in prompts.

---

## 3. Database Schema

The application deliberately uses a **single canonical greenfield schema** in `parlezvous/src-tauri/src/db/schema.rs`. Do not add a historical `SCHEMA_VN` migration chain.

When the product schema changes during this pre-stable phase:

1. Edit `SCHEMA_CURRENT` so a new database is created in the desired final form.
2. Update `SCHEMA_VERSION` only when an already-created canonical database must be invalidated.
3. Update direct schema tests and SQL callers together.
4. Keep `db/mod.rs` responsible only for recognizing the canonical schema, backing up an incompatible old database, and creating the new one.

Pre-greenfield databases are preserved as timestamped `sqlite.legacy-*.db` files rather than being dragged through historical migrations.
---

## Tutor quality gate

After Express Local AI is installed and running, validate the shipping teacher pipeline with:

```bash
cd parlezvous
bun run eval:tutor
```

This is intentionally an ignored model acceptance test rather than a normal CI test because it requires the multi-gigabyte managed model. Keep ordinary unit/integration tests deterministic; use the tutor gate when changing tutor prompts, orchestration, model manifests, or llama.cpp inference settings.
