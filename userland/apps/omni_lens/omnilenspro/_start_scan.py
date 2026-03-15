"""
Auto-split from userland\apps\omni_lens.py — OmniLensPro._start_scan
"""

import tkinter as tk
from tkinter import ttk, messagebox, filedialog
import random
import time
import threading



class OmniLensPro:
    def _start_scan(self):
        if self.scanning:
            return
        self.scanning = True
        self.res_text.config(state=tk.NORMAL)
        self.res_text.delete(1.0, tk.END)
        self.res_text.config(state=tk.DISABLED)
        self._log('>>> INITIATING ON-DEVICE TENSOR FLOW...')
        self.status.config(text='SCANNING VISUAL MATRIX...', bg=PAL['warning'], fg='black')

        def animate_scan(y):
            if not self.scanning:
                return
            if self.scan_line:
                self.canvas.delete(self.scan_line)
            if y < 450:
                self.scan_line = self.canvas.create_line(50, y, 450, y, fill=PAL['accent'], width=3)
                self.after(20, lambda: animate_scan(y + 10))
            else:
                self.canvas.delete(self.scan_line)
                self._finish_scan()
        animate_scan(50)
