import os

def heal_dir(p):
    for r, d, fs in os.walk(p):
        if '.git' in r: continue
        for f in fs:
            if f.endswith('.py'):
                path = os.path.join(r, f)
                try:
                    with open(path, 'r', encoding='utf-8', errors='replace') as file:
                        lines = file.readlines()
                    modified = False
                    new_lines = []
                    for i, line in enumerate(lines):
                        new_lines.append(line)
                        stripped = line.strip()
                        if stripped.endswith(':') and (stripped.startswith(('def ', 'class ', 'async def '))):
                            # Potential empty block
                            has_body = False
                            if i + 1 < len(lines):
                                for j in range(i + 1, min(i + 5, len(lines))):
                                    if lines[j].strip():
                                        if lines[j].startswith((' ', '\t')):
                                            has_body = True
                                        break
                            if not has_body:
                                new_lines.append('    pass\n')
                                modified = True
                    if modified:
                        with open(path, 'w', encoding='utf-8') as file:
                            file.writelines(new_lines)
                except:
                    pass

if __name__ == "__main__":
    print("Fixing all shard indentations...")
    heal_dir(".")
    print("Healing complete.")
