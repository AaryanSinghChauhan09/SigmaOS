# Generated method: NexusShare._draw_nodes
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading

class NexusShare:
    def _draw_nodes(self, nodes_list):
        self.nodes_canvas.delete('all')
        if not nodes_list:
            self.nodes_canvas.create_text(400, 100, text='No adjacent nodes detected. Radar inactive.', fill=PAL['dim'], font=('Inter', 10, 'italic'))
            return
        for i, node in enumerate(nodes_list):
            x = 80 + i * 150
            y = 100
            col = random.choice([PAL['accent'], PAL['success'], '#FDB813'])
            self.nodes_canvas.create_oval(x - 30, y - 30, x + 30, y + 30, fill=col, outline=PAL['sidebar'], width=3)
            self.nodes_canvas.create_text(x, y + 45, text=node, fill=PAL['text'], font=('Inter', 9, 'bold'))
            lbl = tk.Label(self.nodes_canvas, text='TRANSMIT', font=('Inter', 7, 'bold'), bg=PAL['panel'], fg='white', cursor='hand2')
            lbl.bind('<Button-1>', lambda e, n=node: self._transmit_payload(n))
            self.nodes_canvas.create_window(x, y + 65, window=lbl)