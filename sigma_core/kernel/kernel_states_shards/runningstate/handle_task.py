from abc import ABC, abstractmethod

from ._base import RunningState

class RunningState:
    def handle_task(self, kernel, task_desc):
        print(f'[KERNEL-STATE] Running. Executing task: {task_desc}')
        return f'EXECUTED_{task_desc}'