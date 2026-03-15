# Generated method: SovereignPCIBus.__init__
from dataclasses import dataclass, field
from typing import List, Optional

class SovereignPCIBus:
    def __init__(self, kernel):
        self.kernel = kernel
        self.root = DeviceNode('Root Complex', 0, 0, 0, 0, 0, status='RUNNING')
        print('[PCI] Initializing Bus Enumerator...')