import os
import re

ROOT = "."
SKIP_DIRS = {".git", "__pycache__", "node_modules", "evidence_vault", "SOVEREIGN_DISTRO_IMG", "artifacts"}

# Mappings for sanitization
MAPS = {
    r"\bGod-Mode\b": "Apex-Mode",
    r"\bGod Mode\b": "Apex Mode",
    r"\bHoly Trinity\b": "Standard Triad",
    r"\bhell\b": "chaos",
    r"\bHeaven\b": "Optimized State",
    r"\bAaryan\b": "SigmaUser",
    r"\bChauhan\b": "SigmaDeveloper",
    r"c:\\Users\\SigmaUser\\Downloads\\SigmaOS": r".", # Genericize paths
}

def final_sanitization():
    print("Starting Final Sanitization Pass...")
    count = 0
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith((".py", ".md", ".txt", ".html", ".css", ".js")):
                fp = os.path.join(root, file)
                try:
                    with open(fp, "r", encoding="utf-8", errors="replace") as f:
                        content = f.read()
                    
                    new_content = content
                    for pattern, replacement in MAPS.items():
                        new_content = re.sub(pattern, replacement, new_content, flags=re.IGNORECASE)
                    
                    if new_content != content:
                        with open(fp, "w", encoding="utf-8") as f:
                            f.write(new_content)
                        count += 1
                        print(f"  Sanitized: {fp}")
                except Exception as e:
                    print(f"  Error on {fp}: {e}")
    print(f"Total files sanitized this pass: {count}")

if __name__ == "__main__":
    final_sanitization()
