from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

from ._base import BaseSovereignPage

class BaseSovereignPage:
    def __init__(self, page_name):
        super().__init__(f'GUI_PAGE_{page_name}')
        self._elements = []
        self._theme = 'SovereignDark'