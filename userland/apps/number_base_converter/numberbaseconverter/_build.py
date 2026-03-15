# Generated method: NumberBaseConverter._build
import tkinter as tk
from tkinter import ttk

class NumberBaseConverter:
    def _build(self):
        tk.Frame(self, bg=PAL['panel'], height=50).pack(fill='x')
        hdr = self.children[list(self.children)[-1]]
        hdr.pack_propagate(False)
        tk.Label(hdr, text='⬡ NUMBER BASE CONVERTER', fg=PAL['accent'], bg=PAL['panel'], font=('Segoe UI Bold', 13)).pack(side='left', padx=18, pady=10)
        body = tk.Frame(self, bg=PAL['bg'], padx=24, pady=18)
        body.pack(fill='both', expand=True)
        self._vars = {}
        self._entries = {}
        bases = [('Decimal (Base 10)', 'dec', 10), ('Binary (Base 2)', 'bin', 2), ('Octal (Base 8)', 'oct', 8), ('Hexadecimal (Base 16)', 'hex', 16)]
        for i, (lbl, key, base) in enumerate(bases):
            tk.Label(body, text=lbl, fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 9)).grid(row=i, column=0, sticky='w', pady=6)
            v = tk.StringVar()
            e = tk.Entry(body, textvariable=v, bg=PAL['card'], fg='white', font=('Cascadia Code', 13), insertbackground='white', relief='flat', highlightthickness=1, highlightbackground=PAL['border'], width=28)
            e.grid(row=i, column=1, padx=10, pady=6, sticky='w')
            self._vars[key] = (v, base)
            self._entries[key] = e
            v.trace_add('write', lambda *_, k=key: self._on_change(k))
        tk.Label(body, text='Bit width', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 9)).grid(row=4, column=0, sticky='w', pady=6)
        self._bits = tk.StringVar(value='32')
        ttk.Combobox(body, textvariable=self._bits, values=['8', '16', '32', '64'], state='readonly', width=8).grid(row=4, column=1, sticky='w', padx=10)
        sep = tk.Frame(body, bg=PAL['border'], height=1)
        sep.grid(row=5, column=0, columnspan=2, sticky='ew', pady=12)
        ops_fr = tk.Frame(body, bg=PAL['bg'])
        ops_fr.grid(row=6, column=0, columnspan=2, sticky='w')
        tk.Label(ops_fr, text='Bitwise:', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 9)).pack(side='left')
        for op in ('AND', 'OR', 'XOR', 'NOT', '<<', '>>'):
            tk.Button(ops_fr, text=op, bg=PAL['card'], fg=PAL['text'], font=('Segoe UI', 8), relief='flat', padx=8, pady=4, command=lambda o=op: self._bitwise(o)).pack(side='left', padx=3)
        self._result = tk.Label(body, text='', fg=PAL['accent'], bg=PAL['bg'], font=('Cascadia Code', 12), wraplength=500, justify='left')
        self._result.grid(row=7, column=0, columnspan=2, sticky='w', pady=10)
        sep2 = tk.Frame(body, bg=PAL['border'], height=1)
        sep2.grid(row=8, column=0, columnspan=2, sticky='ew', pady=8)
        tk.Label(body, text='ASCII Quick Ref (32-127)', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 8, 'bold')).grid(row=9, column=0, columnspan=2, sticky='w')
        ascii_fr = tk.Frame(body, bg=PAL['card'])
        ascii_fr.grid(row=10, column=0, columnspan=2, sticky='w', pady=4)
        for i, code in enumerate(range(32, 128)):
            ch = chr(code)
            tk.Label(ascii_fr, text=f'{code}={ch}', fg=PAL['dim'], bg=PAL['card'], font=('Cascadia Code', 7), width=6).grid(row=i // 20, column=i % 20)