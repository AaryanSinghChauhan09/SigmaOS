from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand

from ._base import SovereignCommander

class SovereignCommander:
    def initialize(self):
        print('[COMMANDER] Command Dispatcher Online.')