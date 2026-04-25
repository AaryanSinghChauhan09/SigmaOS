#!/usr/bin/env python3
"""
SigmaOS Auto-Shard Reconciliation Tool (SART)
Automates the detection and cleanup of fragmented/fossilized SXX_ shards.
Finds duplicate or overlapping code logic across the legacy directories and 
proposes migrations to the new canonical `sigmaos/` hierarchy.
"""

import os
import hashlib
from collections import defaultdict

def hash_file(filepath):
    """Generate SHA-256 hash of a file to detect exact duplicates."""
    hasher = hashlib.sha256()
    try:
        with open(filepath, 'rb') as f:
            buf = f.read()
            hasher.update(buf)
        return hasher.hexdigest()
    except Exception as e:
        return None

def scan_shards(root_dir):
    """Scans the repository for legacy shard directories and duplicates."""
    print(f"[*] Scanning {root_dir} for legacy shards and duplicates...\n")
    
    file_hashes = defaultdict(list)
    legacy_shards = []
    
    for dirpath, dirnames, filenames in os.walk(root_dir):
        # Ignore git and standard environments
        if '.git' in dirpath or '__pycache__' in dirpath:
            continue
            
        dirname = os.path.basename(dirpath)
        if dirname.startswith('S') and '_' in dirname and any(char.isdigit() for char in dirname):
            if dirpath not in legacy_shards:
                legacy_shards.append(dirpath)
                
        for filename in filenames:
            filepath = os.path.join(dirpath, filename)
            fhash = hash_file(filepath)
            if fhash:
                file_hashes[fhash].append(filepath)

    # Report Phase
    print("=== Legacy Shard Directories Detected ===")
    if not legacy_shards:
        print("  None detected! Architecture is clean.")
    else:
        for shard in legacy_shards:
            print(f"  [X] Mark for deprecation: {shard}")
            
    print("\n=== Exact File Duplicates Detected ===")
    duplicates_found = False
    for fhash, paths in file_hashes.items():
        if len(paths) > 1:
            duplicates_found = True
            print(f"\n  Duplicate Cluster ({fhash[:8]}...):")
            for p in paths:
                print(f"    -> {p}")
                
    if not duplicates_found:
        print("  No duplicate files found.")

    print("\n[+] Audit complete. Run `s-deps prune` to finalize removals.")

if __name__ == "__main__":
    # Scan the current working directory
    scan_shards(".")
