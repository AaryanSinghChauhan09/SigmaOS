import os
import re

ROOT = os.getcwd()

# Define common header directories and files in include/
INCLUDE_ROOT_FILES = [
    "sigma_kernel_types.h", "sigma_log.h", "sigma_hal.h", "SigmaOOP.hpp", 
    "SovereignLibC.h", "sigma_boot.h", "sigma_types.h", "sigma_syscall.h",
    "sigma_net.h", "sigma_fs.h", "sigma_vfs.h", "sigma_sched.h", "sigma_pqc.h",
    "sigma_armor.h", "sigma_audit.h", "sigma_iot.h", "sigma_gaming.h",
    "sigma_optimizer.h", "sigma_compliance.h", "sigma_kube.h", "sigma_regression.h"
]
INCLUDE_DIRS = ["core", "hal", "libc", "system", "fs", "net", "security", "ai", "boot", "drivers", "orchestration", "ui"]

def fix_includes(filepath):
    rel_path = os.path.relpath(filepath, ROOT)
    parts = rel_path.replace("\\", "/").split("/")
    depth = len(parts) - 1
    root_prefix = "../" * depth if depth > 0 else "./"

    with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
        lines = f.readlines()

    new_lines = []
    changed = False
    for line in lines:
        match = re.match(r'#include\s*["<]([^">]+)[">]', line)
        if match:
            inc_path = match.group(1)
            # Remove existing ../ or ./ prefixes to normalize
            clean_path = re.sub(r'^(\.\./)+', '', inc_path)
            clean_path = re.sub(r'^\./', '', clean_path)
            
            # Check if it starts with 'include/' or is one of our root include files
            base = clean_path.split('/')[0]
            if base == "include":
                new_inc = root_prefix + clean_path
                new_line = f'#include "{new_inc}"\n'
                if new_line != line:
                    line = new_line
                    changed = True
            elif base in INCLUDE_ROOT_FILES:
                new_inc = root_prefix + "include/" + clean_path
                new_line = f'#include "{new_inc}"\n'
                if new_line != line:
                    line = new_line
                    changed = True
            elif base in INCLUDE_DIRS:
                # If it's something like "core/sigma_types.h", map to "include/core/sigma_types.h"
                new_inc = root_prefix + "include/" + clean_path
                new_line = f'#include "{new_inc}"\n'
                if new_line != line:
                    line = new_line
                    changed = True
            elif "include/" in clean_path:
                # Catch cases like "../../include/sigma_log.h" -> normalized and depth corrected
                # clean_path will be "include/sigma_log.h"
                new_inc = root_prefix + clean_path
                new_line = f'#include "{new_inc}"\n'
                if new_line != line:
                    line = new_line
                    changed = True

        new_lines.append(line)

    if changed:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
        print(f"Fixed: {rel_path} (depth {depth})")

def main():
    for dirpath, _, filenames in os.walk(ROOT):
        if any(x in dirpath for x in [".git", "node_modules", "build", "obj", ".cache"]):
            continue
        for f in filenames:
            if f.endswith(('.cpp', '.h', '.hpp', '.c')):
                fix_includes(os.path.join(dirpath, f))

if __name__ == "__main__":
    main()
