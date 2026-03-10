import re
import os
from collections import defaultdict

# Root should be relative to the script
ROOT = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(ROOT, 'sigma_gui.py')
pattern = re.compile(r'^\s*def\s+(_build_\w+_page)\s*\(self\):', re.MULTILINE)

if os.path.exists(file_path):
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
            print(f"DUPLICATE UI PAGE: {name} at lines {lines}")
else:
    print(f"[!] Target UI file not found: {file_path}")
