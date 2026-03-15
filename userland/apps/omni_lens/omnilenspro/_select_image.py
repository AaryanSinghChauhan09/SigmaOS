"""
Auto-split from userland\apps\omni_lens.py — OmniLensPro._select_image
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading



class OmniLensPro:
    def _select_image(self):
        file = filedialog.askopenfilename()
        if file:
            self.canvas.delete('all')
            self.canvas.create_rectangle(50, 50, 450, 450, outline=PAL['accent_dim'], width=2)
            self.canvas.create_text(250, 250, text=f"[IMAGE MOUNTED:\n{file.split('/')[-1]}]", fill=PAL['text'], font=('Inter', 10), justify='center')
            self.status.config(text='STATIC IMAGE MOUNTED INTO VRAM', bg=PAL['panel'], fg=PAL['text'])
