# Generated method: LogicSimulator._build_adder
import tkinter as tk
from tkinter import ttk

class LogicSimulator:
    def _build_adder(self, nb):
        tab = tk.Frame(nb, bg=PAL['bg'], padx=30, pady=20)
        nb.add(tab, text='  ➕ Adder  ')
        tk.Label(tab, text='Binary Adder Simulator', fg=PAL['accent'], bg=PAL['bg'], font=('Segoe UI Bold', 13)).pack(pady=(0, 16))
        fr = tk.Frame(tab, bg=PAL['card'], padx=20, pady=16)
        fr.pack(fill='x')
        tk.Label(fr, text='4-bit Ripple Carry Adder', fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 9, 'bold')).grid(row=0, column=0, columnspan=9, sticky='w', pady=(0, 8))
        self._bits_A = [tk.IntVar(value=0) for _ in range(4)]
        self._bits_B = [tk.IntVar(value=0) for _ in range(4)]
        for i in range(4):
            tk.Label(fr, text=f'A{3 - i}', fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 8)).grid(row=1, column=i * 2)
            tk.Checkbutton(fr, variable=self._bits_A[i], bg=PAL['card'], fg=PAL['on'], selectcolor=PAL['bg'], activebackground=PAL['card'], command=self._adder_eval).grid(row=2, column=i * 2)
            tk.Label(fr, text=f'B{3 - i}', fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 8)).grid(row=3, column=i * 2)
            tk.Checkbutton(fr, variable=self._bits_B[i], bg=PAL['card'], fg=PAL['accent'], selectcolor=PAL['bg'], activebackground=PAL['card'], command=self._adder_eval).grid(row=4, column=i * 2)
        self._adder_out = tk.Label(fr, text='A + B = ?', fg=PAL['on'], bg=PAL['card'], font=('Cascadia Code', 14))
        self._adder_out.grid(row=5, column=0, columnspan=9, pady=10)