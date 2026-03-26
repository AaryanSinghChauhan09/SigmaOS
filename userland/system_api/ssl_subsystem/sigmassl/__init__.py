# Generated method: SigmaSSL.__init__
from typing import Dict, List, Any

class SigmaSSL:
    def __init__(self, kernel):
        self.kernel = kernel
        self._instances = {'Sovereign_Linux_v1': 'Running', 'Wasm_Sandbox': 'Idle', 'Doc_Indexer_Container': 'Running'}
        self._memory_usage = '450 MB'
        self._supported_binaries = ['elf', 'wasm', 'docker-oci', 'appimage']