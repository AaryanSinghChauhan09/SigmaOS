# Generated method: LogicSimulator._show_truth
import tkinter as tk
from tkinter import ttk, messagebox

class LogicSimulator:
    def _show_truth(self):
        g = self.gate_cb.get()
        table = f'TRUTH TABLE: {g}\n A | B | OUT\n-----------\n'
        for ia in [0, 1]:
            for ib in [0, 1] if g != 'NOT' else [0]:
                if g == 'AND':
                    r = ia and ib
                elif g == 'OR':
                    r = ia or ib
                elif g == 'NAND':
                    r = not (ia and ib)
                elif g == 'NOR':
                    r = not (ia or ib)
                elif g == 'XOR':
                    r = ia != ib
                elif g == 'NOT':
                    r = not ia
                else:
                    r = 0
                table += f" {ia} | {('-' if g == 'NOT' else ib)} | {int(r)}\n"
        messagebox.showinfo('Logic Lab', table)