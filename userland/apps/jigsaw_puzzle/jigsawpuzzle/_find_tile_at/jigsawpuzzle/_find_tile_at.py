# Generated method: JigsawPuzzle._find_tile_at
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _find_tile_at(self, x, y):
        n = self.grid_n
        ts = self.tile_size
        pad = 4
        board_px = n * (ts + pad) + pad
        ox = max(0, (self.canvas.winfo_width() - board_px) // 2)
        oy = max(0, (self.canvas.winfo_height() - board_px) // 2)
        for tile in self.tiles:
            pos = tile['current']
            row, col = divmod(pos, n)
            x1 = ox + pad + col * (ts + pad)
            y1 = oy + pad + row * (ts + pad)
            if x1 <= x <= x1 + ts and y1 <= y <= y1 + ts:
                return tile
        return None