import os
import re

WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"

# 1. Remove duplicate enum from sigma_boot.h
sigma_boot_path = os.path.join(WORKSPACE_DIR, "include", "sigma_boot.h")
if os.path.exists(sigma_boot_path):
    with open(sigma_boot_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    
    # Strip out the typedef enum
    content = re.sub(r'typedef enum \{[\s\S]*?\} sigma_boot_stage_t;\n', '', content)
    
    with open(sigma_boot_path, "w", encoding="utf-8") as f:
        f.write(content)

# 2. Fix sigma_types.h duplicate SIGMA_OK
sigma_types_path = os.path.join(WORKSPACE_DIR, "include", "core", "sigma_types.h")
if os.path.exists(sigma_types_path):
    with open(sigma_types_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    
    content = re.sub(r'#define SIGMA_OK 0\n', '', content)
    content = re.sub(r'#define SIGMA_ERROR -1\n', '', content)
    
    with open(sigma_types_path, "w", encoding="utf-8") as f:
        f.write(content)

print("Duplicates eradicated!")
