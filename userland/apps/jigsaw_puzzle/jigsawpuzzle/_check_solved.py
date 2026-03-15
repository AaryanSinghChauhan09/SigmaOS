"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._check_solved
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _check_solved(self):
        correct = sum((1 for t in self.tiles if t['current'] == t['correct']))
        total = len(self.tiles)
        pct = int(correct / total * 100)
        self.progress_var.set(pct)
        self.lbl_progress.config(text=f'{pct}% Complete')
        if correct == total:
            self.solved = True
            elapsed = int(time.time() - (self.start_time or time.time()))
            m, s = divmod(elapsed, 60)
            self.status.config(text=f'🏆 SOLVED in {m:02}:{s:02}  |  {self.moves} moves. Congratulations!', bg=PAL['success'])
            messagebox.showinfo('🏆 Puzzle Solved!', f'Congratulations!\n\nYou solved the {self.grid_n}×{self.grid_n} puzzle in {m:02}:{s:02} using {self.moves} moves.')
