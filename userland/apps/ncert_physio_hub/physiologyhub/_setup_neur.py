# Generated method: PhysiologyHub._setup_neur
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

class PhysiologyHub:
    def _setup_neur(self, master):
        tk.Label(master, text='REFLEX ARC & SYNAPSE', font=('Segoe UI Bold', 12), fg=PAL['accent'], bg=PAL['bg']).pack(pady=10)
        c = tk.Canvas(master, bg='#050510', height=300, highlightthickness=0)
        c.pack(fill='both', expand=True, padx=40)
        c.create_oval(100, 100, 160, 160, fill='#1A1E30', outline=PAL['neural'], width=2)
        c.create_line(160, 130, 400, 130, fill=PAL['neural'], width=4)
        c.create_text(130, 200, text='SOMA', fill='white')
        c.create_text(280, 150, text='AXON', fill='white')
        tk.Button(master, text='SIMULATE IMPULSE', command=lambda: messagebox.showinfo('Neural Pro', 'Propagation: Saltatory Conduction Active'), bg=PAL['accent'], fg='white', relief='flat').pack(pady=10)