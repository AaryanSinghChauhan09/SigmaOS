import os
import re

# Workspace paths
WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
WIKI_DIR = os.path.join(WORKSPACE_DIR, "wiki_repo")

# Fix 1: Add SIGMA_OK / SIGMA_ERROR and sigma_boot_stage_t to sigma_kernel_types.h if missing
kernel_types_path = os.path.join(WORKSPACE_DIR, "include", "sigma_kernel_types.h")
if os.path.exists(kernel_types_path):
    with open(kernel_types_path, "r", encoding="utf-8") as f:
        content = f.read()
    
    if "SIGMA_OK" not in content:
        content += "\n#define SIGMA_OK 0\n#define SIGMA_ERROR -1\n"
    
    if "sigma_boot_stage_t" not in content:
        content += """
typedef enum {
    SIGMA_BOOT_STAGE_INIT = 0,
    SIGMA_BOOT_STAGE_KERNEL = 1,
    SIGMA_BOOT_STAGE_USERLAND = 2,
    SIGMA_BOOT_STAGE_RECOVERY = 3
} sigma_boot_stage_t;
"""
    with open(kernel_types_path, "w", encoding="utf-8") as f:
        f.write(content)


# Fix 2: Remove 'inline' from operator new in SigmaOOP.hpp
sigma_oop_path = os.path.join(WORKSPACE_DIR, "include", "SigmaOOP.hpp")
if os.path.exists(sigma_oop_path):
    with open(sigma_oop_path, "r", encoding="utf-8") as f:
        content = f.read()
    content = re.sub(r'inline\s+void\*\s+operator\s+new', r'void* operator new', content)
    content = re.sub(r'inline\s+void\*\s+operator\s+new\[\]', r'void* operator new[]', content)
    content = re.sub(r'inline\s+void\s+operator\s+delete', r'void operator delete', content)
    with open(sigma_oop_path, "w", encoding="utf-8") as f:
        f.write(content)


# Fix 3: Fix SovereignFS.cpp uint32_t to sigma_u32
sov_fs_path = os.path.join(WORKSPACE_DIR, "kernel", "core", "system", "SovereignFS.cpp")
if os.path.exists(sov_fs_path):
    with open(sov_fs_path, "r", encoding="utf-8") as f:
        content = f.read()
    content = content.replace("uint32_t", "sigma_u32")
    content = re.sub(r'#include\s+["<].*sigma_allocator\.h[">]\n', '', content)
    with open(sov_fs_path, "w", encoding="utf-8") as f:
        f.write(content)

# Fix 4: Fix tools/sigma_vr_studio.cpp "Unknown type name 'm'"
vr_studio_path = os.path.join(WORKSPACE_DIR, "tools", "sigma_vr_studio.cpp")
if os.path.exists(vr_studio_path):
    with open(vr_studio_path, "r", encoding="utf-8") as f:
        content = f.read()
    if content.startswith("m\n") or content.startswith("m"):
        content = content.replace("m\n", "", 1)
        if content.startswith("m"):
            content = content[1:]
    with open(vr_studio_path, "w", encoding="utf-8") as f:
        f.write(content)

# Fix 5: HTML background-clip
for html_file in [os.path.join(WORKSPACE_DIR, "..", "SigmaOS-Site", "index.html"), os.path.join(WORKSPACE_DIR, "installer.html")]:
    if os.path.exists(html_file):
        with open(html_file, "r", encoding="utf-8") as f:
            content = f.read()
        content = content.replace("-webkit-background-clip", "background-clip: text; -webkit-background-clip")
        with open(html_file, "w", encoding="utf-8") as f:
            f.write(content)

# Fix 6: SovereignPaging PAGE_SIZE redefined
paging_path = os.path.join(WORKSPACE_DIR, "memory", "paging", "SovereignPaging.cpp")
if os.path.exists(paging_path):
    with open(paging_path, "r", encoding="utf-8") as f:
        content = f.read()
    content = content.replace("#define PAGE_SIZE 4096", "#ifndef PAGE_SIZE\n#define PAGE_SIZE 4096\n#endif")
    with open(paging_path, "w", encoding="utf-8") as f:
        f.write(content)


# Fix 7: Remove Unused Headers
unused_headers = {
    r"include\\net\\sigma_network\.h": ["sigma_types.h"],
    r"include\\sigma_boot\.h": ["sigma_kernel_types.h"],
    r"include\\sigma_optimizer\.h": ["sigma_kernel_types.h"],
    r"include\\libc\\SovereignLibC\.h": ["sigma_types.h"],
    r"include\\storage\\sigma_storage\.h": ["sigma_types.h"],
    r"include\\virt\\sigma_hypervisor\.h": ["sigma_types.h"],
    r"kernel\\core\\ai\\SovereignNeuralHealer\.cpp": ["sigma_kernel_types.h"],
    r"kernel\\core\\boot\\SovereignInit\.cpp": ["SovereignLibC.h", "sigma_hal.h"],
    r"kernel\\core\\drivers\\SovereignVideo\.cpp": ["SigmaOOP.hpp", "sigma_types.h"],
    r"kernel\\core\\memory_manager\.cpp": ["sigma_hal.h"],
    r"kernel\\core\\network\\SovereignVPN\.cpp": ["sigma_kernel_types.h"],
    r"kernel\\core\\SovereignHealthMonitor\.cpp": ["sigma_kernel_types.h", "sigma_time.h"],
    r"kernel\\core\\SovereignMain\.cpp": ["sigma_hal.h"],
    r"kernel\\core\\SovereignOptimizer\.cpp": ["SigmaOOP.hpp"],
    r"kernel\\core\\SovereignSnapshotDiff\.cpp": ["string.h"],
    r"security\\SovereignFuzzer\.cpp": ["sigma_hal.h"],
    r"suites\\S32_SystemTools\\core\\sovereign_shell\\sovereign_shell\.cpp": ["sigma_types.h", "SigmaOOP.hpp"],
    r"tools\\sigma_auto_diag\.cpp": ["sigma_kernel_types.h"],
    r"tools\\sigma_robotics_planner\.cpp": ["sigma_kernel_types.h"]
}

for root, dirs, files in os.walk(WORKSPACE_DIR):
    for file in files:
        file_path = os.path.join(root, file)
        for pattern, headers in unused_headers.items():
            if re.search(pattern, file_path):
                with open(file_path, "r", encoding="utf-8") as f:
                    content = f.read()
                for h in headers:
                    content = re.sub(rf'#include\s+["<].*{re.escape(h)}[">]\n', '', content)
                with open(file_path, "w", encoding="utf-8") as f:
                    f.write(content)

# Fix 8: PowerShell warnings
fix_inc_ps1 = os.path.join(WORKSPACE_DIR, "scripts", "fix_all_includes.ps1")
if os.path.exists(fix_inc_ps1):
    with open(fix_inc_ps1, "r", encoding="utf-8") as f:
        content = f.read()
    content = content.replace("Fix-Includes", "Repair-Includes")
    with open(fix_inc_ps1, "w", encoding="utf-8") as f:
        f.write(content)

ver_sov_ps1 = os.path.join(WORKSPACE_DIR, "verify_sovereignty.ps1")
if os.path.exists(ver_sov_ps1):
    with open(ver_sov_ps1, "r", encoding="utf-8") as f:
        content = f.read()
    content = content.replace("$Matches", "$FoundMatches")
    with open(ver_sov_ps1, "w", encoding="utf-8") as f:
        f.write(content)


# Fix 9: Markdown Formatting
def fix_markdown(filepath):
    with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
        lines = f.readlines()
    
    new_lines = []
    in_code_block = False
    for i, line in enumerate(lines):
        if line.startswith("```"):
            in_code_block = not in_code_block
            
            # MD031: ensure blank line before and after code block
            if in_code_block and i > 0 and lines[i-1].strip() != "":
                new_lines.append("\n")
            new_lines.append(line)
            if not in_code_block and i < len(lines)-1 and lines[i+1].strip() != "":
                new_lines.append("\n")
            continue
            
        if in_code_block:
            new_lines.append(line)
            continue
            
        # MD022: headings surrounded by blank lines
        if line.startswith("#"):
            if i > 0 and lines[i-1].strip() != "":
                new_lines.append("\n")
            
            # MD026: remove trailing punctuation in heading
            heading_content = line.strip()
            while heading_content and heading_content[-1] in ":.,;":
                heading_content = heading_content[:-1]
            new_lines.append(heading_content + "\n")
            
            if i < len(lines)-1 and lines[i+1].strip() != "":
                new_lines.append("\n")
            continue
            
        # MD032: lists surrounded by blank lines
        if line.strip().startswith("- ") or line.strip().startswith("* ") or re.match(r'^\d+\.\s', line.strip()):
            if i > 0 and lines[i-1].strip() != "" and not (lines[i-1].strip().startswith("- ") or lines[i-1].strip().startswith("* ") or re.match(r'^\d+\.\s', lines[i-1].strip())):
                new_lines.append("\n")
            
            # Convert dash to asterisk for MD004 (if unorded list)
            if line.strip().startswith("- "):
                line = line.replace("- ", "* ", 1)
                
            new_lines.append(line)
            if i < len(lines)-1 and lines[i+1].strip() != "" and not (lines[i+1].strip().startswith("- ") or lines[i+1].strip().startswith("* ") or re.match(r'^\d+\.\s', lines[i+1].strip())):
                new_lines.append("\n")
            continue
            
        # MD036: emphasis used instead of heading -> replace **Text** with ### Text if on own line
        if line.strip().startswith("**") and line.strip().endswith("**") and len(line.strip()) > 4:
            new_lines.append("### " + line.strip()[2:-2] + "\n")
            continue
            
        # MD037: space in emphasis
        line = re.sub(r'\*\*\s+(.*?)\s+\*\*', r'**\1**', line)
        line = re.sub(r'\*\s+(.*?)\s+\*', r'*\1*', line)
        
        # MD012: multiple blanks
        if line.strip() == "" and len(new_lines) > 0 and new_lines[-1].strip() == "":
            continue
            
        new_lines.append(line)
        
    # MD047: single trailing newline
    if new_lines and not new_lines[-1].endswith("\n"):
        new_lines[-1] += "\n"
        
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
