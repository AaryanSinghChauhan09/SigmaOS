from abc import ABC, abstractmethod

from ._base import IKernelState

class IKernelState:
    @abstractmethod
    def handle_task(self, kernel, task_desc):
        pass