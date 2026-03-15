import os
import ast
import textwrap
import re

# --- CONFIGURATION ---
ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'evidence_vault', 'SOVEREIGN_DISTRO_IMG', 'artifacts', '.gemini'}
# NEVER split these to avoid infinite recursion
PROTECTED_FILES = {'__init__.py', 'sigma_refactor_pro.py', 'modularize_all.py', 'modularize_v3.py', 'modularize_v4.py'}

def process_file(filepath):
    rel_path = os.path.relpath(filepath, ROOT)
    filename = os.path.basename(filepath)
    
    if any(skip in rel_path for skip in SKIP_DIRS) or filename in PROTECTED_FILES:
        return

    # CRITICAL: Prevent deep recursion
    # If the file is already inside a directory of the same name, skip.
    parent_dir = os.path.basename(os.path.dirname(filepath))
    stem = os.path.splitext(filename)[0]
    if stem.lower() == parent_dir.lower():
        # print(f"Skipping already-nested file: {rel_path}")
        return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
    except:
        return
    
    try:
        tree = ast.parse(source)
    except:
        return

    # Count functions and classes at top level
    items = [n for n in tree.body if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))]
    
    if len(items) <= 1:
        # If there's 1 or 0 top-level items, it's already modular enough.
        # But we still check if the source has any logic at all.
        return 

    print(f"Modularizing: {rel_path} ({len(items)} items)")
    base_dir = os.path.splitext(filepath)[0]
    os.makedirs(base_dir, exist_ok=True)
    
    # Extract imports
    imports = [n for n in tree.body if isinstance(n, (ast.Import, ast.ImportFrom))]
    common_imports_src = "\n".join(ast.unparse(i) for i in imports)
    
    exported_names = []

    for node in items:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            name = node.name
            target = os.path.join(base_dir, f"{name}.py")
            content = f"# Generated: {name}\n{common_imports_src}\n\n{ast.unparse(node)}"
            with open(target, 'w', encoding='utf-8') as f:
                f.write(content)
            exported_names.append(name)
        elif isinstance(node, ast.ClassDef):
            # For brevity, we split the class into its own file + methods in a sub-pkg
            cls_name = node.name
            cls_dir = os.path.join(base_dir, cls_name.lower())
            os.makedirs(cls_dir, exist_ok=True)
            
            # Save core class (vitals)
            core_path = os.path.join(cls_dir, "_core.py")
            # We strip methods for the core file
            core_node = ast.ClassDef(name=cls_name, bases=node.bases, keywords=node.keywords, body=[], decorator_list=node.decorator_list)
            # Add simple pass if empty
            core_node.body = [n for n in node.body if not isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))]
            if not core_node.body: core_node.body = [ast.Pass()]
            
            with open(core_path, 'w', encoding='utf-8') as f:
                f.write(f"# Class Core: {cls_name}\n{common_imports_src}\n\n{ast.unparse(core_node)}")
            
            # Save methods
            for item in node.body:
                if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    m_name = item.name
                    m_path = os.path.join(cls_dir, f"{m_name}.py")
                    # Wrap method in class stub
                    m_content = f"# Method: {cls_name}.{m_name}\n{common_imports_src}\n\nclass {cls_name}:\n"
                    m_content += textwrap.indent(ast.unparse(item), "    ")
                    with open(m_path, 'w', encoding='utf-8') as f:
                        f.write(m_content)
            
            exported_names.append(cls_name)

    # Convert original to shim
    shim_content = f'"""\nSigmaOS Modular Shim\n"""\n'
    for name in exported_names:
        # Check if it was a class or func
        if os.path.isdir(os.path.join(base_dir, name.lower())):
             shim_content += f"from .{stem}.{name.lower()}._core import {name} # noqa\n"
        else:
             shim_content += f"from .{stem}.{name} import {name} # noqa\n"
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(shim_content)

if __name__ == "__main__":
    print("Starting SAFE Global Refactor...")
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith('.py'):
                process_file(os.path.join(root, file))
    print("SAFE Refactor Complete.")
