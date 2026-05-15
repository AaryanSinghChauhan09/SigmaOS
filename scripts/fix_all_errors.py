import os
import re

def fix_all_errors():
    root_dir = os.getcwd()
    # Registry of common headers to their paths relative to root/include/
    # If someone uses #include "header.h" and it's in our include/ dir, we fix it.
    include_headers = {}
    include_root = os.path.join(root_dir, "include")
    for r, d, files in os.walk(include_root):
        for f in files:
            rel_path = os.path.relpath(os.path.join(r, f), include_root).replace('\\', '/')
            include_headers[f] = rel_path

    count = 0
    for root, dirs, files in os.walk(root_dir):
        if ".git" in root or "obj" in root or "wiki_repo" in root:
            continue
            
        for file in files:
            if file.endswith(('.cpp', '.hpp', '.c', '.h')):
                file_path = os.path.join(root, file)
                
                rel_to_root = os.path.relpath(root, root_dir)
                if rel_to_root == ".":
                    depth = 0
                else:
                    depth = len(rel_to_root.replace('\\', '/').split('/'))
                
                prefix = "../" * depth
                
                with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                original_content = content
                
                # 1. Fix depth for includes that already use "include/"
                # pattern: #include "..." or #include <...>
                def include_replacer(match):
                    inc_path = match.group(2)
                    # If it's already a relative path starting with ../, it might be wrong depth
                    if inc_path.startswith("../"):
                        # Extract the part after all ../ and any "include/"
                        clean_path = re.sub(r'^(\.\./)+', '', inc_path)
                        clean_path = re.sub(r'^include/', '', clean_path)
                        
                        # Find the correct path in our include registry if possible
                        header_name = os.path.basename(clean_path)
                        if header_name in include_headers:
                            return f'#include "{prefix}include/{include_headers[header_name]}"'
                        else:
                            return f'#include "{prefix}include/{clean_path}"'
                    
                    # If it's a "local" include but it's actually in include/
                    if inc_path in include_headers:
                         return f'#include "{prefix}include/{include_headers[inc_path]}"'
                    
                    # If it's e.g. "hal/sigma_hal.h"
                    if "/" in inc_path and inc_path.split("/")[0] in ["hal", "libc", "core", "fs", "drivers", "system", "ui"]:
                         # Check if it's in include/
                         if inc_path in include_headers.values():
                             return f'#include "{prefix}include/{inc_path}"'
                             
                    return match.group(0)

                content = re.sub(r'#include\s+([<"])(.*?)([>"])', include_replacer, content)
                
                # 2. Fix sigma_printf return type in implementations
                if file == "SovereignLibC.cpp":
                    content = content.replace("void sigma_printf", "int sigma_printf")
                    if "return count;" not in content:
                        # (Already fixed in my previous edit, but just in case)
                        pass

                # 3. Fix common missing namespace/class issues
                # (This is harder to automate without more context, but let's fix known ones)
                
                if content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    print(f"Fixed: {file_path}")
                    count += 1

    print(f"Total files refactored: {count}")

if __name__ == "__main__":
    fix_all_errors()
