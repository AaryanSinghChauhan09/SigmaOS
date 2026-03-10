"""
Batch fix for common GUI constant name mismatches.
Fixes: FONT_CODE -> FONT_MONO, FONT_LABEL -> FONT_SMALL, FONT_HEAD -> FONT_TITLE
"""
REPLACEMENTS = [
    ("FONT_CODE",   "FONT_MONO"),
    ("FONT_LABEL",  "FONT_SMALL"),
    ("FONT_HEAD",   "FONT_TITLE"),
    ('PAL["fg"]',   'PAL["text"]'),
    ("PAL['fg']",   "PAL['text']"),
]

with open('sigma_gui.py', 'r', encoding='utf-8') as f:
    src = f.read()

for old, new in REPLACEMENTS:
    count = src.count(old)
    if count:
        print(f"  Replacing {count}x '{old}' -> '{new}'")
        src = src.replace(old, new)

with open('sigma_gui.py', 'w', encoding='utf-8') as f:
    f.write(src)

print("Done.")
