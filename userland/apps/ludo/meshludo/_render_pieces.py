# Generated method: MeshLudo._render_pieces
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class MeshLudo:
    def _render_pieces(self):
        self.canv.delete('piece')
        for color in ['RED', 'GREEN', 'BLUE', 'YELLOW']:
            for i in range(4):
                x, y = self.engine.get_piece_coord(color, i)
                self.canv.create_oval(x - 15, y - 15, x + 15, y + 15, fill=PAL[color.lower()], outline='white', width=2, tags='piece')
        self.log_txt.delete('1.0', 'end')
        count = len(self.engine.history)
        for i in range(max(0, count - 12), count):
            self.log_txt.insert('end', f'  ✦ {str(self.engine.history[i])}\n')
        self.log_txt.see('end')