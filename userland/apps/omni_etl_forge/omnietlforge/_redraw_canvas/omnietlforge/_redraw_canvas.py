# Generated method: OmniETLForge._redraw_canvas
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading

class OmniETLForge:
    def _redraw_canvas(self):
        self.canvas.delete('all')
        ins = [n for n in self.nodes if n['type'] == 'IN']
        outs = [n for n in self.nodes if n['type'] == 'OUT']
        for i_node in ins:
            for o_node in outs:
                self.canvas.create_line(i_node['x'] + 60, i_node['y'], o_node['x'] - 60, o_node['y'], fill=PAL['dim'], width=2, arrow=tk.LAST, dash=(4, 4))
        for n in self.nodes:
            self.canvas.create_rectangle(n['x'] - 60, n['y'] - 25, n['x'] + 60, n['y'] + 25, fill=n['color'], outline=PAL['bg'], width=2)
            self.canvas.create_text(n['x'], n['y'], text=n['name'], fill='white', font=('Inter', 8, 'bold'), width=110, justify='center')