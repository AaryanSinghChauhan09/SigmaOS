#!/usr/bin/env python3
"""Fix SigmaOS compilation errors."""
import re, os, subprocess

REPO = "/home/aaryansinghchauhan/SigmaOS"

def read_file(path):
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        return f.read()

def write_file(path, content):
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f"  [FIXED] {path}")

# FIX 1: Add missing pub mod declarations in lib.rs
print("\n=== FIX 1: Add missing pub mod in lib.rs ===")
path = os.path.join(REPO, "src/lib.rs")
content = read_file(path)
missing_mods = []
for mod in ["distro", "drivers", "community", "governance"]:
    if f"pub mod {mod};" not in content and (f"pub use {mod}::" in content or f"use {mod}::" in content):
        missing_mods.append(mod)
if missing_mods:
    insert_after = "pub mod userland;"
    additions = "\n".join([f"pub mod {m};" for m in missing_mods])
    content = content.replace(insert_after, insert_after + "\n" + additions, 1)
    write_file(path, content)
    print(f"  Added: {missing_mods}")
else:
    print("  Already OK")

# FIX 2: Ensure drivers/mod.rs exists
print("\n=== FIX 2: Ensure drivers/mod.rs ===")
drivers_dir = os.path.join(REPO, "src/drivers")
mod_path = os.path.join(drivers_dir, "mod.rs")
if not os.path.exists(mod_path):
    rs_files = [f[:-3] for f in os.listdir(drivers_dir) if f.endswith('.rs') and f != 'mod.rs']
    content = "// SigmaOS Drivers Module\n"
    for f in sorted(rs_files):
        content += f"pub mod {f};\npub use {f}::*;\n"
    write_file(mod_path, content)
else:
    print("  Already exists")

# FIX 3: Fix selinux bare imports
print("\n=== FIX 3: Fix selinux bare imports ===")
result = subprocess.run(["grep", "-rn", "use selinux::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = content.replace("use selinux::", "use crate::security::selinux::")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 4: Fix vulnerability bare imports
print("\n=== FIX 4: Fix vulnerability bare imports ===")
result = subprocess.run(["grep", "-rn", "use vulnerability::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = content.replace("use vulnerability::", "use crate::security::vulnerability::")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 5: Fix linux_bsd_innovations bare imports
print("\n=== FIX 5: Fix linux_bsd_innovations imports ===")
result = subprocess.run(["grep", "-rn", "use linux_bsd_innovations::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = content.replace("use linux_bsd_innovations::", "use crate::kernel::linux_bsd_innovations::")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 6: Fix gui_wizard bare imports
print("\n=== FIX 6: Fix gui_wizard imports ===")
result = subprocess.run(["grep", "-rn", "use gui_wizard::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = content.replace("use gui_wizard::", "use crate::installer::gui_wizard::")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 7: Fix super::structures::ThreadState
print("\n=== FIX 7: Fix ThreadState import ===")
result = subprocess.run(["grep", "-rn", "super::structures::ThreadState", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = content.replace("super::structures::ThreadState", "crate::kernel::structures::ThreadState")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 8: Fix crate::klib::custom_string::SigmaString
print("\n=== FIX 8: Fix SigmaString import ===")
result = subprocess.run(["grep", "-rn", "crate::klib::custom_string::SigmaString", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = content.replace("crate::klib::custom_string::SigmaString", "crate::klib::SigmaString")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 9: Fix userland::shell::StreamTarget (add crate::)
print("\n=== FIX 9: Fix StreamTarget import ===")
result = subprocess.run(["grep", "-rn", "userland::shell::StreamTarget", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        # Only replace bare (not crate::userland) references
        new_content = re.sub(r'(?<!crate::)userland::shell::StreamTarget', 'crate::userland::shell::StreamTarget', content)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 10: Fix vfs:: bare imports  
print("\n=== FIX 10: Fix vfs imports ===")
result = subprocess.run(["grep", "-rn", "use vfs::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = re.sub(r'(?<!crate::filesystem::)(?<!crate::)use vfs::', 'use crate::filesystem::vfs::', content)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 11: Fix sigma_fs:: bare imports
print("\n=== FIX 11: Fix sigma_fs imports ===")
result = subprocess.run(["grep", "-rn", "use sigma_fs::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = re.sub(r'(?<!crate::filesystem::)use sigma_fs::', 'use crate::filesystem::sigma_fs::', content)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 12: Fix unimplemented_features:: bare imports
print("\n=== FIX 12: Fix unimplemented_features imports ===")
result = subprocess.run(["grep", "-rn", "use unimplemented_features::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = re.sub(r'(?<!crate::)use unimplemented_features::', 'use crate::unimplemented_features::', content)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 13: Fix security:: bare imports
print("\n=== FIX 13: Fix bare security:: imports ===")
result = subprocess.run(["grep", "-rn", r"^use security::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for line in result.stdout.strip().split('\n'):
    if line:
        filepath = line.split(':')[0]
        content = read_file(filepath)
        new_content = re.sub(r'^use security::', 'use crate::security::', content, flags=re.MULTILINE)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 14: Fix bare driver:: imports (not crate::driver::)
print("\n=== FIX 14: Fix bare driver:: imports ===")
result = subprocess.run(["grep", "-rln", r"^use driver::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for filepath in result.stdout.strip().split('\n'):
    if filepath:
        content = read_file(filepath)
        new_content = re.sub(r'^use driver::', 'use crate::driver::', content, flags=re.MULTILINE)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 15: Fix crate::klib::string imports (should be alloc::string)
print("\n=== FIX 15: Fix crate::klib::string imports ===")
result = subprocess.run(["grep", "-rln", "crate::klib::string", os.path.join(REPO, "src/")], capture_output=True, text=True)
for filepath in result.stdout.strip().split('\n'):
    if filepath:
        content = read_file(filepath)
        new_content = content.replace("use crate::klib::string::String", "use alloc::string::String")
        new_content = new_content.replace("use crate::klib::string", "use alloc::string")
        new_content = new_content.replace("crate::klib::string::", "alloc::string::")
        if new_content != content:
            write_file(filepath, new_content)

# FIX 16: Find sovereign_navigation_engine location and fix imports
print("\n=== FIX 16: Fix sovereign_navigation_engine imports ===")
find_result = subprocess.run(
    ["find", os.path.join(REPO, "src"), "-name", "sovereign_navigation_engine*"],
    capture_output=True, text=True
)
nav_files = find_result.stdout.strip().split('\n')
nav_mod_path = ""
for f in nav_files:
    if f.endswith('.rs'):
        nav_mod_path = f
        break

if nav_mod_path:
    # Determine crate path
    rel = os.path.relpath(nav_mod_path, os.path.join(REPO, "src"))
    mod_path = rel.replace('.rs', '').replace('/', '::')
    print(f"  Found at: {rel} => crate::{mod_path}")
    
    result = subprocess.run(["grep", "-rln", "use sovereign_navigation_engine::", os.path.join(REPO, "src/")], capture_output=True, text=True)
    for filepath in result.stdout.strip().split('\n'):
        if filepath:
            content = read_file(filepath)
            parent_dir = os.path.dirname(nav_mod_path)
            parent_rel = os.path.relpath(parent_dir, os.path.join(REPO, "src")).replace('/', '::')
            new_content = content.replace("use sovereign_navigation_engine::", f"use crate::{parent_rel}::sovereign_navigation_engine::")
            if new_content != content:
                write_file(filepath, new_content)
else:
    print("  sovereign_navigation_engine not found as .rs file!")
    result = subprocess.run(["grep", "-rln", "sovereign_navigation_engine", os.path.join(REPO, "src/")], capture_output=True, text=True)
    print(f"  Referenced in: {result.stdout[:500]}")

# FIX 17: Find developer_platform location and fix imports
print("\n=== FIX 17: Fix developer_platform imports ===")
find_result = subprocess.run(
    ["find", os.path.join(REPO, "src"), "-name", "developer_platform*"],
    capture_output=True, text=True
)
print(f"  Found: {find_result.stdout[:300]}")
result = subprocess.run(["grep", "-rln", "use developer_platform::", os.path.join(REPO, "src/")], capture_output=True, text=True)
for filepath in result.stdout.strip().split('\n'):
    if filepath:
        content = read_file(filepath)
        new_content = re.sub(r'(?<!crate::)use developer_platform::', 'use crate::developer_platform::', content)
        if new_content != content:
            write_file(filepath, new_content)

# FIX 18: Show where filesystem::FileMode is defined
print("\n=== FIX 18: FileMode investigation ===")
result = subprocess.run(["grep", "-rn", r"struct FileMode\|pub struct FileMode", os.path.join(REPO, "src/")], capture_output=True, text=True)
print(result.stdout[:500])

# FIX 19: Show duplicate function locations
print("\n=== FIX 19: Duplicate function locations ===")
for func in ["stage_offline_packages", "trigger_offline_update_on_reboot", "execute_pending_offline_update", "map_page", "bring_to_foreground"]:
    result = subprocess.run(["grep", "-rn", f"fn {func}", os.path.join(REPO, "src/")], capture_output=True, text=True)
    matches = [l for l in result.stdout.strip().split('\n') if l]
    if len(matches) > 1:
        print(f"  DUPE '{func}': {matches}")

print("\n=== ALL FIXES APPLIED ===")
