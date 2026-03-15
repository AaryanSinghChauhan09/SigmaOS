# Generated method: SigmaMerkleTree._build_tree
import hashlib
from typing import List, Optional

class SigmaMerkleTree:
    def _build_tree(self, nodes: List[MerkleNode]) -> MerkleNode:
        if len(nodes) == 1:
            return nodes[0]
        temp_nodes = nodes.copy()
        if len(temp_nodes) % 2 != 0:
            temp_nodes.append(temp_nodes[-1])
        next_level = []
        for i in range(0, len(temp_nodes), 2):
            next_level.append(MerkleNode(left=temp_nodes[i], right=temp_nodes[i + 1]))
        return self._build_tree(next_level)