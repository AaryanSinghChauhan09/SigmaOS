import os
import ast
import textwrap
import re
import sys

# --- SIGMA OMNI-SOVEREIGN ENGINE v4.9 ---
# Stable Atomic Sharding | Resilient Class Method Extraction | Zero-Dependency.

ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'artifacts', '.gemini', 'tests', 'docs'}
PROTECTED_FILES = {
    'sigma_apex_optimizer.py', 'sigma_pack.py', 'sigma_sovereign_audit.py',
    'import_test.py', 'omni_prompt_demo.py', 'sigma_integrity_audit.py',
    '__init__.py', 'setup.py', 'requirements.txt'
}

REX_PII = re.compile(r'\baaryan\b|\bchauhan\b', re.I)

def sanitize(text):
    return REX_PII.sub('SigmaSovereign', text)

def get_abs_pkg(path):
    rel = os.path.relpath(path, ROOT)
    if rel == '.': return "sigma_core"
    parts = rel.split(os.sep)
    return ".".join(list(parts)[:-1])  # type: ignore

def resolve_imports(tree, pkg):
    imports = []
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            if isinstance(node, ast.ImportFrom) and node.level > 0:
                parts = pkg.split('.')
                base = ".".join(parts[:max(0, len(parts)-(node.level-1))]) if parts and parts != [''] else "sigma_core"
                node = ast.ImportFrom(module=f"{base}.{node.module}" if node.module else base, names=node.names, level=0)
            imports.append(ast.unparse(node))
    return imports

def shard_file(filepath):
    filename = os.path.basename(filepath)
    rel = os.path.relpath(filepath, ROOT)
    if filename in PROTECTED_FILES: return
    if any(s.endswith("_shards") for s in rel.split(os.sep)): return
    if any(s in rel.split(os.sep) for s in SKIP_DIRS): return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            src = f.read()
            if "SigmaOS Apex Optimized Shim (v4.9)" in src: return
            tree = ast.parse(src)
    except: return

    pkg = get_abs_pkg(filepath)
    sisters = {}
    constants = []
    
    for node in tree.body:
        if isinstance(node, (ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)):
            sisters[node.name] = node
        elif isinstance(node, ast.Assign):
            if all(isinstance(t, ast.Name) and getattr(t, 'id', '').isupper() for t in node.targets):
                constants.append(node)
                for t in node.targets:
                    if isinstance(t, ast.Name):
                        sisters[t.id] = node

    print(f"[SHARDING] {rel}")
    shard_root = os.path.splitext(filepath)[0] + "_shards"
    if not os.path.exists(shard_root): os.makedirs(shard_root)
    with open(os.path.join(shard_root, "__init__.py"), 'w') as f: f.write("")
    
    import_header = "\n".join(resolve_imports(tree, pkg))
    shard_dir_name = os.path.basename(shard_root)
    shims = []

    def get_sibs(node, cname, level):
        needed = set()
        for sub in ast.walk(node):
            if isinstance(sub, ast.Name) and isinstance(sub.ctx, ast.Load):
                if sub.id in sisters and sub.id != cname: needed.add(sub.id)
        dots = "." * level
        lines = []  # type: ignore
        for n in sorted(list(needed)):
            t = sisters[n]
            if isinstance(t, ast.Assign): lines.append(f"from {dots}constants import {n}")
            elif isinstance(t, ast.ClassDef): lines.append(f"from {dots}{n.lower()}._base import {n}")
            else: lines.append(f"from {dots}{n} import {n}")
        return "\n".join(lines)

    if constants:
        c_src = f"{import_header}\n\n" + "\n".join(ast.unparse(c) for c in constants)
        with open(os.path.join(shard_root, "constants.py"), 'w', encoding='utf-8') as f: f.write(sanitize(c_src))
        for c in constants:
            for t in c.targets:
                if isinstance(t, ast.Name):
                    shims.append(f"from .{shard_dir_name}.constants import {t.id}")

    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            n = node.name
            s = get_sibs(node, n, 1)
            with open(os.path.join(shard_root, f"{n}.py"), 'w', encoding='utf-8') as f:
                f.write(sanitize(f"{import_header}\n{s}\n\n{ast.unparse(node)}"))
            shims.append(f"def {n}(*args, **kwargs):\n    import importlib\n    mod = importlib.import_module('{pkg}.{shard_dir_name}.{n}')\n    return getattr(mod, '{n}')(*args, **kwargs)")
        elif isinstance(node, ast.ClassDef):
            n = node.name
            cd = os.path.join(shard_root, n.lower())
            if not os.path.exists(cd): os.makedirs(cd)
            with open(os.path.join(cd, "__init__.py"), 'w') as f: f.write("")
            
            bb = []
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)) and sub.name != "__init__":
                    sn = sub.name
                    shim = f"def {sn}(self, *args, **kwargs):\n    import importlib\n    mod = importlib.import_module('.{sn}', package=__package__)\n    return getattr(mod, '{sn}')(self, *args, **kwargs)"
                    bb.append(ast.parse(shim).body[0])
                else: bb.append(sub)
            
            core = ast.ClassDef(name=n, bases=node.bases, keywords=node.keywords, body=bb if bb else [ast.Pass()], decorator_list=node.decorator_list)  # type: ignore
            with open(os.path.join(cd, "_base.py"), 'w', encoding='utf-8') as f:
                f.write(sanitize(f"{import_header}\n{get_sibs(node, n, 2)}\n\n{ast.unparse(core)}"))
            
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)) and sub.name != "__init__":
                    m_src = f"{import_header}\n{get_sibs(sub, n, 2)}\n\n{ast.unparse(sub)}"
                    with open(os.path.join(cd, f"{sub.name}.py"), 'w', encoding='utf-8') as f: f.write(sanitize(m_src))
            shims.append(f"from .{shard_dir_name}.{n.lower()}._base import {n}")

    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f"\"\"\"\nSigmaOS Apex Optimized Shim (v4.9)\n\"\"\"\n" + "\n".join(shims) + "\n")

def main():
    print("--- SigmaOS Omni-Sovereign Engine v4.9 ---")
    for r, ds, fs in os.walk(ROOT):
        if any(d.endswith("_shards") for d in r.split(os.sep)): continue
        for f in fs:
            if f.endswith('.py'): shard_file(os.path.join(r, f))
    print("--- Modularity Record established. ---")

if __name__ == "__main__": main()
