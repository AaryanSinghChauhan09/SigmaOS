# Generated method: LogicSimulator._toggle
import tkinter as tk
from tkinter import ttk, messagebox

class LogicSimulator:
    def _toggle(self, var):
        self.sim_state[var] = 1 - int(self.sim_state[var])
        if var == 'A':
            self.btn_a.config(text=str(self.sim_state['A']))
        else:
            self.btn_b.config(text=str(self.sim_state['B']))
        self._update()