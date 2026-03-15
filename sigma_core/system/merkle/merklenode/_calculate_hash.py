# Generated method: MerkleNode._calculate_hash
import hashlib
from typing import List, Optional

class MerkleNode:
    def _calculate_hash(self) -> str:
        """USP: Cryptographic Hardening. Uses prefixes to prevent second-preimage attacks."""
        if self.data:
            content = b'\x00' + str(self.data).encode()
            return hashlib.sha256(content).hexdigest()
        combine = b'\x01' + self.left.hash.encode() + self.right.hash.encode()
        return hashlib.sha256(combine).hexdigest()