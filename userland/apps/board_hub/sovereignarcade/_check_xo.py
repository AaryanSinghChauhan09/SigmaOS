"""
Auto-split from userland\apps\board_hub.py — SovereignArcade._check_xo
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional



class SovereignArcade:
    def _check_xo(self):
        wins = [(0, 1, 2), (3, 4, 5), (6, 7, 8), (0, 3, 6), (1, 4, 7), (2, 5, 8), (0, 4, 8), (2, 4, 6)]
        return any((self.xo_board[w[0]] == self.xo_board[w[1]] == self.xo_board[w[2]] != '' for w in wins))
