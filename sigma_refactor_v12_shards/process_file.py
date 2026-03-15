from functools import lru_cache
import os
import ast
import textwrap
import re
from ..constants import ROOT
from ..constants import PROTECTED_FILES
from ..sanitize import sanitize
from ..constants import PROTECTED_DIRS
from ..constants import SKIP_DIRS

@lru_cache(128)
def process_file(filepath):
    filename = os.path.basename(filepath)
    rel = os.path.relpath(filepath, ROOT)
    if filename in PROTECTED_FILES:
        return
    if any((p in rel.split(os.sep) for p in PROTECTED_DIRS)):
        return
    if any((s in rel for s in SKIP_DIRS)):
        return
    if '_shards' in rel:
        return
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            src = f.read()
            if not src.strip():
                return
            tree = ast.parse(src)
    except:
        return
    items = [n for n in tree.body if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))]
    if not items:
        return
    print(f'Propagating Sovereignty (v12): {rel}')
    shard_dir = os.path.splitext(filepath)[0] + '_shards'
    os.makedirs(shard_dir, exist_ok=True)
    imports = [n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom))]
    import_src = '\n'.join((ast.unparse(i) for i in imports))
    shims = []
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(shard_dir, f'{name}.py')
            content = f'# SigmaOS Apex Shard: {name}\n{import_src}\n\n{ast.unparse(node)}'
            with open(target, 'w', encoding='utf-8') as f:
                f.write(sanitize(content))
            shims.append(f'from ._shards.{name} import {name}')
        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_dir = os.path.join(shard_dir, cls_name.lower())
            os.makedirs(cls_dir, exist_ok=True)
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            core_node.body = [n for n in node.body if not isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
            if not core_node.body:
                core_node.body = [ast.Pass()]
            with open(os.path.join(cls_dir, '_base.py'), 'w', encoding='utf-8') as f:
                f.write(sanitize(f'{import_src}\n\n{ast.unparse(core_node)}'))
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = sub.name
                    m_target = os.path.join(cls_dir, f'{m_name}.py')
                    m_src = f'{import_src}\n\nclass {cls_name}:\n' + textwrap.indent(ast.unparse(sub), '    ')
                    with open(m_target, 'w', encoding='utf-8') as f:
                        f.write(sanitize(m_src))
            shims.append(f'from ._shards.{cls_name.lower()}._base import {cls_name}')
    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f'"""\nSigmaOS Apex Shim (v12.0)\n"""\n' + '\n'.join(shims) + '\n')