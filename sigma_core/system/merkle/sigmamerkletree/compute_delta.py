# Generated method: SigmaMerkleTree.compute_delta
import hashlib
from typing import List, Optional

class SigmaMerkleTree:
    @staticmethod
    def compute_delta(tree_a, tree_b) -> List[int]:
        """USP: High-Speed Sync. Identifies byte-indices of differing shards between two states."""
        if not tree_a.root or not tree_b.root:
            return []
        if tree_a.get_root_hash() == tree_b.get_root_hash():
            return []
        diff_indices = []
        for i in range(min(len(tree_a._leaves), len(tree_b._leaves))):
            if tree_a._leaves[i].hash != tree_b._leaves[i].hash:
                diff_indices.append(i)
        return diff_indices