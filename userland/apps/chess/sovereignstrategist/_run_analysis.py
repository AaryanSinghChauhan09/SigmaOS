"""
Auto-split from userland\apps\chess.py — SovereignStrategist._run_analysis
"""

import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys



class SovereignStrategist:
    def _run_analysis(self):
        s = self.engine.evaluate_board() / 10.0
        self.score_lbl.config(text=f"{('+' if s > 0 else '')}{s:.2f}")
        self.analysis_bar['value'] = 50 + s * 10
        self.hist_txt.delete('1.0', 'end')
        history = cast(List[str], self.engine.history)
        for m in history[-10:]:
            self.hist_txt.insert('end', f'{m}\n')
