import os
import re

cpp_fixes = {
    "kernel/core/hal/SovereignHAL.cpp": [
        ('#include "../../../include/sigma_log.h"\n#include "../../../include/sigma_hal.h"\n#include "../../../include/libc/SovereignLibC.h"', 
         '#include "../../../include/sigma_log.h"\n#include "../../../include/hal/sigma_hal.h"')
    ],
    "include/core/sigma_scheduler.h": [
        ('#include "./sigma_kernel_types.h"\n', '')
    ],
    "include/hal/sigma_hal.h": [
        ('#include "../core/sigma_types.h"\n', '')
    ],
    "kernel/core/drivers/SovereignPCMCIA.cpp": [
        ('#include "../../../include/sigma_kernel_types.h"\n', '')
    ],
    "kernel/core/drivers/SovereignVideo.cpp": [
        ('#include "../../../include/SigmaOOP.hpp"\n#include "../../../include/sigma_kernel_types.h"\n#include "../../../include/sigma_log.h"', 
         '#include "../../../include/sigma_kernel_types.h"\n#include "../../../include/sigma_log.h"'),
        ('#include "../../../include/sigma_types.h"\n', '')
    ],
    "kernel/core/SovereignHypervisor.cpp": [
        ('#include "../../include/sigma_kernel_types.h"\n', '')
    ],
    "kernel/core/SovereignRegression.cpp": [
        ('#include "../../include/SigmaOOP.hpp"\n', '')
    ],
    "ui/SovereignZenithDesktop.cpp": [
        ('#include "../include/sigma_kernel_types.h"\n#include "../include/hal/sigma_hal.h"\n#include "../include/sigma_kernel_types.h"\n#include "../include/libc/SovereignLibC.h"', 
         '#include "../include/sigma_kernel_types.h"')
    ],
    "userland/ZenithDesktop.cpp": [
        ('#include "../include/SigmaOOP.hpp"\n#include "../include/sigma_kernel_types.h"\n#include "../include/sigma_log.h"\n#include "../include/ui/SovereignGUI.h"', 
         '#include "../include/SigmaOOP.hpp"\n#include "../include/sigma_log.h"')
    ]
}

def apply_fixes(fixes_dict, base_dir):
    for rel_path, fixes in fixes_dict.items():
        path = os.path.join(base_dir, rel_path)
        if not os.path.exists(path):
            print(f"File not found: {path}")
            continue
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        for old, new in fixes:
            content = content.replace(old, new)

        with open(path, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Fixed {rel_path}")

base_dir = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
apply_fixes(cpp_fixes, base_dir)

# HTML Fixes
app_store_path = os.path.join(base_dir, 'app_store.html')
if os.path.exists(app_store_path):
    with open(app_store_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # fix backdrop
    content = content.replace('backdrop-filter: blur(12px);\n            -webkit-backdrop-filter: blur(12px);',
                              '-webkit-backdrop-filter: blur(12px);\n            backdrop-filter: blur(12px);')
    
    # fix inline styles
    content = content.replace('<style>', '''<style>
        .profile-icon {
            width: 40px; height: 40px; border-radius: 50%; background: var(--glass-bg); border: 1px solid var(--glass-border); display: flex; justify-content:center; align-items:center; cursor: pointer;
        }
        .delay-1 { animation-delay: 0.1s; }
        .delay-2 { animation-delay: 0.2s; }
        .delay-3 { animation-delay: 0.3s; }
        .delay-4 { animation-delay: 0.4s; }
        .delay-5 { animation-delay: 0.5s; }
        .delay-6 { animation-delay: 0.6s; }
        .delay-7 { animation-delay: 0.7s; }
        .install-banner-btn { padding: 12px 30px; width: auto; }''')
    
    content = content.replace('style="width: 40px; height: 40px; border-radius: 50%; background: var(--glass-bg); border: 1px solid var(--glass-border); display: flex; justify-content:center; align-items:center; cursor: pointer;"', 'class="profile-icon"')
    content = content.replace('style="animation-delay: 0.1s;"', 'class="delay-1"')
    content = content.replace('style="animation-delay: 0.2s;"', 'class="delay-2"')
    content = content.replace('style="animation-delay: 0.3s;"', 'class="delay-3"')
    content = content.replace('style="animation-delay: 0.4s;"', 'class="delay-4"')
    content = content.replace('style="animation-delay: 0.5s;"', 'class="delay-5"')
    content = content.replace('style="animation-delay: 0.6s;"', 'class="delay-6"')
    content = content.replace('style="animation-delay: 0.7s;"', 'class="delay-7"')
    content = content.replace('style="padding: 12px 30px; width: auto;"', 'class="install-banner-btn"')
    content = content.replace('class="featured-banner animate" class="delay-1"', 'class="featured-banner animate delay-1"')
    content = content.replace('class="install-btn" class="install-banner-btn"', 'class="install-btn install-banner-btn"')
    content = content.replace('class="app-card animate" class="delay-2"', 'class="app-card animate delay-2"')
    content = content.replace('class="app-card animate" class="delay-3"', 'class="app-card animate delay-3"')
    content = content.replace('class="app-card animate" class="delay-4"', 'class="app-card animate delay-4"')
    content = content.replace('class="app-card animate" class="delay-5"', 'class="app-card animate delay-5"')
    content = content.replace('class="app-card animate" class="delay-6"', 'class="app-card animate delay-6"')
    content = content.replace('class="app-card animate" class="delay-7"', 'class="app-card animate delay-7"')
    
    with open(app_store_path, 'w', encoding='utf-8') as f:
        f.write(content)
    print("Fixed app_store.html")

installer_path = os.path.join(base_dir, 'installer.html')
if os.path.exists(installer_path):
    with open(installer_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    content = content.replace('<style>', '''<style>
        .mt-30 { margin-top: 30px; }
        .partition-manual { width: 100%; background: #222; color: #888; }''')
    content = content.replace('style="margin-top: 30px;"', 'class="mt-30"')
    content = content.replace('class="tip-box" class="mt-30"', 'class="tip-box mt-30"')
    content = content.replace('style="width:100%; background: #222; color: #888;"', 'class="partition-manual"')
    content = content.replace('class="partition" class="partition-manual"', 'class="partition partition-manual"')
    
    with open(installer_path, 'w', encoding='utf-8') as f:
        f.write(content)
    print("Fixed installer.html")
    
# Markdown Fixes
import glob
wiki_dir = os.path.join(base_dir, 'wiki_repo')
md_files = glob.glob(os.path.join(wiki_dir, '*.md'))

for md_file in md_files:
    with open(md_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # MD037/no-space-in-emphasis: spaces inside emphasis markers
    content = re.sub(r'(\*\*|\*|_|__)\s+(.*?)\s+(\1)', r'\1\2\3', content)
    
    # MD026/no-trailing-punctuation: remove trailing colon from headings
    content = re.sub(r'^(#+ .*):(\s*)$', r'\1\2', content, flags=re.MULTILINE)
    
    # MD036/no-emphasis-as-heading: convert emphasis heading to ### heading
    # Looks like a line that is just **something**
    content = re.sub(r'^(\*\*|__)(.+?)\1\s*$', r'### \2', content, flags=re.MULTILINE)
    
    # MD031/blanks-around-fences
    # Ensure blank line before and after ```
    content = re.sub(r'([^\n])\n```', r'\1\n\n```', content)
    content = re.sub(r'```\n([^\n])', r'```\n\n\1', content)

    # MD060/table-column-style
    # This is tricky to fix automatically, but we can try to format the table.
    
    # Specific fixes based on lint errors:
    if "Common-OS-Problems-Solutions.md" in md_file:
        # MD024/no-duplicate-heading
        # Find duplicate headings and make them unique
        headings = {}
        def repl(match):
            h = match.group(0)
            if h in headings:
                headings[h] += 1
                return f"{h} {headings[h]}"
            headings[h] = 1
            return h
        content = re.sub(r'^#+ .*$', repl, content, flags=re.MULTILINE)
    
    if "Developer-Roadmap.md" in md_file:
        # MD029/ol-prefix: ordered list item prefix
        # Just normalize the list numbers to 1.
        content = re.sub(r'^\s*\d+\.\s', '1. ', content, flags=re.MULTILINE)

    with open(md_file, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f"Fixed {os.path.basename(md_file)}")

