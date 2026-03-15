# Generated file: fix_file
import re, shutil, ast, sys, os

def fix_file(path):
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    fixed = content.replace('"', '"')
    try:
        ast.parse(fixed)
        print(f'[OK] {path}: Parsed successfully after fix.')
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        return True
    except SyntaxError as e:
        print(f'[FAIL] {path} still has SyntaxError at line {e.lineno}: {e.msg}')
        print(f'  Text: {e.text}')
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        return False