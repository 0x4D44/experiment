# Telemetry Captures

Store deterministic DOSBox-X capture metadata here. Each capture should include:
- `recording_id` – string referenced by `source_recording_id` in `symbols.json`.
- `input_seed` – parameters used to drive scripted gameplay.
- `hashes` – SHA256 for raw capture data under `f1gp-data/dos_traces/`.

Format TBD (JSON or YAML) – finalize during Stage 4 when DOSBox harness lands.
