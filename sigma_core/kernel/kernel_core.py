from ..interfaces.base_sovereign import SovereignModule
from .kernel_states import BootingState, RunningState, HaltedState

class SigmaKernel(SovereignModule):
    """
    Sovereign Kernel - Core Execution Engine.
    Implements State Pattern and Composition.
    """
    def __init__(self, power_manager=None):
        super().__init__("KERNEL_CORE")
        self._threads = []
        self._page_table = {}
        self._state = BootingState()
        
        # Composition: Kernel HAS-A PowerManager (not IS-A)
        self._power = power_manager

    def set_state(self, state):
        print(f"[KERNEL] Transitioning to state: {state.__class__.__name__}")
        self._state = state
        if self._power:
            # Side effect via composition
            if isinstance(state, RunningState):
                self._power.set_mode("OPTIMIZED")

    def execute(self, task_desc):
        """Standard ISovereign contract."""
        return self._state.handle_task(self, task_desc)

    def shutdown(self):
        self.set_state(HaltedState())
        self.status = "SHUTTING_DOWN"
        if self._power:
            self._power.set_mode("LOW_POWER")
        print("[KERNEL] Safe shutdown sequence complete.")

    def health_check(self):
        return not isinstance(self._state, HaltedState)
