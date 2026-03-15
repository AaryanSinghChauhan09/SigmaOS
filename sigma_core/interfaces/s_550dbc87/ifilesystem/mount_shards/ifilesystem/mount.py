from abc import ABC, abstractmethod

class IFileSystem:
    @abstractmethod
    def mount(self, path):
        pass