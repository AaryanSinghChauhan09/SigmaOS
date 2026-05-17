import os
import re

WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"

# 1. Fix include/sigma_kernel_types.h
kernel_types_path = os.path.join(WORKSPACE_DIR, "include", "sigma_kernel_types.h")
if os.path.exists(kernel_types_path):
    with open(kernel_types_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    
    # Remove it from the end
    content = re.sub(r'#define SIGMA_OK 0\n#define SIGMA_ERROR -1\n', '', content)
    content = re.sub(r'typedef enum \{[\s\S]*?\} sigma_boot_stage_t;\n', '', content)
    
    # Put it before #endif /* SIGMA_KERNEL_TYPES_H */
    if "SIGMA_BOOT_STAGE_INIT" not in content:
        injection = """
#define SIGMA_OK 0
#define SIGMA_ERROR -1

typedef enum {
    SIGMA_BOOT_STAGE_INIT = 0,
    SIGMA_BOOT_STAGE_KERNEL = 1,
    SIGMA_BOOT_STAGE_USERLAND = 2,
    SIGMA_BOOT_STAGE_RECOVERY = 3
} sigma_boot_stage_t;

"""
        content = content.replace("#endif /* SIGMA_KERNEL_TYPES_H */", injection + "\n#endif /* SIGMA_KERNEL_TYPES_H */")
    
    with open(kernel_types_path, "w", encoding="utf-8") as f:
        f.write(content)

# 2. Fix sigma_libc.h sigma_size_t etc
libc_path = os.path.join(WORKSPACE_DIR, "include", "libc", "SovereignLibC.h")
if os.path.exists(libc_path):
    with open(libc_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    if "sigma_ssize_t" not in content[:500]:
        content = content.replace("#define SOVEREIGN_LIBC_H", "#define SOVEREIGN_LIBC_H\n\n#include \"../sigma_kernel_types.h\"\n")
    with open(libc_path, "w", encoding="utf-8") as f:
        f.write(content)

# 3. Fix unused headers again
unused_headers = {
    r"kernel\\core\\SovereignArmor\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignCloud\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignCompliance\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignForensic\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignML\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignOrchestrator\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignRecovery\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignUserAccounts\.cpp": ["string.h"],
    r"kernel\\core\\system\\SovereignFS\.cpp": ["sigma_allocator.h"],
    r"tools\\sigma_fsck\.cpp": ["string"],
    r"tools\\telemetry-cli\.cpp": ["SigmaOOP.hpp"],
    r"include\\ui\\SovereignGUI\.h": ["sigma_types.h"],
    r"sigma_libc\.h": ["SovereignLibC.h"]
}

for root, dirs, files in os.walk(WORKSPACE_DIR):
    if "node_modules" in dirs:
        dirs.remove("node_modules")
    if ".git" in dirs:
        dirs.remove(".git")
    for file in files:
        file_path = os.path.join(root, file)
        for pattern, headers in unused_headers.items():
            if re.search(pattern, file_path):
                with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                for h in headers:
                    content = re.sub(rf'#include\s+["<].*{re.escape(h)}[">]\n', '', content)
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write(content)

# 4. Fix sigma_print in userland/SovereignAppStore.cpp
app_store = os.path.join(WORKSPACE_DIR, "userland", "SovereignAppStore.cpp")
if os.path.exists(app_store):
    with open(app_store, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    content = content.replace("sigma_print", "sigma_log_info")
    with open(app_store, "w", encoding="utf-8") as f:
        f.write(content)

# 5. Fix allocator_malloc in SovereignFS.cpp
sov_fs = os.path.join(WORKSPACE_DIR, "kernel", "core", "system", "SovereignFS.cpp")
if os.path.exists(sov_fs):
    with open(sov_fs, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    content = content.replace("allocator_malloc", "sigma_malloc")
    with open(sov_fs, "w", encoding="utf-8") as f:
        f.write(content)

# 6. Fix tools/sigma_vr_studio.cpp again
vr_studio_path = os.path.join(WORKSPACE_DIR, "tools", "sigma_vr_studio.cpp")
if os.path.exists(vr_studio_path):
    with open(vr_studio_path, "r", encoding="utf-8", errors="ignore") as f:
        content = f.read()
    if content.startswith("m\n") or content.startswith("m"):
        content = content.replace("m\n", "", 1)
        if content.startswith("m"):
            content = content[1:]
    with open(vr_studio_path, "w", encoding="utf-8") as f:
        f.write(content)

# 7. MD Formatting logic for remainder
def fix_markdown(filepath):
    with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
        lines = f.readlines()
    
    new_lines = []
    in_code_block = False
    for i, line in enumerate(lines):
        if line.startswith("```"):
            in_code_block = not in_code_block
        
        # MD036: emphasis used instead of heading -> replace **Text** with ### Text if on own line
        if not in_code_block and line.strip().startswith("**") and line.strip().endswith("**") and len(line.strip()) > 4:
            new_lines.append("### " + line.strip()[2:-2] + "\n")
            continue
            
        # MD037: space in emphasis
        if not in_code_block:
            line = re.sub(r'\*\*\s+(.*?)\s+\*\*', r'**\1**', line)
            line = re.sub(r'\*\s+(.*?)\s+\*', r'*\1*', line)
        
        # MD012: multiple blanks
        if not in_code_block and line.strip() == "" and len(new_lines) > 0 and new_lines[-1].strip() == "":
            continue
            
        new_lines.append(line)
        
    with open(filepath, "w", encoding="utf-8") as f:
        f.writelines(new_lines)

for root, dirs, files in os.walk(WORKSPACE_DIR):
    if "node_modules" in dirs:
        dirs.remove("node_modules")
    if ".git" in dirs:
        dirs.remove(".git")
    for file in files:
        if file.endswith(".md"):
            fix_markdown(os.path.join(root, file))

print("Automated Fixes Applied!")
