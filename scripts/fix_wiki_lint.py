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
                
                # Fix list markers: *   or 1.   or -   to exactly 1 space
                # Regex: ^(\s*[*1.-])\s{2,}(\S)
                # We use re.MULTILINE
                new_content = re.sub(r'^(\s*[*1.-]|\s*\d+\.)\s{2,}(\S)', r'\1 \2', content, flags=re.MULTILINE)
                
                if new_content != content:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"[FIXED]: {path}")

if __name__ == "__main__":
    import sys
    target = sys.argv[1] if len(sys.argv) > 1 else "."
    fix_md030(target)
