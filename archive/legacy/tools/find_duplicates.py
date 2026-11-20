#!/usr/bin/env python3
"""
Analyze cargo check output to find all duplicate definitions
and their locations for systematic fixing.
"""
import re
import subprocess
from collections import defaultdict

def main():
    # Run cargo check and capture output
    result = subprocess.run(
        ['cargo', 'check', '--lib'],
        cwd='/home/aaryansinghchauhan/SigmaOS',
        capture_output=True,
        text=True
    )
    
    output = result.stderr
    
    # Parse duplicates
    duplicates = defaultdict(list)
    current_name = None
    current_file = None
    current_line = None
    
    for line in output.split('\n'):
        # Match "the name `X` is defined multiple times"
        name_match = re.search(r"the name `([^`]+)` is defined multiple times", line)
        if name_match:
            current_name = name_match.group(1)
        
        # Match file location "--> src/path/file.rs:123:1"
        loc_match = re.search(r"--> (src/[^:]+):(\d+):\d+", line)
        if loc_match and current_name:
            filepath = loc_match.group(1)
            lineno = int(loc_match.group(2))
            duplicates[current_name].append((filepath, lineno))
    
    # Print summary
    print(f"Found {len(duplicates)} duplicate names across codebase\n")
    
    # Group by file
    by_file = defaultdict(list)
    for name, locations in duplicates.items():
        for filepath, lineno in locations:
            by_file[filepath].append((name, lineno))
    
    # Print file-by-file breakdown
    for filepath in sorted(by_file.keys()):
        items = by_file[filepath]
        print(f"\n{filepath} ({len(items)} duplicates):")
        for name, lineno in sorted(items, key=lambda x: x[1]):
            print(f"  Line {lineno}: {name}")
    
    # Print files with most duplicates
    print("\n\nFiles with most duplicates:")
    sorted_files = sorted(by_file.items(), key=lambda x: len(x[1]), reverse=True)
    for filepath, items in sorted_files[:10]:
        print(f"  {filepath}: {len(items)} duplicates")

if __name__ == '__main__':
    main()
