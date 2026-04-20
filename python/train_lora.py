#!/usr/bin/env python3
"""Minimal JSON-lines trainer stub for OmniForge scaffolding.

The production script will load transformers + PEFT; this version only emits metrics so the
Rust orchestrator can exercise stdout parsing end-to-end.
"""

from __future__ import annotations

import argparse
import json
import sys
import time


def main() -> None:
    parser = argparse.ArgumentParser(description="OmniForge LoRA training stub")
    parser.add_argument("--job-id", required=True)
    parser.add_argument("--dataset", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()

    # Pretend to read dataset path for operator visibility in logs.
    print(
        json.dumps(
            {
                "step": 0,
                "epoch": 0,
                "loss": 0.0,
                "lr": 2e-4,
                "note": f"stub warmup job={args.job_id} dataset={args.dataset}",
            }
        ),
        flush=True,
    )

    for step in range(1, 6):
        payload = {
            "step": step,
            "epoch": 1,
            "loss": 1.0 / float(step),
            "lr": 2e-4,
        }
        print(json.dumps(payload), flush=True)
        time.sleep(0.05)

    print(
        f"omniforge: adapter artifacts would land in {args.output}",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
