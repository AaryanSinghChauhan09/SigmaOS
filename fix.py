import os
import re

html_path = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\zenith.html"
with open(html_path, "r", encoding="utf-8") as f:
    content = f.read()

def fix_html(match):
    text = match.group(0)
    if "title=" not in text:
        text = text.replace(">", ' title="input">')
    if "placeholder=" not in text and ("<input" in text or "<textarea" in text):
        text = text.replace(">", ' placeholder="input">')
    return text

content = re.sub(r'<input[^>]*>', fix_html, content)
content = re.sub(r'<select[^>]*>', fix_html, content)
content = re.sub(r'<textarea[^>]*>', fix_html, content)

with open(html_path, "w", encoding="utf-8") as f:
    f.write(content)

problem_files = {
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignAllocator.cpp": ["sigma_hal.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\drivers\SovereignVideo.cpp": ["SigmaOOP.hpp", "sigma_types.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\scheduler.cpp": ["sigma_hal.h", "SovereignLibC.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignCgroup.cpp": ["sigma_hal.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignContainer.cpp": ["sigma_hal.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignLBU.cpp": ["sigma_hal.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignOverlayFS.cpp": ["sigma_hal.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignSyscall.cpp": ["SovereignLibC.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\SovereignTuner.cpp": ["sigma_kernel_types.h", "sigma_hal.h", "SovereignLibC.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\syscall\dispatcher.h": ["syscalls.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\scheduler\SovereignScheduler.cpp": ["SovereignLibC.h", "sigma_hal.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\tests\kselftest\kselftest_sigma.h": ["string.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\userland\apps\SigmaAIIntegration.cpp": ["SovereignLibC.h"],
    r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\include\sigma_boot.h": ["sigma_kernel_types.h"],
}

for path, headers in problem_files.items():
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            lines = f.readlines()
        with open(path, "w", encoding="utf-8") as f:
            for line in lines:
                skip = False
                for h in headers:
                    if "#include" in line and h in line:
                        skip = True
                        break
                if not skip:
                    f.write(line)

path = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\tools\profession_calculators.cpp"
if os.path.exists(path):
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    count = 0
    with open(path, "w", encoding="utf-8") as f:
        for line in lines:
            if "sigma_log.h" in line:
                count += 1
                if count == 2:
                    continue
            f.write(line)

path = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\kernel\core\syscall\sigma_syscall_dispatcher.h"
if os.path.exists(path):
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    content = content.replace("#define K_ERR_INVAL", "#ifndef K_ERR_INVAL\n#define K_ERR_INVAL")
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
