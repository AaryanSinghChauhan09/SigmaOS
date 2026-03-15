# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import hashlib
import os

class SovereigntyManager:
    def calculate_shard_hash(self, file_path):
        """Calculates SHA-256 hash of a module shard."""
        sha256 = hashlib.sha256()
        with open(file_path, 'rb') as f:
            while (chunk := f.read(8192)):
                sha256.update(chunk)
        return sha256.hexdigest()