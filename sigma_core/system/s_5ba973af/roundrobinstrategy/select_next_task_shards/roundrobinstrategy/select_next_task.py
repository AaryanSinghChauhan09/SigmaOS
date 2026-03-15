from abc import ABC, abstractmethod

class RoundRobinStrategy:
    def select_next_task(self, task_queue):
        if task_queue:
            return task_queue.pop(0)
        return None