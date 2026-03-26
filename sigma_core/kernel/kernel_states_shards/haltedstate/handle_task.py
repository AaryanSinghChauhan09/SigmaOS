from abc import ABC, abstractmethod

from ._base import HaltedState

class HaltedState:
    def handle_task(self, kernel, task_desc):
        print(f'[KERNEL-STATE] Halted. Dropping task: {task_desc}')
        return 'DROPPED_HALTED'