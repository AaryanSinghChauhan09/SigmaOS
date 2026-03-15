from abc import ABC, abstractmethod

from ._base import IFileSystem

class IFileSystem:
    @abstractmethod
    def mount(self, path):
        raise NotImplementedError