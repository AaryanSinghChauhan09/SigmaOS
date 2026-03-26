"""
SigmaOS GUI Integrity Fixer
============================
1. Finds all self._build_XXX_page() calls in sigma_gui.py
2. Detects which ones are missing method definitions
3. Injects stub methods for any missing ones
4. Also scans for unknown PAL keys and FONT_ constants
"""

import re

with open("sigma_gui.py", "r", encoding="utf-8") as f:
    src = f.read()

# --- 1. Find all _build_*_page() calls (distinct) ---
calls = set(re.findall(r'self\.(_build_\w+_page)\(', src))
# Find all defined methods
defs  = set(re.findall(r'def (_build_\w+_page)\(', src))

missing = sorted(calls - defs)
print(f"Missing page builder stubs: {missing}")

# --- 2. Create stubs for missing ones ---
stub_block = "\n"
for m in missing:
    page_key = m.replace("_build_", "").replace("_page", "")
    stub_block += f"""
    def {m}(self):
        p = tk.Frame(self._content, bg=PAL["bg"])
        self._pages["{page_key}"] = p
        tk.Label(p, text="🚧 {page_key.replace('_', ' ').title()} — Coming Soon",
                 font=FONT_LOGO, fg=PAL["gold"], bg=PAL["bg"]).pack(expand=True, pady=80)
        tk.Label(p, text="This sovereign module is being prepared.",
                 font=FONT_MED, fg=PAL["dim"], bg=PAL["bg"]).pack()

"""

# --- 3. Inject stubs before the final `launch_gui` function ---
inject_marker = "\ndef launch_gui("
if inject_marker in src and missing:
    src = src.replace(inject_marker, stub_block + inject_marker)
    print(f"Injected {len(missing)} stubs.")
else:
    if not missing:
        print("No missing stubs needed.")
    else:
        print("WARNING: Could not find injection point.")

# --- 4. Find unknown PAL keys (heuristic scan) ---
used_keys  = set(re.findall(r'PAL\["(\w+)"\]', src))
known_keys = {"bg","bg2","bg3","accent","accent2","cyan","teal","gold","red","green",
              "text","dim","card","card_hover","border","blue","purple","orange","pink",
              "silver","nav_bg"}
unknown_pal = used_keys - known_keys
if unknown_pal:
    print(f"Unknown PAL keys (will be mapped to dim): {unknown_pal}")
    for k in unknown_pal:
        src = src.replace(f'PAL["{k}"]', 'PAL["dim"]')

# --- 5. Fix unknown FONT_ constants ---
used_fonts   = set(re.findall(r'(FONT_\w+)', src))
known_fonts  = {"FONT_MONO","FONT_SMALL","FONT_MED","FONT_BOLD","FONT_TITLE","FONT_LOGO"}
unknown_fonts = used_fonts - known_fonts
if unknown_fonts:
    print(f"Unknown FONT_ constants (will be mapped to FONT_SMALL): {unknown_fonts}")
    for f in unknown_fonts:
        src = src.replace(f, "FONT_SMALL")

with open("sigma_gui.py", "w", encoding="utf-8") as f:
    f.write(src)

print("\\nAll fixes applied. sigma_gui.py updated.")
