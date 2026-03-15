"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._simulate_circuit
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _simulate_circuit(self, tag, name):
        w = tk.Toplevel(self)
        w.title(f'Simulate — {name}')
        w.geometry('420x300')
        w.configure(bg=PAL['bg'])
        out = tk.Text(w, bg=PAL['card'], fg=PAL['on'], font=('Cascadia Code', 10), borderwidth=0, padx=12, pady=12)
        out.pack(fill='both', expand=True, padx=16, pady=16)
        out.insert('end', f'CIRCUIT: {name}\n' + '─' * 36 + '\n\n')
        if tag == 'FA':
            for A in (0, 1):
                for B in (0, 1):
                    for Cin in (0, 1):
                        S, Cout = full_adder(A, B, Cin)
                        out.insert('end', f'A={A} B={B} Cin={Cin}  →  Sum={S} Cout={Cout}\n')
        elif tag == 'HA':
            for A in (0, 1):
                for B in (0, 1):
                    S, C = half_adder(A, B)
                    out.insert('end', f'A={A} B={B}  →  Sum={S} Carry={C}\n')
        elif tag == 'MUX':
            for sel in (0, 1):
                for A in (0, 1):
                    for B in (0, 1):
                        Y = A if sel == 0 else B
                        out.insert('end', f'sel={sel} A={A} B={B}  →  Y={Y}\n')
        elif tag == 'DEC24':
            for A in (0, 1):
                for B in (0, 1):
                    n = A * 2 + B
                    Y = [0, 0, 0, 0]
                    Y[n] = 1
                    out.insert('end', f'A={A} B={B}  →  Y3..0={Y[3]}{Y[2]}{Y[1]}{Y[0]}\n')
        else:
            out.insert('end', 'Simulation output for this circuit coming soon!\n')
