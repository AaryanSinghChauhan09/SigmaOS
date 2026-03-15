# Generated method: SigmaMerkleTree.get_root_hash
import hashlib
from typing import List, Optional

class SigmaMerkleTree:
    def get_root_hash(self) -> Optional[str]:
        return self.root.hash if self.root else None