# Generated method: LogicSimulator._build_ui
import tkinter as tk
from tkinter import ttk, messagebox

class LogicSimulator:
    def _build_ui(self):
        for w in self.winfo_children():
            w.destroy()
        hdr = tk.Frame(self, bg='#11142A', height=60)
        hdr.pack(fill='x')
        tk.Label(hdr, text='🔌 INTERACTIVE LOGIC GATES PRO', fg='#F59E0B', bg='#11142A', font=('Segoe UI Bold', 14)).pack(pady=15)
        canv_fr = tk.Frame(self, bg='#0B0D17')
        canv_fr.pack(fill='both', expand=True)
        self.canvas = tk.Canvas(canv_fr, bg='#0D0F18', highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, padx=40, pady=40)
        ctrl = tk.Frame(self, bg='#11142A', height=100)
        ctrl.pack(fill='x')
        tk.Label(ctrl, text='Input A:', fg='white', bg='#11142A').pack(side='left', padx=10)
        self.btn_a = tk.Button(ctrl, text=str(self.sim_state['A']), width=4, command=lambda: self._toggle('A'), bg='#1A1E30', fg='white', relief='flat')
        self.btn_a.pack(side='left', padx=5)
        tk.Label(ctrl, text='Input B:', fg='white', bg='#11142A').pack(side='left', padx=10)
        self.btn_b = tk.Button(ctrl, text=str(self.sim_state['B']), width=4, command=lambda: self._toggle('B'), bg='#1A1E30', fg='white', relief='flat')
        self.btn_b.pack(side='left', padx=5)
        tk.Label(ctrl, text='Select Gate:', fg='white', bg='#11142A').pack(side='left', padx=20)
        self.gate_cb = ttk.Combobox(ctrl, values=['AND', 'OR', 'NAND', 'NOR', 'XOR', 'NOT'], width=10, state='readonly')
        self.gate_cb.set(self.sim_state.get('Gate', 'AND'))
        self.gate_cb.pack(side='left', padx=5)
        self.gate_cb.bind('<<ComboboxSelected>>', lambda e: self._update())
        tk.Button(ctrl, text='VIEW TRUTH TABLE', command=self._show_truth, bg='#6C63FF', fg='white', relief='flat', padx=15).pack(side='right', padx=30)
        self._update()