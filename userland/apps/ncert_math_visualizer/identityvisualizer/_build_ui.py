# Generated method: IdentityVisualizer._build_ui
import tkinter as tk

class IdentityVisualizer:
    def _build_ui(self):
        ctrl = tk.Frame(self, bg='#13162A', pady=20)
        ctrl.pack(fill='x')
        tk.Label(ctrl, text='Length a:', fg='white', bg='#13162A').pack(side='left', padx=10)
        tk.Scale(ctrl, from_=20, to=200, variable=self.a, orient='horizontal', command=lambda x: self._draw(), bg='#13162A', fg='white', highlightthickness=0).pack(side='left', padx=10)
        tk.Label(ctrl, text='Length b:', fg='white', bg='#13162A').pack(side='left', padx=10)
        tk.Scale(ctrl, from_=20, to=200, variable=self.b, orient='horizontal', command=lambda x: self._draw(), bg='#13162A', fg='white', highlightthickness=0).pack(side='left', padx=10)
        self.canvas.destroy()
        self.canvas = tk.Canvas(self, bg='#0D0F18', highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, padx=50, pady=50)
        self.label.destroy()
        self.label = tk.Label(self, text='', font=('Segoe UI Bold', 14), fg='#6C63FF', bg='#0D0F18', pady=20)
        self.label.pack()