import os
import re

def fix_markdown(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    new_lines = []
    for i, line in enumerate(lines):
        # MD009: Trailing spaces
        line = line.rstrip() + '\n'
        
        # MD026: Trailing punctuation in headings
        if line.startswith('#'):
            line = re.sub(r'([:.?!])\n$', r'\n', line)
            
        # MD022: Blanks around headings
        if line.startswith('#'):
            if new_lines and new_lines[-1].strip():
                new_lines.append('\n')
            new_lines.append(line)
            if i + 1 < len(lines) and lines[i+1].strip():
                new_lines.append('\n')
            continue

        # MD032: Blanks around lists
        if line.lstrip().startswith(('* ', '- ', '1. ')):
            if new_lines and not new_lines[-1].startswith(('*', '-', '1.', ' ', '\t')) and new_lines[-1].strip():
                new_lines.append('\n')
            new_lines.append(line)
            if i + 1 < len(lines) and not lines[i+1].lstrip().startswith(('*', '-', '1.', ' ', '\t')) and lines[i+1].strip():
                new_lines.append('\n')
            continue

        new_lines.append(line)

    # MD047: Single trailing newline
    content = "".join(new_lines).rstrip() + '\n'

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

def walk_and_fix(root):
    for dirpath, _, filenames in os.walk(root):
        if '.git' in dirpath: continue
        for f in filenames:
            if f.endswith('.md'):
                fix_markdown(os.path.join(dirpath, f))

if __name__ == "__main__":
    walk_and_fix(".")
