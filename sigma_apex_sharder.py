import os
import ast
import textwrap
import re
import sys

# --- SIGMA APEX SHARDER v2.3 ---

ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'artifacts', '.gemini', 'tests', 'docs'}
PROTECTED_FILES = {
    'sigma_apex_sharder.py', 
    '__init__.py', 
    'sigma_refactor_v12.py',
    'sigma_absolute_imports.py',
    'setup.py',
    'requirements.txt',
    'import_test.py'
}

# Sanitization Patterns
PERSONAL_DATA = re.compile(r'\baaryan\b|\bchauhan\b', re.I)
RELIGIOUS_TERMS = re.compile(r'\bgod\b|\blord\b|\bfaith\b|\bspirit\b|\bholy\b|\bprayer\b|\bdivine\b|\bworship\b', re.I)
VULGAR_TERMS = re.compile(r'\bshit\b|\bfuck\b|\bbitch\b|\bdamn\b|\bcrap\b', re.I)

def sanitize(text):
    text = PERSONAL_DATA.sub('SigmaSovereign', text)
    text = RELIGIOUS_TERMS.sub('Universal', text)
    text = VULGAR_TERMS.sub('Substandard', text)
    return text

def get_imports(tree):
    imports = []
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            imports.append(ast.unparse(node))
    return "\n".join(imports)

def shard_file(filepath):
    filename = os.path.basename(filepath)
    rel_path = os.path.relpath(filepath, ROOT)
    
    if filename in PROTECTED_FILES: return
    if any(s in rel_path.split(os.sep) for s in SKIP_DIRS): return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
            if not source.strip(): return
            tree = ast.parse(source)
    except: return

    siblings = {}
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            siblings[node.name] = node

    if not siblings:
        process_generic_file(filepath)
        return

    print(f"[SHARDING] {rel_path}")
    shard_root = os.path.splitext(filepath)[0] + "_shards"
    os.makedirs(shard_root, exist_ok=True)
    with open(os.path.join(shard_root, "__init__.py"), 'w') as f: pass
    
    global_imports = get_imports(tree)
    shard_pkg = os.path.basename(shard_root)
    shims = []
    
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(shard_root, f"{name}.py")
            
            local_imports = []
            for sib_name in siblings:
                if sib_name != name and re.search(r'\b' + sib_name + r'\b', ast.unparse(node)):
                    sib_node = siblings[sib_name]
                    if isinstance(sib_node, ast.ClassDef):
                        local_imports.append(f"from .{shard_pkg}.{sib_name.lower()}._base import {sib_name}")
                    else:
                        local_imports.append(f"from .{shard_pkg}.{sib_name} import {sib_name}")

            content = f"\"\"\"\nSigmaOS Apex Shard: {name}\n\"\"\"\n{global_imports}\n" + "\n".join(local_imports) + f"\n\n{ast.unparse(node)}"
            with open(target, 'w', encoding='utf-8') as f:
                f.write(sanitize(content))
            shims.append(f"from .{shard_pkg}.{name} import {name}")

        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_dir = os.path.join(shard_root, cls_name.lower())
            os.makedirs(cls_dir, exist_ok=True)
            with open(os.path.join(cls_dir, "__init__.py"), 'w') as f: pass
            
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            for sub in node.body:
                if not isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    core_node.body.append(sub)
            if not core_node.body: core_node.body = [ast.Pass()]
            
            local_imports = []
            for sib_name in siblings:
                if sib_name != cls_name and re.search(r'\b' + sib_name + r'\b', ast.unparse(core_node)):
                    sib_node = siblings[sib_name]
                    if isinstance(sib_node, ast.ClassDef):
                        local_imports.append(f"from ...{shard_pkg}.{sib_name.lower()}._base import {sib_name}")
                    else:
                        local_imports.append(f"from ...{shard_pkg}.{sib_name} import {sib_name}")

            with open(os.path.join(cls_dir, "_base.py"), 'w', encoding='utf-8') as f:
                f.write(sanitize(f"{global_imports}\n" + "\n".join(local_imports) + f"\n\n{ast.unparse(core_node)}"))
            
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = sub.name
                    m_imports = [f"from ._base import {cls_name}"]
                    for sib_name in siblings:
                        if sib_name != cls_name and re.search(r'\b' + sib_name + r'\b', ast.unparse(sub)):
                            sib_node = siblings[sib_name]
                            if isinstance(sib_node, ast.ClassDef):
                                m_imports.append(f"from ..{sib_name.lower()}._base import {sib_name}")
                            else:
                                m_imports.append(f"from ..{sib_name} import {sib_name}")

                    m_src = f"{global_imports}\n" + "\n".join(m_imports) + f"\n\nclass {cls_name}:\n" + textwrap.indent(ast.unparse(sub), "    ")
                    with open(os.path.join(cls_dir, f"{m_name}.py"), 'w', encoding='utf-8') as f:
                        f.write(sanitize(m_src))
            
            shims.append(f"from .{shard_pkg}.{cls_name.lower()}._base import {cls_name}")

    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f"\"\"\"\nSigmaOS Apex Shim (v2.0)\n\"\"\"\n" + "\n".join(shims) + "\n")

def process_generic_file(filepath):
    filename = os.path.basename(filepath)
    rel_path = os.path.relpath(filepath, ROOT)
    if filename in PROTECTED_FILES: return
    if any(s in rel_path.split(os.sep) for s in SKIP_DIRS): return
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
        sanitized = sanitize(source)
        if sanitized != source:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(sanitized)
            print(f"[CLEAN] {rel_path}")
    except: pass

def main():
    print("--- SigmaOS Apex Sharder v2.3 ---")
    shardable_exts = {'.py'}
    sanitizable_exts = {'.c', '.cpp', '.h', '.rs', '.asm', '.md', '.txt', '.bat', '.sh', '.vbs', '.ovf', '.sigma'}
    for root, dirs, files in os.walk(ROOT):
        current_rel = os.path.relpath(root, ROOT)
        if any(s in current_rel.split(os.sep) for s in SKIP_DIRS): continue
        for d in list(dirs):
            if d.endswith("_shards") or d in SKIP_DIRS: dirs.remove(d)
        for file in files:
            filepath = os.path.join(root, file)
            ext = os.path.splitext(file)[1].lower()
            if ext in shardable_exts: shard_file(filepath)
            elif ext in sanitizable_exts: process_generic_file(filepath)
    print("--- Complete. ---")

if __name__ == "__main__":
    main()
