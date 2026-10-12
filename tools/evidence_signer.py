#!/usr/bin/env python3
"""
<<<<<<< HEAD
SigmaOS Evidence Signer Tool
Generates SHA256 checksum manifests and attestation metadata for reports and evidence bundles.
"""

import argparse
import datetime
import hashlib
import json
import os


def generate_sha256(filepath):
    h = hashlib.sha256()
    with open(filepath, "rb") as f:
        while chunk := f.read(8192):
            h.update(chunk)
    return h.hexdigest()


def sign_evidence_directory(reports_dir="reports"):
    if not os.path.exists(reports_dir):
        print(f"Directory {reports_dir} does not exist.")
        return

    manifest = {
        "timestamp": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "signer": "SigmaOS Evidence Signer v1.0.0",
        "artifacts": {}
    }

    for root, _, files in os.walk(reports_dir):
        for file in sorted(files):
            if file == "evidence_manifest.json":
                continue
            fpath = os.path.join(root, file)
            rel_path = os.path.relpath(fpath, reports_dir)
            sha256_hash = generate_sha256(fpath)
            manifest["artifacts"][rel_path] = {
                "sha256": sha256_hash,
                "size_bytes": os.path.getsize(fpath)
            }

    manifest_path = os.path.join(reports_dir, "evidence_manifest.json")
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"Evidence manifest generated at {manifest_path} with {len(manifest['artifacts'])} signed artifacts.")


def main():
    parser = argparse.ArgumentParser(description="SigmaOS Evidence Signer")
    parser.add_argument("--dir", default="reports", help="Reports directory to sign")
    args = parser.parse_args()

    sign_evidence_directory(args.dir)


if __name__ == "__main__":
    main()
=======
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
>>>>>>> origin/main
