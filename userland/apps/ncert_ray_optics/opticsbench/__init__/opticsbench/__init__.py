# Generated method: OpticsBench.__init__
import tkinter as tk
from tkinter import ttk

class OpticsBench:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • Ray Optics Virtual Bench')
        self.geometry('1000x800')
        self.configure(bg=PAL['bg'])
        self.mode = tk.StringVar(value='Concave Mirror')
        self.focal_len = 20.0
        self.obj_dist = 40.0
        self._build_ui()