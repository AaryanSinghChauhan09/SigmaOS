# Generated method: JigsawPuzzle._build_demo_tiles
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _build_demo_tiles(self):
        """Build colorful numbered tiles for demo mode."""
        n = self.grid_n
        total = n * n
        colors = ['#FF6B6B', '#FF9F43', '#FFD93D', '#6BCB77', '#4D96FF', '#C77DFF', '#FF70AB', '#00B4D8', '#F4A261', '#52B788', '#FB8500', '#8338EC', '#3A86FF', '#FF006E', '#FFBE0B', '#8AC926', '#1982C4', '#6A4C93', '#FF595E', '#FFCA3A', '#6A994E', '#023E8A', '#E56B6F', '#B5179E', '#480CA8', '#4361EE', '#4CC9F0', '#F72585', '#7400B8', '#43AA8B', '#90BE6D', '#F9C74F', '#F9844A', '#F8961E', '#F3722C', '#577590']
        self.tiles = []
        for i in range(total):
            row, col = divmod(i, n)
            self.tiles.append({'id': i, 'correct': i, 'current': i, 'color': colors[i % len(colors)], 'label': str(i + 1), 'canvas_id': None, 'text_id': None})
        self.status.config(text=f'Demo mode — {n}×{n} grid. Load an image for a real jigsaw!', bg=PAL['accent2'])