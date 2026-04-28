import os
import re

def fix_includes(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith(('.hpp', '.cpp', '.h', '.c')):
                path = os.path.join(root, file)
                with open(path, 'r') as f:
                    content = f.read()
                
                # Replace various SigmaOOP.hpp include styles with root-relative <SigmaOOP.hpp>
                new_content = re.sub(r'#include\s+["\'].*SigmaOOP\.hpp["\']', '#include <SigmaOOP.hpp>', content)
                
                # Also fix SovereignLibC.h if it's included directly
                new_content = re.sub(r'#include\s+["\'].*SovereignLibC\.h["\']', '#include <SovereignLibC.h>', new_content)

                if new_content != content:
                    with open(path, 'w') as f:
                        f.write(new_content)
                    print(f"Fixed: {path}")

if __name__ == "__main__":
    fix_includes("kernel")
    fix_includes("drivers")
    fix_includes("libc")
