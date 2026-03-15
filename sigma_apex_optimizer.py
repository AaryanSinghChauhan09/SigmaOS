import os
import ast
import textwrap
import re
import sys

# --- SIGMA OMNI-SOVEREIGN OPTIMIZER v4.4 ---
# Features: Sibling Dependency Resolution | Constant Sharding | Lazy Loading

ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'artifacts', '.gemini', 'tests', 'docs'}
PROTECTED_FILES = {
    'sigma_apex_optimizer.py', 
    'sigma_pack.py',
    'sigma_sovereign_audit.py',
    'import_test.py',
    '__init__.py', 
    'setup.py',
    'requirements.txt'
}

REX = {
    'PII': re.compile(r'\baaryan\b|\bchauhan\b', re.I),
    'REL': re.compile(r'\bgod\b|\blord\b|\bfaith\b|\bspirit\b|\bholy\b|\bprayer\b|\bdivine\b|\bworship\b', re.I),
    'VUL': re.compile(r'\bshit\b|\bfuck\b|\bbitch\b|\bdamn\b|\bcrap\b', re.I)
}

def sanitize(text):
    text = REX['PII'].sub('SigmaSovereign', text)
    text = REX['REL'].sub('Universal', text)
    text = REX['VUL'].sub('Substandard', text)
    return text

class OmniOptimizer(ast.NodeTransformer):
    def visit_ClassDef(self, node):
        has_slots = any(isinstance(n, ast.Assign) and 
                        any(isinstance(target, ast.Name) and target.id == '__slots__' for target in n.targets) 
                        for n in node.body)
        if not has_slots:
            attrs = set()
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)) and sub.name == "__init__":
                    for instr in sub.body:
                        if isinstance(instr, ast.Assign):
                            for target in instr.targets:
                                if isinstance(target, ast.Attribute) and isinstance(target.value, ast.Name) and target.value.id == "self":
                                    attrs.add(target.attr)
            if attrs:
                slots = ast.Assign(
                    targets=[ast.Name(id='__slots__', ctx=ast.Store())],
                    value=ast.Tuple(elts=[ast.Constant(value=a) for a in sorted(list(attrs))], ctx=ast.Load())
                )
                ast.copy_location(slots, node)
                node.body.insert(0, slots)
        return self.generic_visit(node)

    def visit_FunctionDef(self, node):
        if any(x in node.name.lower() for x in ['compute', 'calculate', 'analyze', 'search', 'verify', 'process']):
            if not any(isinstance(d, ast.Call) and getattr(d.func, 'id', '') == 'lru_cache' for d in node.decorator_list):
                dec = ast.Call(func=ast.Name(id='lru_cache', ctx=ast.Load()), args=[ast.Constant(value=128)], keywords=[])
                ast.copy_location(dec, node)
                node.decorator_list.append(dec)
        return self.generic_visit(node)

def get_abs_pkg(path):
    rel = os.path.relpath(path, ROOT)
    if rel == '.': return "sigma_core"
    parts = rel.split(os.sep)
    return ".".join(parts[:-1])

def resolve_imports(tree, pkg):
    imports = []
    has_lru = False
    for node in tree.body:
        if isinstance(node, (ast.Import, ast.ImportFrom)):
            if isinstance(node, ast.ImportFrom) and node.level > 0:
                parts = pkg.split('.')
                if not parts or parts == ['']: base = "sigma_core"
                else: base = ".".join(parts[:max(0, len(parts)-(node.level-1))])
                mod = f"{base}.{node.module}" if node.module else base
                node = ast.ImportFrom(module=mod, names=node.names, level=0)
            imports.append(ast.unparse(node))
            if 'lru_cache' in ast.unparse(node): has_lru = True
    return imports, has_lru

def shard_and_optimize(filepath):
    filename = os.path.basename(filepath)
    rel = os.path.relpath(filepath, ROOT)
    if filename in PROTECTED_FILES: return
    if any(s.endswith("_shards") for s in rel.split(os.sep)): return
    if any(s in rel.split(os.sep) for s in SKIP_DIRS): return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            src = f.read()
            if "SigmaOS Apex Optimized Shim" in src: return
            tree = ast.parse(src)
    except:
        return

    pkg = get_abs_pkg(filepath)
    tree = OmniOptimizer().visit(tree)
    
    # Identify sisters for dependency injection
    sisters = {n.name: n for n in tree.body if isinstance(n, (ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef))}
    constants = [n for n in tree.body if isinstance(n, ast.Assign) and 
                 all(isinstance(t, ast.Name) and t.id.isupper() for t in n.targets)]
    for c in constants:
        for t in c.targets: sisters[t.id] = c

    print(f"[SHARDING] {rel}")
    shard_root = os.path.splitext(filepath)[0] + "_shards"
    os.makedirs(shard_root, exist_ok=True)
    with open(os.path.join(shard_root, "__init__.py"), 'w') as f: pass
    
    imports, has_lru = resolve_imports(tree, pkg)
    if any('lru_cache' in ast.unparse(n) for n in tree.body) and not has_lru:
        imports.insert(0, "from functools import lru_cache")
    
    import_header = "\n".join(imports)
    shard_dir_name = os.path.basename(shard_root)
    shims = []

    # 1. Constants
    if constants:
        c_src = f"{import_header}\n\n" + "\n".join(ast.unparse(c) for c in constants)
        with open(os.path.join(shard_root, "constants.py"), 'w', encoding='utf-8') as f: f.write(sanitize(c_src))
        for c in constants:
            for t in c.targets: shims.append(f"from .{shard_dir_name}.constants import {t.id}")

    # Helper to find sibling dependencies in a node
    def get_sibling_imports(node, current_name=None):
        needed = set()
        for sub in ast.walk(node):
            if isinstance(sub, ast.Name) and isinstance(sub.ctx, ast.Load):
                if sub.id in sisters and sub.id != current_name:
                    needed.add(sub.id)
        
        sib_lines = []
        for name in needed:
            target_node = sisters[name]
            if isinstance(target_node, ast.Assign):
                sib_lines.append(f"from ..constants import {name}")
            elif isinstance(target_node, ast.ClassDef):
                sib_lines.append(f"from ..{name.lower()}._base import {name}")
            else:
                sib_lines.append(f"from ..{name} import {name}")
        return "\n".join(sib_lines)

    # 2. Functions & Classes
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(shard_root, f"{name}.py")
            sibs = get_sibling_imports(node, name)
            content = f"{import_header}\n{sibs}\n\n{ast.unparse(node)}"
            with open(target, 'w', encoding='utf-8') as f: f.write(sanitize(content))
            shim = (f"def {name}(*args, **kwargs):\n"
                    f"    import importlib\n"
                    f"    mod = importlib.import_module('{pkg}.{shard_dir_name}.{name}')\n"
                    f"    return getattr(mod, '{name}')(*args, **kwargs)")
            shims.append(shim)

        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_dir = os.path.join(shard_root, cls_name.lower())
            os.makedirs(cls_dir, exist_ok=True)
            with open(os.path.join(cls_dir, "__init__.py"), 'w') as f: pass
            
            sibs = get_sibling_imports(node, cls_name)
            core = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            for sub in node.body:
                if not isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)): core.body.append(sub)
            if not core.body: core.body = [ast.Pass()]
            
            with open(os.path.join(cls_dir, "_base.py"), 'w', encoding='utf-8') as f:
                f.write(sanitize(f"{import_header}\n{sibs}\n\n{ast.unparse(core)}"))
            
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = sub.name
                    m_sibs = get_sibling_imports(sub, cls_name)
                    m_src = f"{import_header}\n{m_sibs}\nfrom ._base import {cls_name}\n\nclass {cls_name}:\n" + textwrap.indent(ast.unparse(sub), "    ")
                    with open(os.path.join(cls_dir, f"{m_name}.py"), 'w', encoding='utf-8') as f:
                        f.write(sanitize(m_src))
            
            shims.append(f"from .{shard_dir_name}.{cls_name.lower()}._base import {cls_name}")

    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f"\"\"\"\nSigmaOS Apex Optimized Shim (v4.4)\n\"\"\"\n" + "\n".join(shims) + "\n")

def main():
    print("--- SigmaOS Omni-Sovereign Engine v4.4 ---")
    for root, dirs, files in os.walk(ROOT):
        if any(d.endswith("_shards") for d in root.split(os.sep)): continue
        for d in list(dirs):
            if d.endswith("_shards") or d in SKIP_DIRS: dirs.remove(d)
        for file in files:
            if file.endswith('.py'):
                shard_and_optimize(os.path.join(root, file))
    print("--- Modularity & Dependency Sovereignty Peak. ---")

if __name__ == "__main__":
    main()
