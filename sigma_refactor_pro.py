import os
import ast
import textwrap
import re

# --- CONFIGURATION ---
ROOT = os.getcwd()
SKIP_DIRS = {'.git', '__pycache__', 'node_modules', 'evidence_vault', 'SOVEREIGN_DISTRO_IMG', 'artifacts', '.gemini'}
SKIP_FILES = {'sigma_refactor_pro.py', 'modularize_all.py', 'modularize_v3.py', 'modularize_v4.py'}

# Keywords to sanitize
RELIGIOUS_VULGAR_KEYWORDS = [
    'god', 'lord', 'faith', 'spirit', 'holy', 'divine', 'bless', 'curse', 'chaos', 'Optimized State',
    'bible', 'quran', 'gita', 'temple', 'church', 'mosque'
]

# Personal Data to mask
# We'll also use regex to catch hardcoded user home paths
PERSONAL_DATA_RE = [
    (re.compile(r'C:\\Users\\[a-zA-Z0-9]+', re.IGNORECASE), r'C:\\Users\\SigmaUser'),
    (re.compile(r'/home/[a-zA-Z0-9]+', re.IGNORECASE), r'/home/sigmauser'),
    (re.compile(r'SigmaUser', re.IGNORECASE), 'SigmaUser'),
    (re.compile(r'SigmaDeveloper', re.IGNORECASE), 'SigmaDeveloper')
]

def sanitize_text(text):
    # Mask personal data and paths
    for pattern, replacement in PERSONAL_DATA_RE:
        text = pattern.sub(replacement, text)
    
    # Genericize religious terms
    def replacer(match):
        word = match.group(0)
        if word.lower() in RELIGIOUS_VULGAR_KEYWORDS:
            return "Universal"
        return word

    return re.sub(r'\b\w+\b', replacer, text)

class DeepModularizer(ast.NodeTransformer):
    def __init__(self, filename, rel_path):
        self.filename = filename
        self.rel_path = rel_path
        self.imports = []
        self.items_to_save = [] # List of (type, name, node, class_name)
    
    def visit_Import(self, node):
        self.imports.append(node)
        return node

    def visit_ImportFrom(self, node):
        self.imports.append(node)
        return node
    
    def visit_FunctionDef(self, node):
        self.items_to_save.append(('func', node.name, node, None))
        return None 

    def visit_AsyncFunctionDef(self, node):
        self.items_to_save.append(('async_func', node.name, node, None))
        return None

    def visit_ClassDef(self, node):
        class_name = node.name
        new_class_body = []
        for item in node.body:
            if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                self.items_to_save.append(('method', item.name, item, class_name))
            else:
                new_class_body.append(item)
        
        node.body = new_class_body
        self.items_to_save.append(('class_shim', node.name, node, None))
        return None

def process_file(filepath):
    rel_path = os.path.relpath(filepath, ROOT)
    if any(skip in rel_path for skip in SKIP_DIRS) or os.path.basename(filepath) in SKIP_FILES:
        return

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            source = f.read()
    except:
        return # Skip binary or unreadable
    
    source = sanitize_text(source)
    
    try:
        tree = ast.parse(source)
    except:
        return

    dm = DeepModularizer(filepath, rel_path)
    dm.visit(tree)
    
    if not dm.items_to_save:
        # Just update the original with sanitized text
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(source)
        return

    base_dir = os.path.splitext(filepath)[0]
    os.makedirs(base_dir, exist_ok=True)
    
    common_imports_src = "\n".join(ast.unparse(i) for i in dm.imports)
    
    exported_names = []

    for type, name, node, class_name in dm.items_to_save:
        if type in ('func', 'async_func'):
            file_path = os.path.join(base_dir, f"{name}.py")
            content = f"# Generated file: {name}\n{common_imports_src}\n\n{ast.unparse(node)}"
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            exported_names.append(name)
        elif type == 'method':
            class_dir = os.path.join(base_dir, class_name.lower())
            os.makedirs(class_dir, exist_ok=True)
            file_path = os.path.join(class_dir, f"{name}.py")
            content = f"# Generated method: {class_name}.{name}\n{common_imports_src}\n\nclass {class_name}:\n"
            content += textwrap.indent(ast.unparse(node), "    ")
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
        elif type == 'class_shim':
            file_path = os.path.join(base_dir, f"_{name}_core.py")
            content = f"# Generated class core: {name}\n{common_imports_src}\n\n{ast.unparse(node)}"
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            exported_names.append(name)

    # REWRITE ORIGINAL AS SHIM
    shim_content = f'"""\nSigmaOS Modular Shim for {os.path.basename(filepath)}\n"""\n'
    for name in exported_names:
        # Simplified import logic
        if type == 'class_shim':
             shim_content += f"from .{os.path.basename(base_dir)}._{name}_core import {name} # noqa\n"
        else:
             shim_content += f"from .{os.path.basename(base_dir)}.{name} import {name} # noqa\n"
    
    if not filepath.endswith('__init__.py'):
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(shim_content)

if __name__ == "__main__":
    print("Starting Global SigmaOS Refactor Pro...")
    for root, dirs, files in os.walk(ROOT):
        dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
        for file in files:
            if file.endswith('.py'):
                process_file(os.path.join(root, file))
    print("Refactor Pro Complete.")
