"""
SigmaOS Web3 Decentralized State Ledger
Implements immutable Directed Acyclic Graph (DAG) state tracking.
"""
import hashlib
import time
from typing import Dict, Any, List

class DagNode:
    def __init__(self, action: str, previous_hash: str):
        self.timestamp = time.time()
        self.action = action
        self.previous_hash = previous_hash
        self.hash = self._calculate_hash()

    def _calculate_hash(self) -> str:
        data = f"{self.timestamp}{self.action}{self.previous_hash}"
        return hashlib.sha256(data.encode()).hexdigest()

class Web3StateLedger:
    def __init__(self):
        self.chain: List[DagNode] = []
        # Create Genesis block
        self._append_node("GENESIS_BOOT")

    def _append_node(self, action: str):
        prev_hash = self.chain[-1].hash if self.chain else "0"
        new_node = DagNode(action, prev_hash)
        self.chain.append(new_node)
        print(f"[Web3Ledger] State committed: {action} (Hash: {new_node.hash[:8]})")

    def commit_transaction(self, subsystem: str, action: str):
        """
        Record a state change immutably.
        """
        transaction = f"[{subsystem}] -> {action}"
        self._append_node(transaction)

    def verify_integrity(self) -> bool:
        """
        Audit the DAG for tampering.
        """
        for i in range(1, len(self.chain)):
            current = self.chain[i]
            previous = self.chain[i-1]
            if current.previous_hash != previous.hash:
                return False
            if current.hash != current._calculate_hash():
                return False
        return True
