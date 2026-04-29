import os
import re

def fix_md030(directory):
    print(f"SIGMA [LINT]: Scouring {directory} for MD030 violations...")
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.md'):
                path = os.path.join(root, file)
                with open(path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Aggressive MD030 Fix: Remove ALL leading whitespace and force exactly 1 space after marker
                # Matches: (optional spaces)(marker)(multiple spaces)(non-space)
                # Group 1: marker (including digits.)
                # Group 2: actual content
                new_content = re.sub(r'^\s*([*1.-]|\d+\.)\s{2,}(\S)', r'\1 \2', content, flags=re.MULTILINE)
                
                # Also handle cases where there is only 1 space but leading whitespace exists
                new_content = re.sub(r'^\s+([*1.-]|\d+\.)\s(\S)', r'\1 \2', new_content, flags=re.MULTILINE)
                
                if new_content != content:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"[FIXED]: {path}")

if __name__ == "__main__":
    import sys
    target = sys.argv[1] if len(sys.argv) > 1 else "."
    fix_md030(target)
