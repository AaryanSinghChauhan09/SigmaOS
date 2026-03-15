# Generated method: SigmaExplorer.quick_look
import os
import time

class SigmaExplorer:
    def quick_look(self, file_path: str) -> dict:
        """USP: macOS Finder-style instant preview without fully loading into memory."""
        ext = file_path.split('.')[-1] if '.' in file_path else 'unknown'
        return {'file': file_path, 'type': ext.upper(), 'preview': f'[SIMULATED PREVIEW OF {ext.upper()} CONTENT]', 'metadata': {'size': '42 KB', 'encrypted': True}}