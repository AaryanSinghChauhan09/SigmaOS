"""
Auto-split from userland\apps\omni_tools_android.py — OmniToolsApp._generate_qr
"""

import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime



class OmniToolsApp:
    def _generate_qr(self) -> None:
        data = self.qr_data_entry.get().strip()
        if not data:
            messagebox.showinfo('QR', 'Enter data to encode.')
            return
        matrix = _build_qr_matrix(data, modules=21)
        self.qr_canvas.delete('all')
        modules = len(matrix)
        cell = 420 // (modules + 4)
        offset = cell * 2
        for r, row in enumerate(matrix):
            for c, bit in enumerate(row):
                x0 = offset + c * cell
                y0 = offset + r * cell
                fill = '#000000' if bit else '#FFFFFF'
                self.qr_canvas.create_rectangle(x0, y0, x0 + cell, y0 + cell, fill=fill, outline='')
        self.status.config(text=f'QR generated for: {data[:60]}', bg=PAL['success'], fg='black')
