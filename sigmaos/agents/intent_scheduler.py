"""
SigmaOS Intent Scheduler
Replaces standard Round-Robin CPU scheduling with goal-oriented task execution.
"""
from typing import List

class IntentTask:
    def __init__(self, goal: str):
        self.goal = goal
        self.subtasks: List[str] = []
        self.is_resolved = False

    def decompose(self):
        """
        Simulates the LLM breaking down a high-level goal into actionable syscalls.
        """
        print(f"[IntentScheduler] Decomposing goal: {self.goal}")
        self.subtasks = [f"Step 1 for {self.goal}", f"Step 2 for {self.goal}"]

class IntentScheduler:
    def __init__(self):
        self.active_intents: List[IntentTask] = []

    def submit_intent(self, goal: str):
        task = IntentTask(goal)
        task.decompose()
        self.active_intents.append(task)
        print(f"[IntentScheduler] Intent '{goal}' queued. Subtasks generated: {len(task.subtasks)}")

    def tick(self):
        """
        The OS clock tick. Instead of swapping CPU registers, it executes the next subtask.
        """
        for task in self.active_intents:
            if not task.is_resolved and task.subtasks:
                current_step = task.subtasks.pop(0)
                print(f"[IntentScheduler] Executing: {current_step}")
                if not task.subtasks:
                    task.is_resolved = True
                    print(f"[IntentScheduler] Goal achieved: {task.goal}")
