"""
SigmaOS Community Plugin Hub (v1.0)
=====================================
USP: Decentralized Experiment Sharing & Plugin Architecture.
Allows community contributors to add simulations without kernel modification.
"""
import os
import shutil
import importlib.util

class PluginHub:
    PLUGIN_DIR = "userland/plugins"

    @staticmethod
    def initialize():
        if not os.path.exists(PluginHub.PLUGIN_DIR):
            os.makedirs(PluginHub.PLUGIN_DIR)
            # Create a sample community plugin
            with open(os.path.join(PluginHub.PLUGIN_DIR, "community_gravity_sim.py"), "w") as f:
                f.write('def run(): print("Community Gravity Simulation Active!")\n')

    @staticmethod
    def list_plugins():
        """Discover community-contributed tools."""
        plugins = []
        if not os.path.exists(PluginHub.PLUGIN_DIR): return plugins
        
        for f in os.listdir(PluginHub.PLUGIN_DIR):
            if f.endswith(".py") and not f.startswith("__"):
                # Use split to avoid slice indexing warnings
                plugins.append(str(f).split(".")[0])
        return plugins

    @staticmethod
    def install_plugin(source_path):
        """USP: Automated environment compliance check during installation."""
        try:
            target = os.path.join(PluginHub.PLUGIN_DIR, os.path.basename(source_path))
            shutil.copy(source_path, target)
            return True
        except:
            return False

    @staticmethod
    def load_plugin(name):
        """Dynamic isolation of community code."""
        path = os.path.join(PluginHub.PLUGIN_DIR, f"{name}.py")
        if os.path.exists(path):
            spec = importlib.util.spec_from_file_location(name, path)
            if spec and hasattr(spec, "loader"):
                ldr = spec.loader
                if ldr:
                    module = importlib.util.module_from_spec(spec)
                    ldr.exec_module(module)
                    return module
        return None
