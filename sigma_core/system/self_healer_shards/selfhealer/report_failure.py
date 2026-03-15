# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
import importlib
from .base_sovereign import SigmaModule

class SelfHealer:
    def report_failure(self, shard_id):
        self.failure_log[shard_id] = self.failure_log.get(shard_id, 0) + 1
        return self.attempt_heal(shard_id)