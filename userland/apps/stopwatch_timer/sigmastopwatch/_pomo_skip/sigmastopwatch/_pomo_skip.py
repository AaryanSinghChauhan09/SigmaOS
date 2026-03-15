# Generated method: SigmaStopwatch._pomo_skip
import tkinter as tk
from tkinter import ttk
import time, threading
from typing import Any

class SigmaStopwatch:
    def _pomo_skip(self):
        self._pomo_running = False
        self._pomo_next_phase()
        self._pomo_running = True
        self._pomo_tick()