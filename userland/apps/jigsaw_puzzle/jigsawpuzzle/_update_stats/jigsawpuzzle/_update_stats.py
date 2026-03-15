# Generated method: JigsawPuzzle._update_stats
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _update_stats(self):
        n = self.grid_n
        total = n * n
        correct = sum((1 for t in self.tiles if t['current'] == t['correct']))
        pct = int(correct / total * 100) if total else 0
        self.lbl_moves.config(text=str(self.moves))
        self.lbl_grid.config(text=f'{n}×{n}')
        self.lbl_tiles.config(text=str(total))
        self.progress_var.set(pct)
        self.lbl_progress.config(text=f'{pct}% Complete')