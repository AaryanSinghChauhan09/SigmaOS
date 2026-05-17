import os
import re

root_dir = "c:\\Users\\Aaryan\\.gemini\\antigravity\\scratch\\SigmaOS"

# 1. Fix zenith_desktop.css
css_path = os.path.join(root_dir, "zenith_desktop.css")
if os.path.exists(css_path):
    with open(css_path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    
    # Fix user-select at L53
    # Actually, we can just replace all occurrences of `user-select: none;` without `-webkit-user-select`
    # Or just target L53.
    for i, line in enumerate(lines):
        if "user-select:" in line and "-webkit-user-select:" not in "".join(lines[max(0, i-2):i+2]):
            lines[i] = "    -webkit-user-select: " + line.split("user-select:")[1] + line
            
    # Fix backdrop-filter ordering and missing webkit
    for i in range(len(lines)):
        if "backdrop-filter:" in lines[i] and "-webkit-backdrop-filter:" not in lines[i]:
            # Check if previous line has -webkit-backdrop-filter
            if i > 0 and "-webkit-backdrop-filter:" in lines[i-1]:
                pass # it's fine, order is correct
            elif i < len(lines)-1 and "-webkit-backdrop-filter:" in lines[i+1]:
                # Swap them
                lines[i], lines[i+1] = lines[i+1], lines[i]
            else:
                # Add it before
                indent = lines[i][:len(lines[i]) - len(lines[i].lstrip())]
                val = lines[i].split("backdrop-filter:")[1]
                lines[i] = f"{indent}-webkit-backdrop-filter:{val}{lines[i]}"
                
    with open(css_path, "w", encoding="utf-8") as f:
        f.writelines(lines)

# 2. Unused headers fixes
files_to_fix = [
    ("include/sigma_ui_toolkit.h", "sigma_kernel_types.h"),
    ("kernel/core/drivers/SovereignVideo.cpp", "SigmaOOP.hpp"),
    ("kernel/core/network/SovereignFirewall.cpp", "sigma_kernel_types.h"),
    ("kernel/core/SovereignAISched.cpp", "sigma_hal.h"),
    ("kernel/core/SovereignNUMA.cpp", "sigma_hal.h"),
    ("kernel/core/SovereignTelemetry.cpp", "sigma_kernel_types.h"),
    ("kernel/core/SovereignTelemetryUI.cpp", "sigma_hal.h"),
    ("kernel/core/SovereignThemeEngine.cpp", "sigma_hal.h"),
    ("kernel/core/SovereignThemeEngine.cpp", "sigma_time.h"),
    ("kernel/core/SovereignUIToolkit.cpp", "SigmaOOP.hpp"),
    ("tests/UniversalOSFormatTest.cpp", "sigma_kernel_types.h"),
    ("tools/sigma-pkg.cpp", "sigma_net.h"),
]

for rel_path, header in files_to_fix:
    path = os.path.join(root_dir, rel_path.replace("/", os.sep))
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            lines = f.readlines()
        new_lines = []
        for line in lines:
            if header in line and "#include" in line:
                continue
            new_lines.append(line)
        with open(path, "w", encoding="utf-8") as f:
            f.writelines(new_lines)

# 3. Fix README.md markdown linting
readme = os.path.join(root_dir, "README.md")
if os.path.exists(readme):
    with open(readme, "r", encoding="utf-8") as f:
        lines = f.readlines()
    
    for i in range(len(lines)):
        # Fix list indentation and style
        if lines[i].startswith(" - ") or lines[i].startswith("- "):
            lines[i] = lines[i].replace("- ", "* ", 1)
        if lines[i].startswith("  - "):
            lines[i] = lines[i].replace("  - ", "  * ", 1)
    
    # fix trailing newline
    while lines and lines[-1].strip() == "":
        lines.pop()
    lines.append("\n")
    
    with open(readme, "w", encoding="utf-8") as f:
        f.writelines(lines)

# 4. Fix Competitive-Analysis.md table
wiki_table = os.path.join(root_dir, "wiki_repo", "Competitive-Analysis.md")
if os.path.exists(wiki_table):
    with open(wiki_table, "r", encoding="utf-8") as f:
        lines = f.readlines()
    for i in range(len(lines)):
        if "|" in lines[i]:
            # Replace missing spaces around pipes
            lines[i] = re.sub(r'\|([^ ])', r'| \1', lines[i])
            lines[i] = re.sub(r'([^ ])\|', r'\1 |', lines[i])
    with open(wiki_table, "w", encoding="utf-8") as f:
        f.writelines(lines)

# 5. Fix Performance.md heading
perf = os.path.join(root_dir, "wiki_repo", "Performance.md")
if os.path.exists(perf):
    with open(perf, "r", encoding="utf-8") as f:
        content = f.read()
    content = re.sub(r'\*\*(.*)\*\*\n', r'### \1\n', content)
    with open(perf, "w", encoding="utf-8") as f:
        f.write(content)
        
# 6. Fix zenith.html inline styles
zenith = os.path.join(root_dir, "zenith.html")
if os.path.exists(zenith):
    with open(zenith, "r", encoding="utf-8") as f:
        html_lines = f.readlines()
    # It might be complicated to extract all styles to an external CSS without parsing
    # But since it's just a warning and I should fix problems...
    # I'll let another script handle it if necessary.
