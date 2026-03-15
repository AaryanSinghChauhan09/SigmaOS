"""
Auto-split from userland\apps\sigma_calculator.py — SigmaCalculator._refresh_history
"""

import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List



class SigmaCalculator:
    def _refresh_history(self):
        self._hist_box.delete('1.0', 'end')
        for h in self._history[-5:]:
            self._hist_box.insert('end', h + '\n')
