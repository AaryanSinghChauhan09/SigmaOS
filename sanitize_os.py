import os
import sys
from pathlib import Path

# Paths to sanitize
SIGMA_ROOT = Path(os.getcwd())

# Replacement logic (simple string replaces for safety)
REPLACEMENTS = {
    # Full paths
    "C:\\Users\\Sovereign-User": os.environ.get("USERPROFILE", "C:\\Users\\User"),
    "c:\\Users\\Sovereign-User": os.environ.get("USERPROFILE", "C:\\Users\\User"),
    "C:/Users/Aaryan": os.environ.get("USERPROFILE", "C:/Users/Aaryan").replace("\\", "/"),
    "C:/Users/Aaryan": os.environ.get("USERPROFILE", "C:/Users/Aaryan").replace("\\", "/"),
    
    # Identifiers
    "O-Sovereign": "O-Sovereign",
    "Sovereign-User": "Sovereign-User",
}

def sanitize_file(file_path):
    try:
        # Open with explicit UTF-8 and errors ignore to avoid codec crashes
        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            
        modified = content
        for find_str, replace_str in REPLACEMENTS.items():
            if find_str in modified:
                modified = modified.replace(find_str, replace_str)
            
        if modified != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(modified)
            print(f"[RECOVER] Sanitized: {os.path.basename(file_path)}")
            return True
    except Exception as e:
        pass
    return False

def main():
    # Use ascii fallback for progress bars/symbols to avoid Windows terminal issues
    print(f"[*] Starting OS Sanitization loop in {SIGMA_ROOT}...")
    count = 0
    for root, dirs, files in os.walk(SIGMA_ROOT):
        # Skip git and cache
        if any(x in root for x in ('.git', '__pycache__', '.pytest_cache')):
            continue
            
        for file in files:
            if file.endswith(('.py', '.md', '.txt', '.ps1', '.bat', '.json', '.xml')):
                if sanitize_file(os.path.join(root, file)):
                    count += 1
    print(f"DONE - Sanitization complete. {count} files processed.")

if __name__ == "__main__":
    main()
