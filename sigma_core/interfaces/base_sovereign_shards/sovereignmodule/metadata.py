from abc import ABC, abstractmethod
import time

from ._base import SovereignModule

class SovereignModule:
    @property
    def metadata(self) -> dict:
        return {'name': self.name, 'id': self.object_id, 'status': self.status, 'uptime': time.time() - self._created_at}