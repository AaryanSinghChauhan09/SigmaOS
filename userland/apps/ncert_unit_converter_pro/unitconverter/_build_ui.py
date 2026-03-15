# Generated method: UnitConverter._build_ui
import tkinter as tk
from tkinter import ttk

class UnitConverter:
    def _build_ui(self):
        hdr = tk.Frame(self, bg='#11142A', height=50)
        hdr.pack(fill='x')
        tk.Label(hdr, text='📏 EXHAUSTIVE UNIT CONVERTER', fg='#EC4899', bg='#11142A', font=('Segoe UI Bold', 12)).pack(pady=10)
        main = tk.Frame(self, bg='#0B0D17', padx=30, pady=20)
        main.pack(fill='both', expand=True)
        tk.Label(main, text='Category:', fg='white', bg='#0B0D17').grid(row=0, column=0, sticky='w', pady=5)
        self.cat_cb = ttk.Combobox(main, textvariable=self.cat_var, values=list(self.data.keys()), state='readonly')
        self.cat_cb.grid(row=0, column=1, sticky='ew', pady=5)
        self.cat_cb.bind('<<ComboboxSelected>>', self._update_units)
        tk.Label(main, text='Input Value:', fg='white', bg='#0B0D17').grid(row=1, column=0, sticky='w', pady=10)
        tk.Entry(main, textvariable=self.in_val, bg='#1A1E30', fg='white', relief='flat').grid(row=1, column=1, sticky='ew')
        tk.Label(main, text='From:', fg='white', bg='#0B0D17').grid(row=2, column=0, sticky='w', pady=5)
        self.from_cb = ttk.Combobox(main, textvariable=self.from_var, state='readonly')
        self.from_cb.grid(row=2, column=1, sticky='ew', pady=5)
        tk.Label(main, text='To:', fg='white', bg='#0B0D17').grid(row=3, column=0, sticky='w', pady=5)
        self.to_cb = ttk.Combobox(main, textvariable=self.to_var, state='readonly')
        self.to_cb.grid(row=3, column=1, sticky='ew', pady=5)
        tk.Button(main, text='CONVERT', command=self.convert, bg='#6C63FF', fg='white', relief='flat', pady=8).grid(row=4, column=0, columnspan=2, sticky='ew', pady=20)
        self.res_lbl = tk.Label(main, textvariable=self.out_val, fg='#00D26A', bg='#0B0D17', font=('Segoe UI Bold', 14))
        self.res_lbl.grid(row=5, column=0, columnspan=2, pady=10)
        self._update_units()