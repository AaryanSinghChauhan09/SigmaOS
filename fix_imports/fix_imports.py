# Generated file: fix_imports
import os
from pathlib import Path

def fix_imports():
    for f in root.rglob('*.py'):
        try:
            content = f.read_text('utf-8')
            new_content = content
            new_content = new_content.replace('from ', 'from ')
            if 'import kernel.' in new_content:
                pass
            if new_content != content:
                f.write_text(new_content, 'utf-8')
                print(f'Fixed imports in {f.relative_to(root)}')
        except Exception as e:
            print(f'Could not process {f}: {e}')