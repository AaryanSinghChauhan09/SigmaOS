
import re
from collections import defaultdict

file_path = r'C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\sigma_gui.py'
pattern = re.compile(r'^\s*def\s+(_build_\w+_page)\s*\(self\):', re.MULTILINE)

with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
    content = f.read()

matches = pattern.finditer(content)
func_map = defaultdict(list)

for match in matches:
    name = match.group(1)
    line_no = content.count('\n', 0, match.start()) + 1
    func_map[name].append(line_no)

for name, lines in func_map.items():
    if len(lines) > 1:
        print(f"DUPLICATE: {name} at lines {lines}")
    else:
        # print(f"UNIQUE: {name} at line {lines[0]}")
        pass
