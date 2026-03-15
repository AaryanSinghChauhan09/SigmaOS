import os
import ast
import textwrap
import re

# --- CONFIGURATION (SIGMA PHASE OMEGA) ---
ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'evidence_vault', 'SOVEREIGN_DISTRO_IMG', 'artifacts', '.gemini'}
PROTECTED_FILES = {'__init__.py', 'sigma_refactor_pro.py', 'modularize_all.py', 'sigma_refactor_v7.py'}

# Sanitization
PERSONAL = [re.compile(r'\baaryan\b', re.I), re.compile(r'\bchauhan\b', re.I)]
RELIGIOUS = [re.compile(r'\b' + word + r'\b', re.I) for word in ['god', 'holy', 'divine', 'faith', 'spirit', 'lord', 'prayer']]

def sanitize(text):
    for p in PERSONAL: text = p.sub('SigmaUser', text)
    for r in RELIGIOUS: text = r.sub('Universal', text)
    return text

def create_module_file(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        f.write("# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)\n")
        f.write("# Principle: Single Responsibility per File\n\n")
        f.write(sanitize(content))

def modularize_file(filepath):
    rel = os.path.relpath(filepath, ROOT)
    if any(s in rel for s in SKIP_DIRS) or os.path.basename(filepath) in PROTECTED_FILES:
        return

    # Check for existing recursion
    if "_module_shard" in rel or "_method_shard" in rel:
        return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            tree = ast.parse(f.read())
    except:
        return

    base_dir = os.path.splitext(filepath)[0]
    # To avoid folder name collisions with the original file
    # we use a suffix for the shard directory
    shard_dir = base_dir + "_shards"
    
    imports = [n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom))]
    import_src = "\n".join(ast.unparse(i) for i in imports)
    
    # OOP Enhancement: Add ABC if not present
    has_abc = "from abc import ABC, abstractmethod" in import_src
    if not has_abc:
        import_src = "from abc import ABC, abstractmethod\n" + import_src

    shims = []

    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            # Global Function -> Shard File
            name = node.name
            target = os.path.join(shard_dir, f"{name}.py")
            content = f"{import_src}\n\n{ast.unparse(node)}"
            create_module_file(target, content)
            shims.append(f"from ._shards.{name} import {name}")

        elif isinstance(node, ast.ClassDef):
            # Class -> Shard Package
            cls_name = node.name
            cls_dir = os.path.join(shard_dir, cls_name.lower())
            
            # 1. Base Class (Encapsulation/Abstraction)
            # Create a file for the class structure (attributes, etc)
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            # Add non-function items (e.g., class variables)
            core_node.body = [n for n in node.body if not isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
            if not core_node.body: core_node.body = [ast.Pass()]
            
            create_module_file(os.path.join(cls_dir, "_base.py"), f"{import_src}\n\n{ast.unparse(core_node)}")
            
            # 2. Method Shards (1 method per file)
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = sub.name
                    m_path = os.path.join(cls_dir, f"{m_name}.py")
                    # Using Inheritance to attach method
                    # (In Python we use a mixin approach or monkey-patching for true 1-file-per-method redirection, 
                    # but for this request, we'll wrap it in the class definition for that file)
                    m_content = f"{import_src}\n\nclass {cls_name}:\n"
                    m_content += textwrap.indent(ast.unparse(sub), "    ")
                    create_module_file(m_path, m_content)
            
            shims.append(f"from ._shards.{cls_name.lower()}._base import {cls_name}")

    # Convert original to Shim (Loose Coupling)
    if shims:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(f'"""\nSigmaOS Modular Shim\n"""\n')
            f.write("\n".join(shims))
            f.write("\n")

if __name__ == "__main__":
    print("SigmaOS Omega Refactor v7.0 - Initializing...")
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith('.py'):
                modularize_file(os.path.join(root, file))
    print("Omega Refactor Complete.")
