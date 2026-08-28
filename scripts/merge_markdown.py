#!/usr/bin/env python3
import sys
import os

def main():
    ours = sys.argv[1]
    base = sys.argv[2]
    theirs = sys.argv[3]
    marker = sys.argv[4]
    
    with open(ours, 'r') as f:
        ours_content = f.read()
    with open(theirs, 'r') as f:
        theirs_content = f.read()
    
    if not ours_content.strip():
        with open(ours, 'w') as f:
            f.write(theirs_content)
        return 0
    if not theirs_content.strip():
        return 0
    if ours_content == theirs_content:
        return 0
    
    # For markdown, prefer the version with more content
    if len(theirs_content) > len(ours_content):
        with open(ours, 'w') as f:
            f.write(theirs_content)
    return 0

if __name__ == "__main__":
    main()
