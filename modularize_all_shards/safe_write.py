# SigmaOS Apex Shard: safe_write
import os
import ast
import sys
import textwrap

def safe_write(path: str, content: str, overwrite: bool=True):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if os.path.exists(path) and (not overwrite):
        print(f'  [SKIP] {path} already exists.')
        return
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f'  [WROTE] {path}')