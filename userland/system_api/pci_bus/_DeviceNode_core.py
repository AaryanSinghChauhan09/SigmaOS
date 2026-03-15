# Generated class core: DeviceNode
from dataclasses import dataclass, field
from typing import List, Optional

@dataclass
class DeviceNode:
    name: str
    vendor_id: int
    device_id: int
    bus: int
    slot: int
    func: int
    bars: List[PCIBar] = field(default_factory=list)
    status: str = 'OFFLINE'
    children: List['DeviceNode'] = field(default_factory=list)