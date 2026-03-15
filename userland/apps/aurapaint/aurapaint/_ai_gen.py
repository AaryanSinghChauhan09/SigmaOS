"""
Auto-split from userland\apps\aurapaint.py — AuraPaint._ai_gen
"""

import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any



class AuraPaint:
    def _ai_gen(self):
        prompt = simpledialog.askstring('Aura-Synth', 'Describe drawing intent:')
        if prompt:
            self._set_status(f'SYNTHESIZING: {prompt}')
            self.after(1500, lambda: self._apply_synth(prompt))
