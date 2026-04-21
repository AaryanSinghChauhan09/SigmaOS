import os
import re

def check_file(path, pattern, message):
    if not os.path.exists(path):
        print(f"[FAIL] {path} not found.")
        return False
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
        if re.search(pattern, content, re.IGNORECASE):
            print(f"[PASS] {message}")
            return True
        else:
            print(f"[FAIL] {message}")
            return False

print("--- CI SIMULATION ---")
# Web.yml checks
check_file("web_ui/index.html", "fonts.googleapis.com", "Google Fonts in index.html")
check_file("web_ui/styles/style.css", "backdrop-filter", "backdrop-filter in style.css")
check_file("web_ui/styles/style.css", "-webkit-backdrop-filter", "-webkit-backdrop-filter in style.css")
check_file("web_ui/styles/style.css", "transition:", "transition in style.css")
check_file("web_ui/styles/style.css", "border-radius:", "border-radius in style.css")

# Harmony/Cleanliness checks
with open("web_ui/index.html", 'r', encoding='utf-8') as f:
    html = f.read()
    if 'style="' in html:
        print("[FAIL] Inline styles still exist in index.html.")
    else:
        print("[PASS] No inline styles in index.html.")

# CSS Prefix checks
with open("web_ui/styles/modules/windows.css", 'r', encoding='utf-8') as f:
    css = f.read()
    if '-webkit-user-select' in css:
        print("[PASS] -webkit-user-select present in windows.css")
    else:
        print("[FAIL] -webkit-user-select missing in windows.css")
