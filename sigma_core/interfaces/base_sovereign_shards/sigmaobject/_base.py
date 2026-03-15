from abc import ABC, abstractmethod
import time


class SigmaObject(ABC):
    __slots__ = ('__internal_id', '_created_at', '_status')

    def __init__(self):
        self.__internal_id = f'OBJ-{time.time()}'
        self._created_at = time.time()
        self._status = 'INITIALIZED'