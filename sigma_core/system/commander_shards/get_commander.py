from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand
from ..sovereigncommander._base import SovereignCommander

def get_commander() -> SovereignCommander:
    return SovereignCommander()