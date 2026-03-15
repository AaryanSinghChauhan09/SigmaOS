# Generated method: LogicSimulator._update
import tkinter as tk
from tkinter import ttk, messagebox

class LogicSimulator:
    def _update(self):
        self.canvas.delete('all')
        a, b = (int(self.sim_state['A']), int(self.sim_state['B']))
        g = self.gate_cb.get()
        self.sim_state['Gate'] = g
        if g == 'AND':
            res = a and b
        elif g == 'OR':
            res = a or b
        elif g == 'NAND':
            res = not (a and b)
        elif g == 'NOR':
            res = not (a or b)
        elif g == 'XOR':
            res = a != b
        elif g == 'NOT':
            res = not a
        else:
            res = 0
        res = int(res)
        color_a = '#00D26A' if a else '#FF4D4D'
        color_b = '#00D26A' if b else '#FF4D4D'
        color_res = '#00D26A' if res else '#FF4D4D'
        self.canvas.create_line(50, 150, 200, 150, fill=color_a, width=4)
        if g != 'NOT':
            self.canvas.create_line(50, 250, 200, 250, fill=color_b, width=4)
        self.canvas.create_rectangle(200, 120, 350, 280, fill='#1A1E30', outline='white', width=2)
        self.canvas.create_text(275, 200, text=g, fill='white', font=('Segoe UI Bold', 16))
        self.canvas.create_line(350, 200, 500, 200, fill=color_res, width=4)
        self.canvas.create_oval(510, 180, 550, 220, fill=color_res, outline='white')
        self.canvas.create_text(530, 200, text=str(res), fill='black', font=('Segoe UI Bold', 12))