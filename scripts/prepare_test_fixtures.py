# Quick script to prepare test fixtures from JSON files.

import json
from base64 import b64encode
from pathlib import Path

ROOT_DIR = Path(__file__).parent.parent.absolute()

ignore_folders = ["target", ".venv", ".env", "env", "DOWNLOADS", "sandbox", ".vscode", ".github"]

for base_dir in ROOT_DIR.iterdir():
    if base_dir.is_dir() and base_dir.name not in ignore_folders:
        for json_file in base_dir.rglob("*.json"):
            try:
                data = json.loads(json_file.read_text())
            except json.decoder.JSONDecodeError:
                print(f"Error decoding {json_file}")
                continue
            # Encode
            encoded_file = json_file.parent / f"{json_file.stem}.tmfxture"
            encoded_file.write_bytes(b64encode(json.dumps(data).encode("utf-8")))
            # Redump original to be pretty
            json_file.write_text(json.dumps(data, indent=2))
            print(f"Prepared {json_file} -> {encoded_file}")
