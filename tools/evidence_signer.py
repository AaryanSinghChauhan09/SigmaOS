#!/usr/bin/env python3
"""
tools/evidence_signer.py - SHA256 Evidence Attestation Signer

Generates cryptographic verification digests (SHA256) for agent execution outputs,
build reports, and boot logs under reports/.
"""

import os
import sys
import json
import hashlib
import datetime

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def hash_file(filepath):
    h = hashlib.sha256()
    with open(filepath, 'rb') as f:
        while chunk := f.read(65536):
            h.update(chunk)
    return h.hexdigest()

def sign_reports():
    reports_dir = os.path.join(REPO_ROOT, "reports")
    manifest = {
        "signed_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "artifacts": {}
    }

    if os.path.exists(reports_dir):
        for root, dirs, files in os.walk(reports_dir):
            for f in files:
                if f == "evidence_signatures.json":
                    continue
                full_path = os.path.join(root, f)
                rel_path = os.path.relpath(full_path, REPO_ROOT)
                manifest["artifacts"][rel_path] = {
                    "sha256": hash_file(full_path),
                    "size_bytes": os.path.getsize(full_path)
                }

    out_path = os.path.join(reports_dir, "evidence_signatures.json")
    with open(out_path, 'w', encoding='utf-8') as f:
        json.dump(manifest, f, indent=2)

    print(f"Signed {len(manifest['artifacts'])} report artifacts.")
    print(f"Wrote SHA256 digest manifest to {out_path}")

if __name__ == "__main__":
    sign_reports()
