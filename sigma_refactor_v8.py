import os
import ast
import textwrap
import re
import hashlib

# --- SIGMA OMEGA CONFIG v8.0 ---
ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'SOVEREIGN_DISTRO_IMG', 'artifacts', '.gemini'}
PROTECTED = {'__init__.py', 'sigma_refactor_pro.py', 'modularize_all.py', 'sigma_refactor_v7.py', 'sigma_refactor_v8.py', 'shard_healer.py'}

# Sanitization
PERSONAL = re.compile(r'\baaryan\b|\bchauhan\b', re.I)
RELIGIOUS = re.compile(r'\bgod\b|\blord\b|\bfaith\b|\bspirit\b|\bholy\b|\bprayer\b', re.I)
VULGAR = re.compile(r'\bshit\b|\bfuck\b|\bbitch\b|\bdamn\b', re.I)

def sanitize(text):
    text = PERSONAL.sub('SigmaSovereign', text)
    text = RELIGIOUS.sub('Universal', text)
    text = VULGAR.sub('Substandard', text)
    return text

def get_safe_shard_dir(base_path):
    """Prevents Filename too long by hashes if path depth > 5."""
    parts = base_path.split(os.sep)
    if len(parts) > 6:
        h = hashlib.md5(base_path.encode()).hexdigest()[:8]
        parent = os.path.dirname(base_path)
        name = os.path.basename(base_path)
        return os.path.join(parent, f"s_{h}")
    return base_path + "_shards"

def process_file(filepath):
    rel = os.path.relpath(filepath, ROOT)
    if any(s in rel for s in SKIP_DIRS) or os.path.basename(filepath) in PROTECTED:
        return
    if "_shards" in rel or "_module_shard" in rel:
        return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            src = f.read()
            tree = ast.parse(src)
    except:
        return

    shard_dir = get_safe_shard_dir(os.path.splitext(filepath)[0])
    os.makedirs(shard_dir, exist_ok=True)
    
    imports = [n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom))]
    import_src = "\n".join(ast.unparse(i) for i in imports)
    
    shims = []
    
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(shard_dir, f"{name}.py")
            content = f"# SigmaOS Shard: {name}\n{import_src}\n\n{ast.unparse(node)}"
            with open(target, 'w', encoding='utf-8') as f:
                f.write(sanitize(content))
            # Resolve relative import for shim
            rel_shard = os.path.basename(shard_dir)
            shims.append(f"from .{rel_shard}.{name} import {name} # noqa")

        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_dir = os.path.join(shard_dir, cls_name.lower())
            os.makedirs(cls_dir, exist_ok=True)
            
            # Base/Core Shard
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            core_node.body = [n for n in node.body if not isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
            if not core_node.body: core_node.body = [ast.Pass()]
            
            with open(os.path.join(cls_dir, "_base.py"), 'w', encoding='utf-8') as f:
                f.write(sanitize(f"{import_src}\n\n{ast.unparse(core_node)}"))
            
            # Methods
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = sub.name
                    m_target = os.path.join(cls_dir, f"{m_name}.py")
                    m_src = f"{import_src}\n\nclass {cls_name}:\n" + textwrap.indent(ast.unparse(sub), "    ")
                    with open(m_target, 'w', encoding='utf-8') as f:
                        f.write(sanitize(m_src))
            
            rel_shard = os.path.basename(shard_dir)
            shims.append(f"from .{rel_shard}.{cls_name.lower()}._base import {cls_name} # noqa")

    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f'"""\nSigmaOS Sovereign Shim (v8.0)\n"""\n' + "\n".join(shims) + "\n")

if __name__ == "__main__":
    print("SigmaOS Sovereign Refactor v8.0 - Initializing...")
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith('.py'):
                process_file(os.path.join(root, file))
    print("Sovereign Refactor Complete.")
