from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand

from ._base import SovereignCommander

class SovereignCommander:
    def __init__(self):
        super().__init__('SOVEREIGN_COMMANDER')
        self._commands = {}