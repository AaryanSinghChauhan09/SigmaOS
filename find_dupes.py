import re
from collections import Counter

content = open("sigma_gui.py", encoding="utf-8").read()
matches = re.finditer(r"def (\w+)\(self", content)
methods = [m.group(1) for m in matches]
counts = Counter(methods)
dupes = {m: c for m, c in counts.items() if c > 1}
for d, c in dupes.items():
    locs = [m.start() for m in re.finditer(f"def {d}\\(self", content)]
    lines = [content.count('\n', 0, l) + 1 for l in locs]
    print(f"{d}: {c} times at lines {lines}")
