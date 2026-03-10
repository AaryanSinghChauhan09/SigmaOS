"""
Cosmos AI-OS: PCI Bus Enumerator & Device Tree
==============================================
Mission: Hardware Discovery, BAR Mapping, and MSI-X Signaling.
"""

from dataclasses import dataclass, field
from typing import List, Optional

@dataclass
class PCIBar:
    base: int
    size: int
    is_io: bool = False

@dataclass
class DeviceNode:
    name: str
    vendor_id: int
    device_id: int
    bus: int
    slot: int
    func: int
    bars: List[PCIBar] = field(default_factory=list)
    status: str = "OFFLINE"
    children: List['DeviceNode'] = field(default_factory=list)

class SovereignPCIBus:
    def __init__(self, kernel):
        self.kernel = kernel
        self.root = DeviceNode("Root Complex", 0, 0, 0, 0, 0, status="RUNNING")
        print("[PCI] Initializing Bus Enumerator...")

    def scan_bus(self):
        """Simulates PCI scanning of 256 buses."""
        print("[PCI] Scanning for connected hardware...")
        # Simulating finding a few key devices
        found = [
            {"name": "Intel E1000", "vid": 0x8086, "did": 0x100E, "class": "Network"},
            {"name": "Cosmos GPU", "vid": 0x10DE, "did": 0xFEED, "class": "Graphics"},
            {"name": "Virtio-9P", "vid": 0x1AF4, "did": 0x1009, "class": "Storage"}
        ]
        
        for i, dev in enumerate(found):
            node = DeviceNode(
                name=dev["name"],
                vendor_id=dev["vid"],
                device_id=dev["did"],
                bus=0, slot=i+1, func=0,
                status="DISCOVERED"
            )
            # Add simulated BARs
            node.bars.append(PCIBar(base=0xF0000000 + (i * 0x1000), size=4096))
            self.root.children.append(node)
            print(f"[PCI] Found {dev['name']} at 00:{i+1}.0 (VID:{hex(dev['vid'])})")

    def get_device_tree_lisp(self):
        """Returns the tree in a Lisp-serialized format."""
        def _serialize(node):
            return {
                "name": node.name,
                "vid": hex(node.vendor_id),
                "did": hex(node.device_id),
                "status": node.status,
                "children": [_serialize(c) for c in node.children]
            }
        return _serialize(self.root)

    def trigger_msix(self, device_name, vector):
        print(f"[MSI-X] Message received from {device_name} (Vector: {hex(vector)})")
        # Route to Kernel Interrupt Manager
        self.kernel.registry["interrupts"].handle_irq(vector)
