# Generated method: PhysiologyHub._setup_resp
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

class PhysiologyHub:
    def _setup_resp(self, master):
        tk.Label(master, text='LUNG CAPACITY FLOWMETER', font=('Segoe UI Bold', 12), fg=PAL['neural'], bg=PAL['bg']).pack(pady=10)
        fr = tk.Frame(master, bg=PAL['bg'])
        fr.pack(pady=20)
        tk.Label(fr, text='TV (ml):', fg='white', bg=PAL['bg']).grid(row=0, column=0)
        self.tv = tk.Entry(fr, width=10)
        self.tv.insert(0, '500')
        self.tv.grid(row=0, column=1, padx=5)
        tk.Label(fr, text='IRV (ml):', fg='white', bg=PAL['bg']).grid(row=1, column=0)
        self.irv = tk.Entry(fr, width=10)
        self.irv.insert(0, '2500')
        self.irv.grid(row=1, column=1, padx=5)
        tk.Button(master, text='CALCULATE VITAL CAPACITY', command=self._calc_vc, bg=PAL['neural'], fg='white', relief='flat').pack(pady=10)
        self.vc_res = tk.Label(master, text='VC: -- ml', fg=PAL['text'], bg=PAL['bg'], font=('Segoe UI Bold', 14))
        self.vc_res.pack()