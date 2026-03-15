"""
Auto-split from userland\apps\chess.py — SovereignStrategist._ai_move
"""

import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys



class SovereignStrategist:
    def _ai_move(self):
        m = self.engine.get_ai_move()
        if m:
            self.engine.execute_move(*m)
        self._draw_board()
        self._run_analysis()
        self._update_status('YOUR TURN')
