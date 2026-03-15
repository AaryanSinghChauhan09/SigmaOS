"""
Auto-split from userland\apps\sigma_calculator.py — SigmaCalculator._update
"""

import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List



class SigmaCalculator:
    def _update(self, txt):
        self._disp.config(text=txt[-22:] if len(txt) > 22 else txt)
