"""
Fix corrupted escape sequences in sigma_gui.py.
Replaces literal \\ followed by \" with just " in contexts where they appear as Python code.
Also syncs SOVEREIGN_DISTRO_IMG/sigma_gui.py.
"""
import re, shutil, ast, sys, os

def fix_file(path):
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()

    # Pattern: Replace \" that appear as escaped backslash-quote OUTSIDE of actual strings
    # These are literal \\ followed by \" in source text (i.e., the file contains \" as 3 chars)
    # Strategy: replace every occurrence of \" with just "
    fixed = content.replace('\"', '"')

    try:
        ast.parse(fixed)
        print(f"[OK] {path}: Parsed successfully after fix.")
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        return True
    except SyntaxError as e:
        print(f"[FAIL] {path} still has SyntaxError at line {e.lineno}: {e.msg}")
        print(f"  Text: {e.text}")
        # Still save — we may have fixed many but not all
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        return False

if __name__ == "__main__":
    targets = [
        "sigma_gui.py",
        "SOVEREIGN_DISTRO_IMG/sigma_gui.py",
    ]
    for t in targets:
        if os.path.exists(t):
            fix_file(t)
        else:
            print(f"[SKIP] {t} not found")
