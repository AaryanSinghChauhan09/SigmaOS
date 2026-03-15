# Generated method: SigmaLocalAINexus.__init__
import socket
import json
import time
from typing import Dict, Any, Optional

class SigmaLocalAINexus:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.local_nodes: Dict[str, str] = {'ollama_layer': 'http://localhost:11434', 'lm_studio_layer': 'http://localhost:1234'}
        self.active_node: str = 'sigma-quantized-native'
        self.weights_loaded: bool = False