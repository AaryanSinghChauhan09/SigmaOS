"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._shuffle
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _shuffle(self):
        if self.solved:
            self.solved = False
        positions = list(range(self.grid_n * self.grid_n))
        random.shuffle(positions)
        for tile, pos in zip(self.tiles, positions):
            tile['current'] = pos
        self.moves = 0
        self.start_time = time.time()
        self._update_stats()
        self._render_tiles()
        self.status.config(text='Puzzle shuffled! Drag tiles to solve.', bg=PAL['accent2'])
