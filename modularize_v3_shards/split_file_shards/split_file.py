# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import ast, os, sys, textwrap

def split_file(filepath: str) -> int:
    """Split one .py file into a package of per-function files.
    Returns the number of new module files created."""
    with open(filepath, encoding='utf-8', errors='replace') as f:
        source = f.read()
    if len(source.encode()) < MIN_BYTES:
        return 0
    try:
        tree = ast.parse(source, filename=filepath)
    except SyntaxError as e:
        print(f'  [SYNTAX-ERR] {filepath}: {e}')
        return 0
    rel = os.path.relpath(filepath, ROOT)
    dirname = os.path.dirname(filepath)
    stem = os.path.splitext(os.path.basename(filepath))[0]
    pkg_dir = os.path.join(dirname, stem)
    hdr_imports = get_top_imports(tree, source)
    hdr_constants = get_top_constants(tree)
    created = []
    shim_exports: list[str] = []
    for node in tree.body:
        if isinstance(node, ast.FunctionDef) or isinstance(node, ast.AsyncFunctionDef):
            fname = node.name
            fn_src = ast.unparse(node)
            content = node_header_comment(rel, fname) + hdr_imports + '\n\n' + fn_src + '\n'
            out = os.path.join(pkg_dir, f'{fname}.py')
            result = safe_write(out, content, overwrite=True)
            print(result)
            created.append(fname)
            shim_exports.append(fname)
        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_pkg_dir = os.path.join(pkg_dir, cls_name.lower())
            class_exports: list[str] = []
            class_imports_src = hdr_imports
            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    method_name = item.name
                    method_src = ast.unparse(item)
                    content = node_header_comment(rel, f'{cls_name}.{method_name}') + class_imports_src + f'\n\nclass {cls_name}:\n' + textwrap.indent(method_src, '    ') + '\n'
                    out = os.path.join(cls_pkg_dir, f'{method_name}.py')
                    result = safe_write(out, content, overwrite=True)
                    print(result)
                    class_exports.append(method_name)
                elif isinstance(item, (ast.Assign, ast.AnnAssign)):
                    attrs_path = os.path.join(cls_pkg_dir, '_attrs.py')
                    attr_src = ast.unparse(item)
                    os.makedirs(cls_pkg_dir, exist_ok=True)
                    with open(attrs_path, 'a', encoding='utf-8') as af:
                        af.write(attr_src + '\n')
            if class_exports:
                result = make_init(cls_pkg_dir, class_exports)
                print(result)
                created.append(cls_name.lower())
                shim_exports.append(cls_name)
    if not created:
        return 0
    result = make_init(pkg_dir, [s.lower() if s[0].isupper() else s for s in shim_exports])
    print(result)
    shim = f'"""\n{stem}.py — backward-compat shim.\nReal implementation lives in {stem}/ package.\n"""\n\n' + '\n'.join((f'from {stem}.{s} import *  # noqa' for s in shim_exports)) + '\n\n__all__ = ' + repr(shim_exports) + '\n'
    result = safe_write(filepath, shim, overwrite=True)
    print(result)
    return len(created)