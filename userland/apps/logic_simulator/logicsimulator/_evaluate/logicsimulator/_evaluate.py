# Generated method: LogicSimulator._evaluate
import tkinter as tk
from tkinter import ttk

class LogicSimulator:
    def _evaluate(self):
        g = self._gate_var.get()
        a = bool(self._A.get())
        b = bool(self._B.get())
        out = int(GATES[g](a, b))
        col = PAL['on'] if out else PAL['off']
        self._out_lbl.config(text=str(out), fg=col)
        self._out_txt.config(text='HIGH' if out else 'LOW', fg=col)