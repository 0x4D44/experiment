#!/usr/bin/env python3
"""Copy HLIL exports into f1gp-port/docs/re."""

from __future__ import annotations

import argparse
import shutil
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
HLIL_SRC = REPO_ROOT / "re" / "artifacts" / "hlil"
HLIL_DST = REPO_ROOT / "f1gp-port" / "docs" / "re"


def main():
    parser = argparse.ArgumentParser(description="Publish HLIL exports")
    parser.add_argument("--clean", action="store_true", help="Remove destination files before copying")
    args = parser.parse_args()

    if not HLIL_SRC.exists():
        print("No HLIL artifacts to publish", flush=True)
        return
    HLIL_DST.mkdir(parents=True, exist_ok=True)
    if args.clean:
        for item in HLIL_DST.glob("*.c"):
            item.unlink()
    copied = 0
    for artifact in HLIL_SRC.glob("*.c"):
        shutil.copy2(artifact, HLIL_DST / artifact.name)
        copied += 1
    print(f"Copied {copied} HLIL files into {HLIL_DST}")


if __name__ == "__main__":
    main()
