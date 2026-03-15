# Generated method: SigmaMerkleTree.__init__
import hashlib
from typing import List, Optional

class SigmaMerkleTree:
    def __init__(self, data_blocks: List[str]):
        if not data_blocks:
            self.root = None
            self._leaves = []
            return
        self._leaves = [MerkleNode(data=b) for b in data_blocks]
        self.root = self._build_tree(self._leaves)