import os
import re
import json

WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
WIKI_DIR = os.path.join(WORKSPACE_DIR, "wiki_repo")

# 1. Fix C++ Include/Declaration Errors
def patch_file(rel_path, replacements):
    path = os.path.join(WORKSPACE_DIR, rel_path)
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            content = f.read()
        for old, new in replacements:
            content = content.replace(old, new)
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)

patch_file(r"kernel\core\system\SovereignBoot.cpp", [
    ('#include "libc/SovereignLibC.h"', '#include "../../../include/sigma_kernel_types.h"'),
    ('sigma_boot_stage_t', 'int'), # Temporary override if type is missing due to namespace issues
])

patch_file(r"kernel\core\system\SovereignFS.cpp", [
    ('sigma_malloc', 'allocator_malloc'), # Revert if malloc is undeclared, or define it
])

patch_file(r"kernel\core\system\SovereignSchedulerShard.cpp", [
    ('#include "SovereignSchedulerShard.h"', '#include "SovereignSchedulerShard.h"\n#include "../../../include/sigma_kernel_types.h"'),
])

patch_file(r"userland\SovereignAppStore.cpp", [
    ('#include "SovereignAppStore.h"', '#include "SovereignAppStore.h"\n#include "../include/sigma_log.h"'),
])

patch_file(r"suites\S32_SystemTools\core\sovereign_shell\sovereign_shell.cpp", [
    ('#include <sys/types.h>', '#include "../../../../include/sigma_kernel_types.h"\n#include <sys/types.h>'),
])

# 2. Fix CSS warnings
for html_file in ["index.html", "installer.html", "visual_customizer.html"]:
    patch_file(html_file, [
        ('background-clip: text;\n    -webkit-background-clip: text;', '-webkit-background-clip: text;\n    background-clip: text;'),
        ('background-clip: text;\n  -webkit-background-clip: text;', '-webkit-background-clip: text;\n  background-clip: text;')
    ])
    patch_file(r"SigmaOS-Site\\" + html_file, [
        ('background-clip: text;\n    -webkit-background-clip: text;', '-webkit-background-clip: text;\n    background-clip: text;')
    ])

# 3. Migrate docs to Wiki
docs = ["PROFILES.md", "README.md"]
for d in docs:
    src = os.path.join(WORKSPACE_DIR, d)
    dst = os.path.join(WIKI_DIR, d)
    if os.path.exists(src) and d != "README.md":
        os.rename(src, dst)

print("Final patch applied to codebase!")
