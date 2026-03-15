# Generated method: SovereignTriage._assign_jurisdiction
import time
import uuid
from typing import Dict, Any, List, Optional

class SovereignTriage:
    def _assign_jurisdiction(self, shard_id: str) -> str:
        """USP: Hierarchical Routing. Logic to map shards to Jurisdictions."""
        shard_map = {'kernel': 'KERNEL', 'loader': 'KERNEL', 'registry': 'KERNEL', 'hal': 'HAL', 'bootloader': 'HAL', 'net_sentinel': 'HAL', 'stealth': 'SECURITY', 'integrity': 'SECURITY', 'compliance': 'SECURITY', 'architect': 'SECURITY', 'shell': 'UI', 'compositor': 'UI', 'vision': 'UI', 'cortex': 'AI', 'gurukul': 'AI', 'intelligence': 'AI', 'mesh': 'MESH', 'sync': 'MESH', 'sync_v2': 'MESH'}
        return shard_map.get(shard_id.lower(), 'KERNEL')