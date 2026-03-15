"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._start_demo
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _start_demo(self):
        self.grid_n = self.grid_var.get()
        self._build_demo_tiles()
        self._render_tiles()
        self._update_stats()
