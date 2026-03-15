"""
SigmaOS Sovereign Vector Memory v1.0
====================================
USP: Zero-dependency, low-latency kernel-level vector storage for "Unlimited Context".
Allows AI agents to store and recall experiences and telemetry without external DBs.
"""
import array
import math
import json
import os
import time
from typing import List, Tuple, Dict, Any

class VectorMemory:
    def __init__(self, dimension: int = 128, storage_path: str = "sigma_storage/vector_memory.bin"):
        self.dim = dimension
        self.storage_path = storage_path
        self.vectors = [] # List of array.array('f')
        self.metadata = [] # List of Dict
        self._ensure_storage()
        self.load()

    def _ensure_storage(self):
        os.makedirs(os.path.dirname(self.storage_path), exist_ok=True)

    def _generate_embedding_mock(self, text: str) -> array.array:
        """Simulates a lightweight embedding using character frequency and hashing."""
        emb = [0.0] * self.dim
        for i, char in enumerate(text):
            idx = (ord(char) * (i + 1)) % self.dim
            emb[idx] += 1.0
        
        # Normalize
        norm = math.sqrt(sum(x*x for x in emb)) or 1.0
        return array.array('f', [x / norm for x in emb])

    def add_memory(self, text: str, payload: Dict[str, Any] = None):
        """Stores a new contextual memory."""
        vector = self._generate_embedding_mock(text)
        meta = {
            "text": text,
            "timestamp": time.time(),
            "payload": payload or {}
        }
        self.vectors.append(vector)
        self.metadata.append(meta)
        self.save()

    def search(self, query: str, top_k: int = 3) -> List[Dict]:
        """Finds most similar memories using Cosine Similarity."""
        if not self.vectors:
            return []

        query_vec = self._generate_embedding_mock(query)
        scores = []

        for i, vec in enumerate(self.vectors):
            score = self._cosine_similarity(query_vec, vec)
            scores.append((score, self.metadata[i]))

        # Sort by score descending
        scores.sort(key=lambda x: x[0], reverse=True)
        return [item[1] for item in scores[:top_k]]

    def _cosine_similarity(self, v1, v2) -> float:
        dot = sum(a * b for a, b in zip(v1, v2))
        return dot # Since vectors are normalized during creation

    def save(self):
        """Serializes vectors and metadata to disk."""
        try:
            with open(self.storage_path, "wb") as f:
                # Header: Number of vectors
                f.write(len(self.vectors).to_bytes(4, 'little'))
                for vec in self.vectors:
                    vec.tofile(f)
            
            with open(self.storage_path + ".meta", "w") as f:
                json.dump(self.metadata, f)
        except Exception as e:
            print(f"VectorMemory Save Error: {e}")

    def load(self):
        """Restores memory state from disk."""
        if not os.path.exists(self.storage_path):
            return

        try:
            with open(self.storage_path, "rb") as f:
                size_data = f.read(4)
                if not size_data: return
                count = int.from_bytes(size_data, 'little')
                for _ in range(count):
                    vec = array.array('f')
                    vec.fromfile(f, self.dim)
                    self.vectors.append(vec)
            
            meta_path = self.storage_path + ".meta"
            if os.path.exists(meta_path):
                with open(meta_path, "r") as f:
                    self.metadata = json.load(f)
        except Exception as e:
            print(f"VectorMemory Load Error: {e}")

if __name__ == "__main__":
    mem = VectorMemory()
    mem.add_memory("Optimized the kernel thermal throttle for high-performance gaming mode.")
    mem.add_memory("Applied post-quantum integrity checks to the userland binaries.")
    
    print("Searching for 'performance':")
    results = mem.search("performance")
    for r in results:
        print(f"- {r['text']} (Time: {r['timestamp']})")
