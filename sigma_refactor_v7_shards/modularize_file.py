# SigmaOS Apex Shard: modularize_file
import os
import ast
import textwrap
import re

def modularize_file(filepath):
    rel = os.path.relpath(filepath, ROOT)
    if any((s in rel for s in SKIP_DIRS)) or os.path.basename(filepath) in PROTECTED_FILES:
        return
    if '_module_shard' in rel or '_method_shard' in rel:
        return
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            tree = ast.parse(f.read())
    except:
        return
    base_dir = os.path.splitext(filepath)[0]
    shard_dir = base_dir + '_shards'
    imports = [n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom))]
    import_src = '\n'.join((ast.unparse(i) for i in imports))
    has_abc = 'from abc import ABC, abstractmethod' in import_src
    if not has_abc:
        import_src = 'from abc import ABC, abstractmethod\n' + import_src
    shims = []
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(shard_dir, f'{name}.py')
            content = f'{import_src}\n\n{ast.unparse(node)}'
            create_module_file(target, content)
            shims.append(f'from ._shards.{name} import {name}')
        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_dir = os.path.join(shard_dir, cls_name.lower())
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            core_node.body = [n for n in node.body if not isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
            if not core_node.body:
                core_node.body = [ast.Pass()]
            create_module_file(os.path.join(cls_dir, '_base.py'), f'{import_src}\n\n{ast.unparse(core_node)}')
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = sub.name
                    m_path = os.path.join(cls_dir, f'{m_name}.py')
                    m_content = f'{import_src}\n\nclass {cls_name}:\n'
                    m_content += textwrap.indent(ast.unparse(sub), '    ')
                    create_module_file(m_path, m_content)
            shims.append(f'from ._shards.{cls_name.lower()}._base import {cls_name}')
    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f'"""\nSigmaOS Modular Shim\n"""\n')
            f.write('\n'.join(shims))
            f.write('\n')