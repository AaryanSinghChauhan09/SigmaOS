# Generated method: SigmaCalculator._safe_eval
import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List

class SigmaCalculator:
    def _safe_eval(self):
        try:
            return float(eval(self._expr, {'__builtins__': {}, 'math': math}, {}))
        except Exception:
            return 0.0