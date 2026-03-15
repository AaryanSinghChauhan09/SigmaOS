# Generated method: SigmaMediaForge.scrub_metadata
import os
import sys
import struct
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase

class SigmaMediaForge:
    def scrub_metadata(self, file_path: str) -> Dict[str, Any]:
        """
            Removes EXIF and other tracking metadata from images (JPEG/PNG).
            Pure Python byte manipulation.
            """
        if not os.path.exists(file_path):
            return {'status': 'ERROR', 'msg': 'File not found'}
        ext = os.path.splitext(file_path)[1].lower()
        if ext in ['.jpg', '.jpeg']:
            return self._scrub_jpg(file_path)
        elif ext == '.png':
            return self._scrub_png(file_path)
        return {'status': 'SKIPPED', 'msg': 'Format not supported for scrubbing.'}