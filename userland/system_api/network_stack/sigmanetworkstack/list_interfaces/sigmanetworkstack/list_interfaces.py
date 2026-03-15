# Generated method: SigmaNetworkStack.list_interfaces
import time
import uuid
import hashlib
from dataclasses import dataclass, field
from enum import Enum, auto

class SigmaNetworkStack:
    def list_interfaces(self) -> list[dict]:
        return [{'name': i.name, 'ip4': i.ip4, 'ip6': i.ip6, 'up': i.up, 'speed': i.speed_mbps, 'encryption': i.encryption.value, 'mesh': i.mesh_capable} for i in self._interfaces.values()]