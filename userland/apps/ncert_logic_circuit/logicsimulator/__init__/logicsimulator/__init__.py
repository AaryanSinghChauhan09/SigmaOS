# Generated method: LogicSimulator.__init__
import tkinter as tk
from tkinter import ttk, messagebox

class LogicSimulator:
    def __init__(self):
        super().__init__()
        self.title('SigmaOS • Logic Circuit Simulator')
        self.geometry('900x650')
        self.configure(bg='#0B0D17')
        self.sim_state = {'A': 0, 'B': 0, 'Gate': 'AND'}
        self._build_ui()