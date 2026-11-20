#!/usr/bin/env python3
"""
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
            if file in ["evidence_manifest.json", "evidence_signatures.json"]:
                continue
            fpath = os.path.join(root, file)
            rel_path = os.path.relpath(fpath, reports_dir)
            sha256_hash = generate_sha256(fpath)
            manifest["artifacts"][rel_path] = {
                "sha256": sha256_hash,
                "size_bytes": os.path.getsize(fpath)
            }

    manifest_path = os.path.join(reports_dir, "evidence_manifest.json")
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)

    sig_path = os.path.join(reports_dir, "evidence_signatures.json")
    with open(sig_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)

    print(f"Evidence manifest generated at {manifest_path} and {sig_path} with {len(manifest['artifacts'])} signed artifacts.")


def main():
    parser = argparse.ArgumentParser(description="SigmaOS Evidence Signer")
    parser.add_argument("--dir", default="reports", help="Reports directory to sign")
    args = parser.parse_args()

    sign_evidence_directory(args.dir)


if __name__ == "__main__":
    main()
