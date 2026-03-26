# Generated method: MeshLudo.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class MeshLudo:
    def __init__(self):
        super().__init__()
        self.engine = LudoEngine()
        self.title('Sovereign Mesh Ludo Apex Pro v5.0')
        self.geometry('1100x900')
        self.configure(bg=PAL['bg'])
        self.status: tk.Label = tk.Label()
        self.canv: tk.Canvas = tk.Canvas()
        self.dice_lbl: tk.Label = tk.Label()
        self.log_txt: tk.Text = tk.Text()
        self._build_ui()