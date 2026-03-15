# Generated method: JigsawPuzzle._on_release
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _on_release(self, event):
        if self.solved or not self.drag_data['tile']:
            return
        src_tile = self.drag_data['tile']
        tgt_pos = self._find_tile_pos_at(event.x, event.y)
        if tgt_pos is not None and tgt_pos != src_tile['current']:
            tgt_tile = next((t for t in self.tiles if t['current'] == tgt_pos), None)
            if tgt_tile:
                src_tile['current'], tgt_tile['current'] = (tgt_tile['current'], src_tile['current'])
                self.moves += 1
                self._update_stats()
                self._render_tiles()
                self._check_solved()
        self.drag_data['tile'] = None