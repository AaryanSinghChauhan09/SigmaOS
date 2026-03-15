# Generated file: fix_syntax_errors
import os
import re

def fix_syntax_errors(directory):
    """
    Fixes common syntax error patterns in SigmaOS .py files.
    - Replaces 'apps' with 'apps' when used as a variable/identifier.
    - Replaces 'web_os' with 'web_os' in identifiers.
    - Fixes corrupted escape sequences like " when they break strings.
    """
    pattern_apps = re.compile('([a-zA-Z0-9_]*)apps([a-zA-Z0-9_]*)')
    pattern_gui = re.compile('([a-zA-Z0-9_]*)web_os([a-zA-Z0-9_]*)')
    pattern_escape = re.compile('\\\\\\"')
    total_fixed = 0
    for root, _, files in os.walk(directory):
        if '.git' in root:
            continue
        for file in files:
            if file.endswith('.py'):
                path = os.path.join(root, file)
                try:
                    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                    new_content = content
                    new_content = pattern_apps.sub('\\1apps\\2', new_content)
                    new_content = pattern_gui.sub('\\1web_os\\2', new_content)
                    new_content = pattern_escape.sub('\\"', new_content)
                    new_content = new_content.replace('"', '"')
                    if new_content != content:
                        with open(path, 'w', encoding='utf-8') as f:
                            f.write(new_content)
                        print(f'[FIXED] {path}')
                        total_fixed += 1
                except Exception as e:
                    print(f'[ERROR] Could not process {path}: {e}')
    print(f'Total files fixed: {total_fixed}')