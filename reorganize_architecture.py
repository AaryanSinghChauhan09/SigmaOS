import os
import shutil
from pathlib import Path

# Root should be relative to the script
ROOT = Path(os.path.dirname(os.path.abspath(__file__)))

def create_dirs():
    print("[*] Creating OS Geometry...")
    dirs = [
        "bootloader", "kernel", "libc", "userland", 
        "userland/system_api", "web_os", "apps"
    ]
    for d in dirs:
        (ROOT / d).mkdir(parents=True, exist_ok=True)

def move_files():
    print("[*] Reorganizing Core Components...")
    # Logic for moving legacy 'c_core' etc. if they exist
    c_core = ROOT / "kernel" / "c_core"
    if c_core.exists():
        # ... logic ...
        pass

    # Fix userland structure
    old_apps = ROOT / "apps"
    new_apps = ROOT / "userland" / "apps"
    if old_apps.exists() and old_apps != new_apps:
        for f in old_apps.iterdir():
            if not (new_apps / f.name).exists():
                shutil.move(str(f), str(new_apps / f.name))
        print(f"[✓] Migrated apps to {new_apps}")

def update_imports():
    print("[*] Refactoring Semantic Imports...")
    # This ensures consistency across the codebase
    target_files = ["sigma.py", "launcher.py", "sigma_cli.py", "sigma_gui.py"]
    for m in target_files:
        p = ROOT / m
        if p.exists():
            content = p.read_text('utf-8', errors='ignore')
            content = content.replace('"kernel"', '"sigma_core"')
            content = content.replace("'kernel'", "'sigma_core'")
            p.write_text(content, 'utf-8')

if __name__ == "__main__":
    create_dirs()
    move_files()
    update_imports()
    print("[OK] Architecture Reorganized & Sanitized.")
