# Generated method: SigmaCalculator._save_history
import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List

class SigmaCalculator:
    def _save_history(self):
        try:
            with open(HISTORY_FILE, 'w') as f:
                json.dump(self._history[-50:], f)
        except Exception:
            pass