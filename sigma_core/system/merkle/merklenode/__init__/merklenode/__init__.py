# Generated method: MerkleNode.__init__
import hashlib
from typing import List, Optional

class MerkleNode:
    def __init__(self, left=None, right=None, data=None):
        self.left = left
        self.right = right
        self.data = data
        self.hash = self._calculate_hash()