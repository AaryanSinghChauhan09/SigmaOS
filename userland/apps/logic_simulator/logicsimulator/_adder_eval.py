"""
Auto-split from userland\apps\logic_simulator.py — LogicSimulator._adder_eval
"""

import tkinter as tk
from tkinter import ttk



class LogicSimulator:
    def _adder_eval(self):
        A = sum((b.get() << 3 - i for i, b in enumerate(self._bits_A)))
        B = sum((b.get() << 3 - i for i, b in enumerate(self._bits_B)))
        S = A + B
        carry = 1 if S > 15 else 0
        S_4bit = S & 15
        self._adder_out.config(text=f'{A} + {B} = {S_4bit}  (carry={carry})  BIN: {bin(S_4bit)}')
