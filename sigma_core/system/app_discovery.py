"""
SigmaOS App Discovery Engine (v1.0)
=====================================
USP: Automated Module Registration & Dynamic App Loading.
Enables 'Plug-and-Play' functionality for the NCERT Lab Suite.
"""
import os
import importlib.util

class AppDiscovery:
    @staticmethod
    def find_apps(directory="userland/apps", prefix="ncert_"):
        """Automated discovery of labs and utilities."""
        apps = {}
        if not os.path.exists(directory):
            return apps

        for filename in os.listdir(directory):
            if filename.startswith(prefix) and filename.endswith(".py"):
                # Use split to avoid slice indexing warnings
                mod_name = str(filename).split(".")[0]
                # Extract meta-data by reading the docstring or a specific variable
                # For now, we'll map them to human-readable names
                clean_name = mod_name.replace(prefix, "").replace("_", " ").title()
                apps[clean_name] = mod_name
        return apps

    @staticmethod
    def launch_app(mod_name):
        """Dynamic launcher for discovered modules."""
        try:
            # We use subprocess to keep the main kernel thread isolated and stable
            import subprocess, sys
            path = f"userland/apps/{mod_name}.py"
            if os.path.exists(path):
                subprocess.Popen([sys.executable, path])
                return True
        except Exception as e:
            print(f"[ERROR] Discovery Launch Failure: {e}")
        return False
