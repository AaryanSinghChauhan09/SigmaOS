# Generated method: LogicSimulator._toggle_btn
import tkinter as tk
from tkinter import ttk

class LogicSimulator:
    def _toggle_btn(self, which):
        if which == 'A':
            self._A.set(0 if self._A.get() else 1)
            self._a_btn.config(text=f'A = {self._A.get()}', bg=PAL['on'] if self._A.get() else PAL['off'])
        else:
            self._B.set(0 if self._B.get() else 1)
            self._b_btn.config(text=f'B = {self._B.get()}', bg=PAL['on'] if self._B.get() else PAL['off'])
        self._evaluate()