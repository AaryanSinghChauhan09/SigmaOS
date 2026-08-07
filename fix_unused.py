import re
import os
import sys

def replace_in_file(path, line_num, target, replacement):
    if not os.path.exists(path):
        print(f"File {path} not found")
        return
    with open(path, 'r') as f:
        lines = f.readlines()
    
    idx = line_num - 1
    if 0 <= idx < len(lines):
        if target in lines[idx]:
            lines[idx] = lines[idx].replace(target, replacement)
            with open(path, 'w') as f:
                f.writelines(lines)
            print(f"Fixed {path}:{line_num}")
        else:
            print(f"Target '{target}' not found in {path}:{line_num}. Line is: {lines[idx].strip()}")
    else:
        print(f"Line {line_num} out of bounds in {path}")

# Add manual replacements here
replacements = [
    ("src/fs/filesystem.rs", 543, "handle: FileHandle", "_handle: FileHandle"),
    ("src/fs/filesystem.rs", 551, "handle: FileHandle", "_handle: FileHandle"),
]

for p, l, t, r in replacements:
    replace_in_file(p, l, t, r)
