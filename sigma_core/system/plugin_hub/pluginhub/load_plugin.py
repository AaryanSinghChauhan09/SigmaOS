# Generated method: PluginHub.load_plugin
import os
import shutil
import importlib.util

class PluginHub:
    @staticmethod
    def load_plugin(name):
        """Dynamic isolation of community code."""
        path = os.path.join(PluginHub.PLUGIN_DIR, f'{name}.py')
        if os.path.exists(path):
            spec = importlib.util.spec_from_file_location(name, path)
            if spec and hasattr(spec, 'loader'):
                ldr = spec.loader
                if ldr:
                    module = importlib.util.module_from_spec(spec)
                    ldr.exec_module(module)
                    return module
        return None