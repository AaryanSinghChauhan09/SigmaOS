"""
Auto-split from userland\apps\sigma_calculator.py — SigmaCalculator._load_history
"""

import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List



class SigmaCalculator:
    def _load_history(self):
        try:
            os.makedirs(os.path.dirname(HISTORY_FILE), exist_ok=True)
            if os.path.exists(HISTORY_FILE):
                with open(HISTORY_FILE) as f:
                    self._history = json.load(f)
        except Exception:
            pass
