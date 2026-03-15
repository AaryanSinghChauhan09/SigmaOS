# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, textwrap

def split_file(filepath: str) -> int:
    with open(filepath, 'rb') as f:
        raw = f.read().decode('utf-8', 'replace')
    if len(raw.encode()) < MIN_BYTES:
        return 0
    try:
        tree = ast.parse(raw, filename=filepath)
    except SyntaxError as e:
        print(f'  [SYNTAX-ERR] {filepath}: {e}')
        return 0
    rel = os.path.relpath(filepath, ROOT)
    dirname = os.path.dirname(filepath)
    stem = os.path.splitext(os.path.basename(filepath))[0]
    non_import = [n for n in tree.body if not isinstance(n, (ast.Import, ast.ImportFrom, ast.Expr, ast.Assign, ast.AnnAssign))]
    func_nodes = [n for n in non_import if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
    class_nodes = [n for n in non_import if isinstance(n, ast.ClassDef)]
    if len(func_nodes) + len(class_nodes) <= 1:
        return 0
    pkg_dir = os.path.join(dirname, stem)
    hdr = get_imports(tree)
    created: list[str] = []
    shim_exports: list[str] = []
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            content = f'# auto-split: {rel} — {name}\n' + hdr + '\n\n' + ast.unparse(node) + '\n'
            safe_write(os.path.join(pkg_dir, f'{name}.py'), content)
            created.append(name)
            shim_exports.append(name)
        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_pkg = os.path.join(pkg_dir, cls_name.lower())
            cls_exports: list[str] = []
            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    mname = item.name
                    content = f'# auto-split: {rel} — {cls_name}.{mname}\n' + hdr + f'\n\nclass {cls_name}:\n' + textwrap.indent(ast.unparse(item), '    ') + '\n'
                    safe_write(os.path.join(cls_pkg, f'{mname}.py'), content)
                    cls_exports.append(mname)
                elif isinstance(item, (ast.Assign, ast.AnnAssign)):
                    attrs_path = os.path.join(cls_pkg, '_attrs.py')
                    os.makedirs(cls_pkg, exist_ok=True)
                    with open(attrs_path, 'a', encoding='utf-8') as af:
                        af.write(ast.unparse(item) + '\n')
            if cls_exports:
                make_init(cls_pkg, cls_exports)
                created.append(cls_name.lower())
                shim_exports.append(cls_name)
    if not created:
        return 0
    make_init(pkg_dir, [s.lower() if s[0].isupper() else s for s in shim_exports])
    shim = f'# {stem}.py — backward-compat shim\n' + '\n'.join((f'from {stem}.{s} import *  # noqa' for s in shim_exports)) + '\n\n__all__ = ' + repr(shim_exports) + '\n'
    safe_write(filepath, shim)
    return len(created)