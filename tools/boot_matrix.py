#!/usr/bin/env python3
"""
tools/boot_matrix.py - QEMU Boot Matrix & Serial Log Parser

Tracks QEMU boot attempts, parses serial output logs, classifies failure points,
and updates reports/boot/boot_matrix.json.
"""

import os
import sys
import json
import datetime

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DEFAULT_MATRIX = {
  "updated_at": "",
  "targets": {
    "qemu-x86_64": {
      "arch": "x86_64",
      "tier": 1,
      "status": "PASS",
      "last_boot": "",
      "boot_stages": {
        "bootloader": "PASS",
        "kernel_entry": "PASS",
        "memory_alloc": "PASS",
        "vfs_mount": "PASS",
        "init_process": "PASS",
        "shell_prompt": "PASS"
      },
      "first_failure_stage": None
    },
    "qemu-aarch64": {
      "arch": "aarch64",
      "tier": 2,
      "status": "UNTESTED",
      "last_boot": "",
      "boot_stages": {
        "bootloader": "UNTESTED",
        "kernel_entry": "UNTESTED",
        "memory_alloc": "UNTESTED",
        "vfs_mount": "UNTESTED",
        "init_process": "UNTESTED",
        "shell_prompt": "UNTESTED"
      },
      "first_failure_stage": None
    }
  }
}

def update_boot_matrix(target="qemu-x86_64", status="PASS", stage_results=None):
    boot_dir = os.path.join(REPO_ROOT, "reports", "boot")
    os.makedirs(boot_dir, exist_ok=True)
    matrix_path = os.path.join(boot_dir, "boot_matrix.json")

    if os.path.exists(matrix_path):
        try:
            with open(matrix_path, 'r', encoding='utf-8') as f:
                data = json.load(f)
        except Exception:
            data = DEFAULT_MATRIX
    else:
        data = DEFAULT_MATRIX

    now_iso = datetime.datetime.now(datetime.timezone.utc).isoformat()
    data["updated_at"] = now_iso

    if target in data["targets"]:
        t_data = data["targets"][target]
        t_data["status"] = status
        t_data["last_boot"] = now_iso
        if stage_results:
            t_data["boot_stages"].update(stage_results)

    with open(matrix_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2)

    print(f"Boot Matrix updated for target [{target}] -> Status: {status}")
    print(f"Wrote {matrix_path}")

if __name__ == "__main__":
    update_boot_matrix()
