# Generated method: MeshLudo._draw_board
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class MeshLudo:
    def _draw_board(self):
        sz, sq = (600, 600 / 15)
        mid = 6 * sq
        regions = [(0, 0, PAL['red']), (sz - mid, 0, PAL['green']), (0, sz - mid, PAL['blue']), (sz - mid, sz - mid, PAL['yellow'])]
        for x1, y1, color in regions:
            self.canv.create_rectangle(x1, y1, x1 + mid, y1 + mid, fill=color, outline='#333', width=2)
            self.canv.create_rectangle(x1 + sq, y1 + sq, x1 + mid - sq, y1 + mid - sq, fill='#000', outline='#222')
        self.canv.create_rectangle(mid, mid, sz - mid, sz - mid, fill='#111', outline='white')
        self.canv.create_polygon(mid, mid, sz - mid, mid, sz / 2, sz / 2, fill=PAL['green'])
        self.canv.create_polygon(mid, mid, mid, sz - mid, sz / 2, sz / 2, fill=PAL['red'])
        self.canv.create_polygon(sz - mid, mid, sz - mid, sz - mid, sz / 2, sz / 2, fill=PAL['yellow'])
        self.canv.create_polygon(mid, sz - mid, sz - mid, sz - mid, sz / 2, sz / 2, fill=PAL['blue'])
        for i in range(16):
            self.canv.create_line(i * sq, mid, i * sq, sz - mid, fill='#222')
            self.canv.create_line(mid, i * sq, sz - mid, i * sq, fill='#222')
        self._render_pieces()