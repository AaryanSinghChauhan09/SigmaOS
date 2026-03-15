# Generated file: forge_app
import os

def forge_app(name: str, output_dir: str='userland/apps') -> str:
    """Generate a new SigmaOS app scaffold."""
    class_name = ''.join((x.capitalize() for x in name.replace('_', ' ').split()))
    content = f'"""\n{name} Application for SigmaOS\n"""\nfrom sigma_core.system.interfaces import SigmaModuleBase\n\nclass {class_name}(SigmaModuleBase):\n    def __init__(self, kernel=None):\n        super().__init__(kernel)\n        self.app_id = "{name}_v1"\n\n    def run(self, *args, **kwargs):\n        print(f"[{{self.app_id}}] Execution starting...")\n        return "SUCCESS"\n\n    def health_check(self):\n        return f"OK - {{self.app_id}} ACTIVE"\n'
    return _write(name, output_dir, content, 'app')