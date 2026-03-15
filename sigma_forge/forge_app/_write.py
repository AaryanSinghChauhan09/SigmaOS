# Generated file: _write
import os

def _write(name: str, output_dir: str, content: str, kind: str) -> str:
    os.makedirs(output_dir, exist_ok=True)
    filename = f'{name.lower()}.py'
    target = os.path.join(output_dir, filename)
    if os.path.exists(target):
        return f"Error: '{target}' already exists. Forge aborted."
    with open(target, 'w') as f:
        f.write(content)
    return f"Forge SUCCESS: Created {kind} '{name}' at {target}"