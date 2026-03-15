# Generated method: JigsawPuzzle._on_grid_change
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _on_grid_change(self):
        self.grid_n = self.grid_var.get()
        self.tile_images = []
        if PIL_AVAILABLE and self.pil_img:
            self._build_image_tiles()
        else:
            self._build_demo_tiles()
        self._render_tiles()
        self._update_stats()