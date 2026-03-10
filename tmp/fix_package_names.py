import os

root = r"SIGMA_VIRTUAL_ROOT\Downloads\SigmaOS"
replacements = {
    "system_api": "system_api",
    "desktop_gui": "desktop_gui"
}

for dirpath, dirnames, filenames in os.walk(root):
    if ".git" in dirpath:
        continue
    for filename in filenames:
        if filename.endswith((".py", ".json", ".sh", ".ps1", ".txt", ".md", ".bat")):
            filepath = os.path.join(dirpath, filename)
            try:
                with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                
                new_content = content
                for old, new in replacements.items():
                    new_content = new_content.replace(old, new)
                
                if new_content != content:
                    with open(filepath, "w", encoding="utf-8") as f:
                        f.write(new_content)
                    print(f"Updated: {filepath}")
            except Exception as e:
                print(f"Error processing {filepath}: {e}")
