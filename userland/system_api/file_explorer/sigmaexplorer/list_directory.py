# Generated method: SigmaExplorer.list_directory
import os
import time

class SigmaExplorer:
    def list_directory(self, path: str) -> list:
        """Industry Standard: Real-time file system introspection."""
        import os
        base = os.path.dirname(os.path.dirname(__file__))
        target = os.path.normpath(os.path.join(base, path.strip('/')))
        if not os.path.exists(target):
            return [{'name': 'Error: Path not found', 'type': 'file', 'size': '0'}]
        results = []
        try:
            for item in os.listdir(target):
                full_path = os.path.join(target, item)
                is_dir = os.path.isdir(full_path)
                size = os.path.getsize(full_path) if not is_dir else 0
                results.append({'name': item, 'type': 'dir' if is_dir else 'file', 'size': f'{size / 1024:.1f} KB' if not is_dir else '--'})
        except Exception as e:
            results.append({'name': f'Access Denied: {str(e)}', 'type': 'file', 'size': '0'})
        return sorted(results, key=lambda x: (x['type'] != 'dir', x['name']))