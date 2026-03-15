import os
import ast
import textwrap
import re

# --- CONFIGURATION (PHASE OMEGA EXTREME) ---
ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'evidence_vault', 'SOVEREIGN_DISTRO_IMG', 'artifacts', '.gemini'}
# NEVER split these to avoid infinite recursion
PROTECTED_FILES = {'__init__.py', 'sigma_refactor_pro.py', 'modularize_all.py', 'modularize_v3.py', 'modularize_v4.py', 'fix_recursion.py', 'integrity_healer.py'}

# Names to sanitize
PERSONAL_DATA = ["Aaryan", "Chauhan"]

def sanitize_text(text):
    for name in PERSONAL_DATA:
        text = re.sub(r'\b' + re.escape(name) + r'\b', 'SigmaUser', text, flags=re.IGNORECASE)
    # Remove vulgarity fallback
    text = text.replace("shit", "substandard").replace("fuck", "corrupt")
    return text

def process_file(filepath):
    rel_path = os.path.relpath(filepath, ROOT)
    filename = os.path.basename(filepath)
    
    if any(skip in rel_path for skip in SKIP_DIRS) or filename in PROTECTED_FILES:
        return

    # CRITICAL: RECURSION PREVENTION
    # If the file stem is already in the path twice, STOP.
    parts = rel_path.split(os.sep)
    stem = os.path.splitext(filename)[0]
    if parts.count(stem) >= 1 and "_core" in filename:
        return
    
    # If file is inside a folder named exactly after it, it is likely a shim we shouldn't re-split
    if len(parts) > 1 and parts[-2].lower() == stem.lower() and not filename.startswith("_"):
        return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
    except:
        return
    
    source = sanitize_text(source)
    
    try:
        tree = ast.parse(source)
    except:
        return

    # Extract top-level items
    items = [n for n in tree.body if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))]
    
    if not items:
        return

    # If it's already a single-item module, just sanitize and move on
    if len(items) == 1 and not isinstance(items[0], ast.ClassDef):
        # We might still want to add resilience
        return 

    print(f"Modularizing (Omega): {rel_path}")
    base_dir = os.path.splitext(filepath)[0]
    os.makedirs(base_dir, exist_ok=True)
    
    # Extract imports
    imports = [n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom))]
    common_imports_src = "\n".join(ast.unparse(i) for i in imports)
    
    # Resilience Import
    resilience_import = "from sigma_core.security.resilience_guard import resilient_module # noqa\n"
    common_imports_src = resilience_import + common_imports_src

    exported_names = []

    for node in items:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(base_dir, f"{name}.py")
            
            # Add resilience decorator
            has_deco = any(isinstance(d, ast.Name) and d.id == 'resilient_module' for d in node.decorator_list)
            if not has_deco:
                 node.decorator_list.insert(0, ast.Name(id='resilient_module', ctx=ast.Load()))

            content = f"# SigmaOS Omega Module: {name}\n{common_imports_src}\n\n{ast.unparse(node)}"
            with open(target, 'w', encoding='utf-8') as f:
                f.write(content)
            exported_names.append(name)
            
        elif isinstance(node, ast.ClassDef):
            cls_name = node.name
            cls_dir = os.path.join(base_dir, cls_name.lower())
            os.makedirs(cls_dir, exist_ok=True)
            
            core_path = os.path.join(cls_dir, "_core.py")
            # Create a core file with class structure and non-method items
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            core_node.body = [n for n in node.body if not isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
            if not core_node.body: core_node.body = [ast.Pass()]
            
            with open(core_path, 'w', encoding='utf-8') as f:
                f.write(f"# SigmaOS Omega Class Core: {cls_name}\n{common_imports_src}\n\n{ast.unparse(core_node)}")
            
            # Methods
            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = item.name
                    m_path = os.path.join(cls_dir, f"{m_name}.py")
                    
                    # Add resilience decorator
                    has_deco = any(isinstance(d, ast.Name) and d.id == 'resilient_module' for d in item.decorator_list)
                    if not has_deco:
                         item.decorator_list.insert(0, ast.Name(id='resilient_module', ctx=ast.Load()))

                    m_content = f"# SigmaOS Omega Method: {cls_name}.{m_name}\n{common_imports_src}\n\nclass {cls_name}:\n"
                    m_content += textwrap.indent(ast.unparse(item), "    ")
                    with open(m_path, 'w', encoding='utf-8') as f:
                        f.write(m_content)
            
            exported_names.append(cls_name)

    # SHIM GENERATION (Safe)
    shim_content = f'"""\nSigmaOS Omega Modular Shim\n"""\n'
    for name in exported_names:
        if os.path.isdir(os.path.join(base_dir, name.lower())):
             shim_content += f"from .{stem}.{name.lower()}._core import {name} # noqa\n"
        else:
             shim_content += f"from .{stem}.{name} import {name} # noqa\n"
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(shim_content)

if __name__ == "__main__":
    print("SigmaOS Phase Omega (Deep Modularization) starting...")
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith('.py'):
                process_file(os.path.join(root, file))
    print("SigmaOS Phase Omega Complete.")
