from abc import ABC, abstractmethod

class IKernelState(ABC):
    @abstractmethod
    def handle_task(self, kernel, task_desc):
        pass

class BootingState(IKernelState):
    def handle_task(self, kernel, task_desc):
        print(f"[KERNEL-STATE] Booting. Deferring task: {task_desc}")
        return "DEFERRED_BOOTING"

class RunningState(IKernelState):
    def handle_task(self, kernel, task_desc):
        print(f"[KERNEL-STATE] Running. Executing task: {task_desc}")
        return f"EXECUTED_{task_desc}"

class HaltedState(IKernelState):
    def handle_task(self, kernel, task_desc):
        print(f"[KERNEL-STATE] Halted. Dropping task: {task_desc}")
        return "DROPPED_HALTED"
