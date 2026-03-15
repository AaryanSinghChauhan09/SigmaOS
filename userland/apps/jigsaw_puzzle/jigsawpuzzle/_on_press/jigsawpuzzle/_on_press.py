# Generated method: JigsawPuzzle._on_press
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _on_press(self, event):
        if self.solved:
            return
        tile = self._find_tile_at(event.x, event.y)
        if tile:
            self.drag_data['tile'] = tile
            self.drag_data['start_pos'] = tile['current']