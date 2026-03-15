# Generated method: ChemBalancer._build_ui
import tkinter as tk
from tkinter import ttk, messagebox

class ChemBalancer:
    def _build_ui(self):
        hdr = tk.Frame(self, bg='#11142A', height=60)
        hdr.pack(fill='x')
        tk.Label(hdr, text='🧪 CHEMICAL EQUATION BALANCER', fg='#22C55E', bg='#11142A', font=('Segoe UI Bold', 14)).pack(pady=15)
        main = tk.Frame(self, bg='#0B0D17', pady=30)
        main.pack(fill='both', expand=True)
        tk.Label(main, text='Enter Equation (e.g., Fe + Cl2 -> FeCl3):', fg='#E8E8F0', bg='#0B0D17', font=('Segoe UI', 10)).pack()
        ent = tk.Entry(main, textvariable=self.input_var, bg='#1A1E30', fg='white', font=('Consolas', 14), width=50, relief='flat', insertbackground='white')
        ent.pack(pady=15)
        ent.bind('<Return>', lambda e: self.balance())
        btn = tk.Button(main, text='BALANCE EQUATION', command=self.balance, bg='#6C63FF', fg='white', font=('Segoe UI Bold', 10), relief='flat', padx=20, pady=10)
        btn.pack(pady=10)
        self.res_lbl.destroy()
        self.res_lbl = tk.Label(main, text='', fg='#00D26A', bg='#0B0D17', font=('Segoe UI Bold', 16))
        self.res_lbl.pack(pady=30)