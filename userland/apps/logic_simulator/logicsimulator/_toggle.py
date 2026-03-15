"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._toggle
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _toggle(self, var):
        var.set(0 if var.get() else 1)
        self._evaluate()
