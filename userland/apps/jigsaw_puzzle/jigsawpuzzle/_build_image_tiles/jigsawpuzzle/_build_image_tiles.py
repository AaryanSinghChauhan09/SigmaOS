# Generated method: JigsawPuzzle._build_image_tiles
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _build_image_tiles(self):
        if not self.pil_img:
            return
        n = self.grid_n
        ts = self.tile_size
        total = n * n
        img = self.pil_img.resize((ts * n, ts * n))
        self.tile_images = []
        self.tiles = []
        for i in range(total):
            r, c = divmod(i, n)
            box = (c * ts, r * ts, (c + 1) * ts, (r + 1) * ts)
            crop = img.crop(box)
            photo = ImageTk.PhotoImage(crop)
            self.tile_images.append(photo)
            self.tiles.append({'id': i, 'correct': i, 'current': i, 'photo': photo, 'canvas_id': None, 'text_id': None})