from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.kernel.kernel_states import BootingState, RunningState, HaltedState

from ._base import SigmaKernel

class SigmaKernel:
    def execute(self, task_desc):
        """Standard ISovereign contract."""
        return self._state.handle_task(self, task_desc)