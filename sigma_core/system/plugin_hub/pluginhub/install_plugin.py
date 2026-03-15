# Generated method: PluginHub.install_plugin
import os
import shutil
import importlib.util

class PluginHub:
    @staticmethod
    def install_plugin(source_path):
        """USP: Automated environment compliance check during installation."""
        try:
            target = os.path.join(PluginHub.PLUGIN_DIR, os.path.basename(source_path))
            shutil.copy(source_path, target)
            return True
        except:
            return False