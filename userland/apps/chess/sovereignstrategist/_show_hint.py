"""
Auto-split from userland\apps\chess.py — SovereignStrategist._show_hint
"""

import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys



class SovereignStrategist:
    def _show_hint(self):
        wp = [(r, c) for r in range(8) for c in range(8) if self.engine.board[r][c] and cast(str, self.engine.board[r][c]).startswith('W')]
        if wp:
            sr, sc = random.choice(wp)
            self._hints = [(sr, sc), (sr - 1, sc), (sr + 1, sc)]
            self._draw_board()
            self._update_status('HINT ACTIVE')
