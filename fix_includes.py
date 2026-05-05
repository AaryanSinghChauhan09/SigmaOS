import os
import re

def fix_includes(root_dir):
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith(('.cpp', '.h', '.c', '.hpp')):
                path = os.path.join(root, file)
                with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                # Replace #include "path/to/header.h"" with #include "path/to/header.h"
                new_content = re.sub(r'(#include\s+["<][^">]+[">])""', r'\1', content)
                
                if new_content != content:
                    print(f"Fixing {path}")
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)

if __name__ == "__main__":
    fix_includes('kernel')
    fix_includes('include')
