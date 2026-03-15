from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand

from ._base import SovereignCommander

class SovereignCommander:
    def shutdown(self):
        self._commands.clear()
        print('[COMMANDER] Command Dispatcher Offline.')