# Generated method: SigmaConversionEngine.folder_to_zip
import os
import time
import json
import hashlib
from typing import Dict, Any, Optional, Union

class SigmaConversionEngine:
    def folder_to_zip(self, folder_path: str, output_name: str='Archive.zip') -> str:
        """USP: Fast Workspace Compaction."""
        print(f'[CONVERT] Archiving {folder_path} to {output_name}...')
        import zipfile
        try:
            with zipfile.ZipFile(output_name, 'w', zipfile.ZIP_DEFLATED) as zipf:
                for root, dirs, files in os.walk(folder_path):
                    for file in files:
                        zipf.write(os.path.join(root, file), os.path.relpath(os.path.join(root, file), os.path.join(folder_path, '..')))
            self.stats['converstions_count'] = int(self.stats['converstions_count']) + 1
            return f'Success: {output_name} created.'
        except Exception as e:
            return f'Error: Archive failed - {e}'