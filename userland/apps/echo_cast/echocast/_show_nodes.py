# Generated method: EchoCast._show_nodes
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

class EchoCast:
    def _show_nodes(self):
        self.canvas.delete('all')
        nodes_mock = [('Aura Vision Pro', 'Device: Headset', 100, 100), ('Sovereign Display Alpha', 'Device: 8K Monitor', 300, 150), ('Echo Speaker Core', 'Device: Audio Array', 150, 220)]
        for name, desc, x, y in nodes_mock:
            f = tk.Frame(self.canvas, bg=PAL['panel'], borderwidth=2, relief='ridge', highlightbackground=PAL['accent_dim'], highlightcolor=PAL['accent_dim'])
            tk.Label(f, text=name, font=('Inter', 10, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(pady=(10, 2), padx=10)
            tk.Label(f, text=desc, font=('Inter', 8), fg=PAL['dim'], bg=PAL['panel']).pack(pady=(0, 10))
            btn = tk.Button(f, text='START CAST', bg=PAL['sidebar'], fg='white', font=('Inter', 8, 'bold'), command=lambda n=name: self._connect_node(n))
            btn.pack(fill='x')
            self.canvas.create_window(x, y, window=f, width=160, height=90)
        self.status.config(text='RECEPTORS LOCATED IN ETHER | 100% SIGNAL INTEGRITY', bg=PAL['success'], fg='black')