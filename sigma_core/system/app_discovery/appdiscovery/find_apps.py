# Generated method: AppDiscovery.find_apps
import os
import importlib.util

class AppDiscovery:
    @staticmethod
    def find_apps(directory='userland/apps', prefix='ncert_'):
        """Automated discovery of labs and utilities."""
        apps = {}
        if not os.path.exists(directory):
            return apps
        for filename in os.listdir(directory):
            if filename.startswith(prefix) and filename.endswith('.py'):
                mod_name = str(filename).split('.')[0]
                clean_name = mod_name.replace(prefix, '').replace('_', ' ').title()
                apps[clean_name] = mod_name
        return apps