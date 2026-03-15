from abc import ABC, abstractmethod

from ._base import BootingState

class BootingState:
    def handle_task(self, kernel, task_desc):
        print(f'[KERNEL-STATE] Booting. Deferring task: {task_desc}')
        return 'DEFERRED_BOOTING'