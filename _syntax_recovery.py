import os
import re

def fix_syntax_errors(directory):
    """
    Fixes common syntax error patterns in SigmaOS .py files.
    - Replaces 'apps' with 'apps' when used as a variable/identifier.
    - Replaces 'web_os' with 'web_os' in identifiers.
    - Fixes corrupted escape sequences like \" when they break strings.
    """
    # Pattern 1: apps as identifier (variable, function name, etc)
    # We look for apps optionally preceded/followed by alpha-numeric characters (which would be invalid)
    # Most likely it's 'self.apps' or 'apps = '
    # We replace with 'apps' or 'userland_apps'
    
    # regex for invalid identifier characters (slashes in names)
    pattern_apps = re.compile(r'([a-zA-Z0-9_]*)apps([a-zA-Z0-9_]*)')
    pattern_gui = re.compile(r'([a-zA-Z0-9_]*)web_os([a-zA-Z0-9_]*)')
    # Pattern to fix bad escaping like \" -> \"
    pattern_escape = re.compile(r'\\\"')
    
    total_fixed = 0
    for root, _, files in os.walk(directory):
        if '.git' in root: continue
        for file in files:
            if file.endswith('.py'):
                path = os.path.join(root, file)
                try:
                    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                    
                    new_content = content
                    # Fix apps
                    new_content = pattern_apps.sub(r'\1apps\2', new_content)
                    # Fix web_os
                    new_content = pattern_gui.sub(r'\1web_os\2', new_content)
                    # Fix corrupted escapes
                    new_content = pattern_escape.sub(r'\"', new_content)
                    # Also fix double slashes that might have been created
                    new_content = new_content.replace('\"', '\"')
                    
                    if new_content != content:
                        with open(path, 'w', encoding='utf-8') as f:
                            f.write(new_content)
                        print(f"[FIXED] {path}")
                        total_fixed += 1
                except Exception as e:
                    print(f"[ERROR] Could not process {path}: {e}")
    
    print(f"Total files fixed: {total_fixed}")

if __name__ == "__main__":
    fix_syntax_errors(".")
