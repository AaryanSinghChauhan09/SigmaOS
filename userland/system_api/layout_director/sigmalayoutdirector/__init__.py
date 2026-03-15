# Generated method: SigmaLayoutDirector.__init__
from enum import Enum
from dataclasses import dataclass

class SigmaLayoutDirector:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.current_state = UIState(FormFactor.DESKTOP, 1.0, 'Sidebar', 'Dashboard')
        self._stats = {'layout_shifts': 0, 'handoff_events': 0}