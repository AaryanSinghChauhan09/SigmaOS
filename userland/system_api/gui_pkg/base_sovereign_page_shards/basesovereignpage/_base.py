from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod


class BaseSovereignPage(SovereignModule):
    __slots__ = ('_elements', '_theme')
    '\n    Base OOP class for all Userland Pages.\n    Inherits from the core SovereignModule.\n    '