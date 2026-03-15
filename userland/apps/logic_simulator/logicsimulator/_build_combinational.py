"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._build_combinational
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _build_combinational(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'], padx=24, pady=16)
        nb.add(tab, text='  🔗 Circuits  ')
        tk.Label(tab, text='Combinational Logic Circuits', fg=PAL['accent'], bg=PAL['bg'], font=('Segoe UI Bold', 13)).pack(pady=(0, 12))
        circuits = [('2:1 Multiplexer', 'sel,A,B → (A if sel=0 else B)', 'MUX'), ('1:2 Demultiplexer', 'sel,D → Y0=D if sel=0 else Y1=D', 'DEMUX'), ('Decoder 2:4', 'A,B → 4 output lines', 'DEC24'), ('Encoder 4:2', '4 inputs → A,B priority encoding', 'ENC42'), ('Full Adder', 'A+B+Cin → Sum, Cout', 'FA'), ('Half Adder', 'A+B → Sum, Carry', 'HA')]
        for name, desc, tag in circuits:
            card = tk.Frame(tab, bg=PAL['card'], padx=16, pady=10, highlightthickness=1, highlightbackground=PAL['border'])
            card.pack(fill='x', pady=4)
            tk.Label(card, text=name, fg=PAL['accent'], bg=PAL['card'], font=('Segoe UI Bold', 10)).pack(anchor='w')
            tk.Label(card, text=desc, fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 9)).pack(anchor='w')
            tk.Button(card, text='SIMULATE →', bg=PAL['accent'], fg='white', font=('Segoe UI', 8), relief='flat', padx=12, pady=4, command=lambda t=tag, n=name: self._simulate_circuit(t, n)).pack(anchor='e')
