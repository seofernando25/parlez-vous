# Optional handwriting models

The app builds without handwriting models. Recognition is enabled automatically when the corresponding ONNX file is present at build time.

- Hangul: `character_model.onnx` + `character_model.onnx.data`
- Cyrillic: `cyrillic_model.onnx` + `cyrillic_model.onnx.data`

The tracked Cyrillic model in `../../hangulnist/` is detected automatically. Train the Hangul model with `uv run main.py` from `hangulnist/`, then copy its generated files here for Hangul recognition.
