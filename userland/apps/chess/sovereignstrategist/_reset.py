"""
Auto-split from userland\apps\chess.py — SovereignStrategist._reset
"""

import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys



class SovereignStrategist:
    def _reset(self):
        self.engine.reset()
        self._selected = None
        self._hints = []
        self._draw_board()
        self._update_status('RESET')
