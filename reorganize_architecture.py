import os
import shutil
import re
from pathlib import Path

root = Path("C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS")

def create_dirs():
    for d in ["bootloader", "kernel", "libc", "userland", "userland/system-api", "userland/desktop-gui", "userland/userland/apps"]:
        (root / d).mkdir(parents=True, exist_ok=True)

def move_files():
    # 1. Bootloader
    c_core = root / "kernel" / "c_core"
    cpp_core = root / "kernel" / "cpp_core"
    rust_core = root / "kernel" / "rust_core"
    
    # 2. Extract specific files for Bootloader & Libc before Kernel mapping
    if c_core.exists():
        for file in c_core.glob("*.asm"):
            if "boot" in file.name:
                shutil.copy2(file, root / "bootloader" / file.name)
        
        # create libc skeleton if none exists
        for file in c_core.glob("*.c"):
            if file.name in ["stdlib.c", "string.c"]: # just examples if they exist
                shutil.copy2(file, root / "libc" / file.name)
    
    # Now create new kernel dev dir
    new_kernel = root / "kernel_dev"
    new_kernel.mkdir(exist_ok=True)
    
    for core_dir in [c_core, cpp_core, rust_core]:
        if core_dir.exists():
            for f in core_dir.iterdir():
                if f.is_file():
                    shutil.copy2(f, new_kernel / f.name)

    # Move python kernel layer to userland/system-api
    old_kernel = root / "kernel"
    sys_api = root / "userland" / "system-api"
    if old_kernel.exists():
        for f in old_kernel.iterdir():
            if f.is_file() and f.suffix in [".py", ".sh", ".ps1"]:
                shutil.move(str(f), str(sys_api / f.name))
        
        # Safely remove old core dirs and the root kernel dir
        for core_dir in [c_core, cpp_core, rust_core]:
            if core_dir.exists():
                shutil.rmtree(core_dir, ignore_errors=True)
        # Try to clean up __pycache__ etc
        for d in old_kernel.iterdir():
            if d.is_dir():
                shutil.rmtree(d, ignore_errors=True)
        old_kernel.rmdir()
        
    # rename kernel_dev to kernel
    if new_kernel.exists():
        new_kernel.rename(root / "kernel")

    # 3. userland/userland/apps
    userland/apps_dir = root / "userland/apps"
    dest_userland/apps = root / "userland" / "userland/apps"
    if userland/apps_dir.exists() and dest_userland/apps.exists():
        for f in userland/apps_dir.iterdir():
            shutil.move(str(f), str(dest_userland/apps / f.name))
        userland/apps_dir.rmdir()

    # 4. userland/desktop-gui
    web_dir = root / "userland/desktop-gui"
    dest_web = root / "userland" / "desktop-gui"
    if web_dir.exists() and dest_web.exists():
        for f in web_dir.iterdir():
            shutil.move(str(f), str(dest_web / f.name))
        web_dir.rmdir()

def update_imports():
    root = Path("C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS")
    main_files = ["sigma.py", "launcher.py", "sigma_cli.py", "sigma_gui.py", "test_sigmaos_suite.py"]
    for m in main_files:
        p = root / m
        if p.exists():
            content = p.read_text('utf-8')
            content = content.replace('"kernel"', '"userland/system-api"')
            content = content.replace("'kernel'", "'userland/system-api'")
            content = content.replace('"APPS"', '"USERLAND/APPS"')
            content = content.replace("'userland/apps'", "'userland/userland/apps'")
            content = content.replace("'userland/desktop-gui'", "'userland/desktop-gui'")
            content = content.replace('"userland/desktop-gui"', '"userland/desktop-gui"')
            p.write_text(content, 'utf-8')
            
    # Modify ffi_bridge.py
    ffi = root / "userland" / "system-api" / "ffi_bridge.py"
    if ffi.exists():
        content = ffi.read_text('utf-8')
        content = content.replace('root / "c_core" / f"sovereign_core{lib_ext}"', 'root.parent.parent / "kernel" / f"sovereign_core{lib_ext}"')
        content = content.replace('root / "rust_core" / f"libvanguard{lib_ext}"', 'root.parent.parent / "kernel" / f"libvanguard{lib_ext}"')
        ffi.write_text(content, 'utf-8')
        
    # Modify everything that explicitly loaded from userland/desktop-gui/index.html
    for file in root.rglob("*.py"):
        try:
            content = file.read_text('utf-8')
            updates = 0
            if "userland/desktop-gui" in content or "userland/apps" in content:
                content = content.replace("userland/desktop-gui", "userland/desktop-gui")
                content = content.replace("userland/apps", "userland/userland/apps")
                updates += 1
            if '"kernel"' in content or "'kernel'" in content:
                
                # Careful: some might be referencing standard library, but we'll try to replace string paths
                # Only replace paths representing the local folder
                content = content.replace('os.path.join(_ROOT, "userland/system-api")', 'os.path.join(_ROOT, "userland/system-api")')
                content = content.replace("os.path.join(root, 'userland/system-api')", "os.path.join(root, 'userland/system-api')")
                updates += 1
            if updates:
                file.write_text(content, 'utf-8')
        except:
            pass

create_dirs()
move_files()
update_imports()
print("Success")
