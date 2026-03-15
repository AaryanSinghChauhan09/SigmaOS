"""sigma_forge.forge_app — App scaffold generator."""
import os


def forge_app(name: str, output_dir: str = "userland/apps") -> str:
    """Generate a new SigmaOS app scaffold."""
    class_name = "".join(x.capitalize() for x in name.replace("_", " ").split())
    content = f'''"""
{name} Application for SigmaOS
"""
from sigma_core.system.interfaces import SigmaModuleBase

class {class_name}(SigmaModuleBase):
    def __init__(self, kernel=None):
        super().__init__(kernel)
        self.app_id = "{name}_v1"

    def run(self, *args, **kwargs):
        print(f"[{{self.app_id}}] Execution starting...")
        return "SUCCESS"

    def health_check(self):
        return f"OK - {{self.app_id}} ACTIVE"
'''
    return _write(name, output_dir, content, "app")


def _write(name: str, output_dir: str, content: str, kind: str) -> str:
    os.makedirs(output_dir, exist_ok=True)
    filename = f"{name.lower()}.py"
    target = os.path.join(output_dir, filename)
    if os.path.exists(target):
        return f"Error: '{target}' already exists. Forge aborted."
    with open(target, "w") as f:
        f.write(content)
    return f"Forge SUCCESS: Created {kind} '{name}' at {target}"
