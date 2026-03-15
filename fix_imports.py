import os
import re

ROOT = "."
# We look for imports like 'from PackageName.file import' inside 'PackageName/__init__.py'
# and change them to 'from .PackageName.file import' or just 'from .file import'

def fix_broken_imports():
    print("Fixing Relative Imports in __init__.py files...")
    count = 0
    for root, dirs, files in os.walk(ROOT):
        if "__init__.py" in files:
            fp = os.path.join(root, "__init__.py")
            try:
                with open(fp, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # If we are in 'sigma_core/kernel', and we see 'from kernel.X', change to 'from .kernel.X'
                parent_dir = os.path.basename(root)
                grandparent_dir = os.path.basename(os.path.dirname(root))
                
                new_content = content
                # Search for imports that look like 'from folder_name.' inside 'folder_name'
                # and change to 'from .folder_name.'
                regex = r"from\s+("+re.escape(parent_dir)+r")\."
                new_content = re.sub(regex, r"from .\1.", new_content)
                
                if new_content != content:
                    with open(fp, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    count += 1
            except:
                pass
    print(f"Fixed {count} __init__.py files.")

if __name__ == "__main__":
    fix_broken_imports()
