from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.kernel.kernel_states import BootingState, RunningState, HaltedState

from ._base import SigmaKernel

class SigmaKernel:
    def __init__(self, power_manager=None):
        super().__init__('KERNEL_CORE')
        self._threads = []
        self._page_table = {}
        self._state = BootingState()
        self._power = power_manager