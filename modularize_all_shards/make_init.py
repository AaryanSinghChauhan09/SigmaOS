# SigmaOS Apex Shard: make_init
import os
import ast
import sys
import textwrap

def make_init(folder: str, imports: list[str]):
    """Create/update __init__.py in folder with given import lines."""
    init_path = os.path.join(folder, '__init__.py')
    header = '"""Auto-generated __init__.py — SigmaOS deep modularizer."""\n\n'
    body = '\n'.join(imports) + '\n'
    safe_write(init_path, header + body, overwrite=True)