import os
import ast

ROOT = os.getcwd()

def heal_recursive(path):
    for root, dirs, files in os.walk(path):
        for f in files:
            if f.endswith('.py'):
                fp = os.path.join(root, f)
                try:
                    with open(fp, 'r', encoding='utf-8', errors='replace') as file:
                        src = file.read()
                    
                    # Basic check for empty blocks that cause IndentationError
                    # e.g. "class X:\n\ndef f()..."
                    lines = src.splitlines()
                    modified = False
                    new_lines = []
                    for i, line in enumerate(lines):
                        new_lines.append(line)
                        if line.strip().endswith(':') and (i == len(lines)-1 or not lines[i+1].strip() or not lines[i+1].startswith((' ', '\t'))):
                            # Check it's not followed by another indented line further down
                            has_body = False
                            for j in range(i+1, min(i+5, len(lines))):
                                if lines[j].strip() and lines[j].startswith((' ', '\t')):
                                    has_body = True
                                    break
                            if not has_body:
                                new_lines.append('    pass')
                                modified = True
                    
                    if modified:
                        with open(fp, 'w', encoding='utf-8') as file:
                            file.write('\n'.join(new_lines))
                except:
                    pass

if __name__ == "__main__":
    print("Running Standalone Integrity Healer...")
    heal_recursive(ROOT)
    print("Heal Complete.")
