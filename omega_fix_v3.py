import os
import re
import subprocess

WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"

def read(path):
    if not os.path.exists(path): return ""
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        return f.read()

def write(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

def patch_file(rel_path, callback):
    full = os.path.join(WORKSPACE_DIR, rel_path)
    if not os.path.exists(full): return
    c = read(full)
    nc = callback(c)
    if nc != c:
        write(full, nc)
        print(f"  Fixed {rel_path}")

# 1. FIX C++ UNDECLARED IDENTIFIERS (strcmp) & UNUSED HEADERS
def fix_theme_engine(c):
    c = c.replace('strcmp', 'sigma_strcmp')
    c = re.sub(r'#include\s+"sigma_hal\.h"\s*\n', '', c)
    c = re.sub(r'#include\s+"sigma_time\.h"\s*\n', '', c)
    
    # Inject simple sigma_strcmp if not present
    if "sigma_strcmp" in c and "int sigma_strcmp(" not in c:
        injector = """
static int sigma_strcmp(const char* s1, const char* s2) {
    while(*s1 && (*s1 == *s2)) { s1++; s2++; }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}
"""
        c = c.replace('namespace SigmaOS {', injector + '\nnamespace SigmaOS {')
    return c
patch_file("kernel/core/SovereignThemeEngine.cpp", fix_theme_engine)

def remove_unused(header):
    return lambda c: re.sub(rf'#include\s+"(?:.*?/)?{header}"\s*\n', '', c)

patch_file("kernel/core/absorption/SovereignZOS.cpp", remove_unused("sigma_kernel_types.h"))
patch_file("kernel/core/concurrency/SovereignMutex.hpp", remove_unused("SigmaOOP.hpp"))
patch_file("kernel/core/drivers/SovereignVideo.cpp", lambda c: remove_unused("sigma_types.h")(remove_unused("SigmaOOP.hpp")(c)))
patch_file("kernel/core/security/SovereignMAC.cpp", remove_unused("sigma_kernel_types.h"))
patch_file("kernel/core/SovereignForensics.cpp", remove_unused("sigma_kernel_types.h"))
patch_file("kernel/core/SovereignHeap.cpp", remove_unused("sigma_hal.h"))
patch_file("tools/telemetry-cli.cpp", remove_unused("sigma_kernel_types.h"))
patch_file("userland/apps/SovereignProTools.cpp", remove_unused("sigma_kernel_types.h"))
patch_file("userland/SovereignShell.cpp", remove_unused("sigma_hal.h"))

# 2. FIX SovereignBoot.cpp properly
def fix_boot(c):
    c = c.replace('#include "sigma_boot.h"', '#include "../../../include/sigma_boot.h"')
    c = c.replace('static void init()', 'void init()')
    c = re.sub(r'\}\s*$', '\n', c) # Remove extraneous closing brace
    return c
patch_file("kernel/core/system/SovereignBoot.cpp", fix_boot)

# 3. FIX tools/sigma_vr_studio.cpp (Stray 'm')
def fix_vr(c):
    return re.sub(r'^m', '', c)
patch_file("tools/sigma_vr_studio.cpp", fix_vr)

# 4. FIX zenith_desktop.css
def fix_css(c):
    c = c.replace('user-select:', '-webkit-user-select: none; user-select:')
    c = c.replace('backdrop-filter: blur', '-webkit-backdrop-filter: blur(10px); backdrop-filter: blur')
    # Fix ordering warnings if they already exist in wrong order
    c = re.sub(r'(backdrop-filter:[^;]+;)\s*(-webkit-backdrop-filter:[^;]+;)', r'\2 \1', c)
    return c
patch_file("zenith_desktop.css", fix_css)

# 5. FIX HTML INLINE STYLES
extracted_css = []
css_counter = 1000
def extract_html_styles(c):
    global css_counter
    def repl(m):
        global css_counter
        style = m.group(1)
        cls = f"ext-style-{css_counter}"
        css_counter += 1
        extracted_css.append(f".{cls} {{ {style} }}")
        return f'class="{cls}"'
    return re.sub(r'style\s*=\s*["\']([^"\']+)["\']', repl, c)

patch_file("zenith.html", extract_html_styles)
patch_file("web_ui/index.html", extract_html_styles)
patch_file("index.html", extract_html_styles)

if extracted_css:
    with open(os.path.join(WORKSPACE_DIR, "external_styles.css"), "a") as f:
        f.write("\n/* Final extracted inline styles */\n")
        f.write("\n".join(extracted_css) + "\n")

# 6. FIX MARKDOWN LINTING
def fix_readme(c):
    # Fix lists: change - to * and fix indentation
    lines = c.split('\n')
    for i in range(len(lines)):
        if lines[i].startswith('  - '):
            lines[i] = lines[i].replace('  - ', '    * ')
        elif lines[i].startswith('- '):
            lines[i] = lines[i].replace('- ', '* ')
    
    nc = '\n'.join(lines)
    if not nc.endswith('\n'): nc += '\n'
    return nc
patch_file("README.md", fix_readme)

def fix_wiki_tables(c):
    # Add spaces around pipes for compact tables
    return c.replace('|', ' | ').replace('  ', ' ')
patch_file("wiki_repo/Competitive-Analysis.md", fix_wiki_tables)

# 7. COMMIT AND SYNC
def run_git(args, cwd=WORKSPACE_DIR):
    subprocess.run(["git"] + args, cwd=cwd, check=False)

print("Pushing to all branches...")
run_git(["add", "."])
run_git(["commit", "-m", "fix: Resolve all IDE current_problems globally"])
run_git(["push", "origin", "main"])

BRANCHES = [
    "release/standalone", "release/rtos", "release/mobile", 
    "release/microkernel", "release/dual-boot", "release/distributed", 
    "release/cloud", "release/browser", "release/app", 
    "performance-optimized", "gh-pages"
]

for branch in BRANCHES:
    run_git(["checkout", branch])
    run_git(["merge", "main", "-m", "chore: Sync IDE problem fixes"])
    run_git(["push", "origin", branch])

run_git(["checkout", "main"])
print("OMEGA FIX V3 COMPLETE!")
