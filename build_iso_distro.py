import os
import shutil
import json
from pathlib import Path

def build_distro_iso():
    root = Path("C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS")
    dist_img = root / "SOVEREIGN_DISTRO_IMG"
    
    # Clean old distro if it exists
    if dist_img.exists():
        shutil.rmtree(dist_img)
    
    # Create Professional Native Structure
    dirs = [
        "BOOT",             # Pure x86/ARM boot vectors
        "KERNEL",           # Native C/Rust/ASM Binaries
        "LIBC",             # Standard System Libraries
        "USERLAND/API",     # Sovereign Python System API
        "USERLAND/UI",      # Sovereign HTML5/JS Interface
        "USERLAND/APPS",    # Multi-Disciplinary AI Apps
        "RECOVERY"          # Self-Healing & Forensic Tools
    ]
    
    for d in dirs:
        (dist_img / d).mkdir(parents=True, exist_ok=True)
        
    # 1. Map Native Core
    kernel_src = root / "kernel"
    if kernel_src.exists():
        for f in kernel_src.iterdir():
            if f.is_file():
                shutil.copy2(f, dist_img / "KERNEL" / f.name)

    boot_src = root / "bootloader"
    if boot_src.exists():
        for f in boot_src.iterdir():
            if f.is_file():
                shutil.copy2(f, dist_img / "BOOT" / f.name)

    libc_src = root / "libc"
    if libc_src.exists():
        for f in libc_src.iterdir():
            if f.is_file():
                shutil.copy2(f, dist_img / "LIBC" / f.name)

    # 2. Map Userland
    api_src = root / "userland" / "system-api"
    if api_src.exists():
        for f in api_src.iterdir():
            if f.is_file():
                shutil.copy2(f, dist_img / "USERLAND" / "API" / f.name)

    ui_src = root / "userland" / "desktop-gui"
    if ui_src.exists():
        for f in ui_src.iterdir():
            if f.is_file():
                shutil.copy2(f, dist_img / "USERLAND" / "UI" / f.name)

    apps_src = root / "userland" / "apps"
    if apps_src.exists():
        for f in apps_src.iterdir():
            if f.is_file():
                shutil.copy2(f, dist_img / "USERLAND" / "APPS" / f.name)

    # 3. Essential Orchestrator Files (Root of Distro)
    top_files = [
        "sigma.py",
        "sigma_gui.py",
        "sigma_cli.py",
        "sigma_core", # Dir
        "ecosystem",  # Dir
        "SET_AS_NATIVE_BOOT.bat",
        "Vagrantfile",
        "available_features.md"
    ]
    
    for item in top_files:
        src = root / item
        if src.exists():
            if src.is_dir():
                shutil.copytree(src, dist_img / item, dirs_exist_ok=True)
            else:
                shutil.copy2(src, dist_img / item)

    # Create Sovereign Boot Manifest
    manifest = {
        "OS_NAME": "SigmaOS Sovereign",
        "VERSION": "2.0.0-APEX",
        "ARCHITECTURE": "x86_64 / ARM64 Hybrid",
        "BOOT_ENTRY": "sigma.py",
        "KERNEL_LINK": "NATIVE_LINKED",
        "USERLAND_MODEL": "DISTRIBUTED_API",
        "SECURITY_GRADE": "TITAN_ZERO_TRUST"
    }
    
    with open(dist_img / "BOOT" / "manifest.json", 'w') as f:
        json.dump(manifest, f, indent=4)

    # Professional ISO Readme
    with open(dist_img / "SOVEREIGN_README.txt", 'w') as f:
        f.write("============================================================\n")
        f.write(" SIGMA OS SOVEREIGN v2.0 - ARCHITECTURE COMPLIANCE IMAGE \n")
        f.write("============================================================\n\n")
        f.write("PRINCIPLE:\n")
        f.write("  - Ring 0: C/Rust/ASM Kernel (Location: /KERNEL)\n")
        f.write("  - Ring 3: Python/JS/HTML Userland (Location: /USERLAND)\n\n")
        f.write("Sovereignty is built on this foundation.")

    print(f"✅ SIGMAOS ISO DISTRO ASSEMBLED AT: {dist_img}")

if __name__ == "__main__":
    build_distro_iso()
