from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.kernel.kernel_states import BootingState, RunningState, HaltedState

from ._base import SigmaKernel

class SigmaKernel:
    def health_check(self):
        return not isinstance(self._state, HaltedState)