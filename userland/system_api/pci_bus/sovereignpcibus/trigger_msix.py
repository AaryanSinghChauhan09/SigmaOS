# Generated method: SovereignPCIBus.trigger_msix
from dataclasses import dataclass, field
from typing import List, Optional

class SovereignPCIBus:
    def trigger_msix(self, device_name, vector):
        print(f'[MSI-X] Message received from {device_name} (Vector: {hex(vector)})')
        self.kernel.registry['interrupts'].handle_irq(vector)