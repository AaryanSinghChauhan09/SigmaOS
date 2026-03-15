# Generated class core: MeshUpdatePackage
import hashlib
import time
import uuid
from dataclasses import dataclass

@dataclass
class MeshUpdatePackage:
    version: str
    delta_hash: str
    signature: str
    timestamp: float