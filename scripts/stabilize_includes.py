import os
import re

def fix_includes(root_dir):
    pattern = re.compile(r'#include\s+"(?:\.\./)+SigmaOOP\.hpp"')
    replacement = '#include "SigmaOOP.hpp"'
    
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith(('.hpp', '.cpp', '.c')):
                path = os.path.join(root, file)
                with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                new_content = pattern.sub(replacement, content)
                
                if new_content != content:
                    print(f"Fixed: {path}")
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)

if __name__ == "__main__":
    fix_includes("kernel")
    fix_includes("userland")
    fix_includes("ecosystem")
    fix_includes("libc")
