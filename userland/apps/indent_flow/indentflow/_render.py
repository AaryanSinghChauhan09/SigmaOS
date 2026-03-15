# Generated method: IndentFlow._render
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random

class IndentFlow:
    def _render(self):
        self.canv.delete('all')
        code = self.txt.get('1.0', 'end-1c').split('\n')
        y = 50
        colors = ['#5856D6', '#AF52DE', '#FF375F', '#FF9F0A']
        for i, line in enumerate(code):
            raw = line.strip()
            if not raw:
                continue
            indent = (len(line) - len(line.lstrip())) // 4
            x = 40 + indent * 30
            color = colors[indent % len(colors)]
            self.canv.create_rectangle(x, y, x + 300, y + 40, fill=PAL['panel'], outline=color, width=2)
            self.canv.create_text(x + 150, y + 20, text=raw[:35], fill='white', font=('Inter', 9, 'bold'))
            if i > 0:
                self.canv.create_line(x + 20, y - 10, x + 20, y, fill=PAL['dim'], dash=(4, 4))
            y += 60
        self.status.config(text='LOGIC MAP RECONSTRUCTED NATIVELY.', bg=PAL['success'])