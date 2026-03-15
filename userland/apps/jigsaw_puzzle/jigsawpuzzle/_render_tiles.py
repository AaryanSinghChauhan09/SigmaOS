"""
Auto-split from userland\apps\jigsaw_puzzle.py — JigsawPuzzle._render_tiles
"""

import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os



class JigsawPuzzle:
    def _render_tiles(self):
        self.canvas.delete('all')
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
            x2, y2 = (x1 + ts, y1 + ts)
            correct = tile['current'] == tile['correct']
            border = PAL['success'] if correct else PAL['border']
            if 'photo' in tile and tile['photo']:
                cid = self.canvas.create_image(x1, y1, anchor='nw', image=tile['photo'], tags=(f"tile_{tile['id']}", 'tile'))
                self.canvas.create_rectangle(x1, y1, x2, y2, outline=border, width=3 if correct else 1, tags=(f"tile_{tile['id']}", 'tile'))
                tile['canvas_id'] = cid
            else:
                cid = self.canvas.create_rectangle(x1, y1, x2, y2, fill=tile['color'], outline=border, width=3 if correct else 1, tags=(f"tile_{tile['id']}", 'tile'))
                tid = self.canvas.create_text(x1 + ts // 2, y1 + ts // 2, text=tile['label'], font=('Segoe UI', max(10, ts // 6), 'bold'), fill='white', tags=(f"tile_{tile['id']}", 'tile'))
                tile['canvas_id'] = cid
                tile['text_id'] = tid
            tile['x1'], tile['y1'] = (x1, y1)
