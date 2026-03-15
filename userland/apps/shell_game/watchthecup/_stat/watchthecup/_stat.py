# Generated method: WatchTheCup._stat
import tkinter as tk
from tkinter import messagebox
import random
import time

class WatchTheCup:
    def _stat(self, parent, label, value):
        fr = tk.Frame(parent, bg=PAL['card'], padx=20, pady=10)
        fr.pack(side='left', expand=True, fill='x')
        tk.Label(fr, text=label, font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['card']).pack()
        lbl = tk.Label(fr, text=value, font=('Segoe UI', 16, 'bold'), fg=PAL['accent'], bg=PAL['card'])
        lbl.pack()
        return lbl