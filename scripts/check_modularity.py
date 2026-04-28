import os
import sys
import re

def check_modularity(directory):
    errors = 0
    # Pattern to detect raw parent-directory includes (un-sharded access)
    unsafe_include_pattern = re.compile(r'#include\s+"\.\./.*"')
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(('.c', '.h', '.cpp', '.hpp')):
                path = os.path.join(root, file)
                with open(path, 'r', errors='ignore') as f:
                    for i, line in enumerate(f, 1):
                        if unsafe_include_pattern.search(line):
                            print(f"[ERROR]: Un-sharded access detected in {path}:{i} -> {line.strip()}")
                            errors += 1
    return errors

if __name__ == "__main__":
    base_dir = "."
    print(f"Σ [SENTINEL]: Auditing shard boundaries in {os.path.abspath(base_dir)}...")
    error_count = check_modularity(base_dir)
    if error_count > 0:
        print(f"[FAIL]: Found {error_count} modularity violations.")
        sys.exit(1)
    print("[PASS]: Shard boundaries verified.")
    sys.exit(0)
