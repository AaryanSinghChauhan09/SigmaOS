from ..interfaces.base_sovereign import SovereignModule
from .kernel_states import BootingState, RunningState, HaltedState

class SigmaKernel(SovereignModule):
    """
    Sovereign Kernel - Core Execution Engine.
    Implements State Pattern for robust lifecycle management.
    """
    def __init__(self):
        super().__init__("KERNEL_CORE")
        self._threads = []
        self._page_table = {}
        self._state = BootingState()

    def set_state(self, state):
        print(f"[KERNEL] Transitioning to state: {state.__class__.__name__}")
        self._state = state

    def execute(self, task_desc):
        """Standard ISovereign contract."""
        return self._state.handle_task(self, task_desc)

    def shutdown(self):
        self.set_state(HaltedState())
        self.status = "SHUTTING_DOWN"
        print("[KERNEL] Safe shutdown sequence complete.")

    def health_check(self):
        return not isinstance(self._state, HaltedState)
