from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.command_interfaces import ICommander, ICommand


class SovereignCommander(SovereignModule, ICommander):
    __slots__ = ('_commands',)
    '\n    Sovereign Commander.\n    Centralized router for system-level actions (Command Pattern).\n    '