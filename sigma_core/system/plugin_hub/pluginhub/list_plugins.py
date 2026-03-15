# Generated method: PluginHub.list_plugins
import os
import shutil
import importlib.util

class PluginHub:
    @staticmethod
    def list_plugins():
        """Discover community-contributed tools."""
        plugins = []
        if not os.path.exists(PluginHub.PLUGIN_DIR):
            return plugins
        for f in os.listdir(PluginHub.PLUGIN_DIR):
            if f.endswith('.py') and (not f.startswith('__')):
                plugins.append(str(f).split('.')[0])
        return plugins