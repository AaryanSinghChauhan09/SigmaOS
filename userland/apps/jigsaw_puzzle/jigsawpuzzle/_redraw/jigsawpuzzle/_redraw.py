# Generated method: JigsawPuzzle._redraw
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _redraw(self):
        self.canvas.after(10, self._render_tiles)