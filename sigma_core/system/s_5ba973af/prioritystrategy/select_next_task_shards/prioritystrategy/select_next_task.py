from abc import ABC, abstractmethod

class PriorityStrategy:
    def select_next_task(self, task_queue):
        if not task_queue:
            return None
        task_queue.sort(key=lambda x: x.get('priority', 0), reverse=True)
        return task_queue.pop(0)