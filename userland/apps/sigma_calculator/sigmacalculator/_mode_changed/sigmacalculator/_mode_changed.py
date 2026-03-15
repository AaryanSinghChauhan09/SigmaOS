# Generated method: SigmaCalculator._mode_changed
import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List

class SigmaCalculator:
    def _mode_changed(self):
        try:
            val = float(self._expr) if self._expr else 0
            m = self._mode.get()
            if m == 'HEX':
                self._update(hex(int(val)).upper())
            elif m == 'BIN':
                self._update(bin(int(val)))
            elif m == 'OCT':
                self._update(oct(int(val)))
            else:
                self._update(str(val))
        except Exception:
            pass