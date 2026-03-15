# Generated method: VennVisualizer._build_ui
import tkinter as tk

class VennVisualizer:
    def _build_ui(self):
        ctrl = tk.Frame(self, bg='#11142A', pady=20)
        ctrl.pack(fill='x')
        tk.Label(ctrl, text='Set A:', fg='white', bg='#11142A').pack(side='left', padx=10)
        tk.Entry(ctrl, textvariable=self.set_a, width=20).pack(side='left', padx=5)
        tk.Label(ctrl, text='Set B:', fg='white', bg='#11142A').pack(side='left', padx=10)
        tk.Entry(ctrl, textvariable=self.set_b, width=20).pack(side='left', padx=5)
        tk.Button(ctrl, text='UPDATE', command=self._draw, bg='#6C63FF', fg='white', relief='flat').pack(side='left', padx=20)
        self.canvas = tk.Canvas(self, bg='#0D0F18', highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, padx=40, pady=40)
        self.info = tk.Label(self, text='', fg='#E8E8F0', bg='#0D0F18', font=('Segoe UI', 11), pady=20)
        self.info.pack()