# Generated method: OpticsBench._build_ui
import tkinter as tk
from tkinter import ttk

class OpticsBench:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['optics'], height=70)
        hdr.pack(fill='x')
        tk.Label(hdr, text='🔭 VIRTUAL OPTICS BENCH', font=('Segoe UI Bold', 18), fg=PAL['accent'], bg=PAL['optics']).pack(pady=15)
        ctrl = tk.Frame(self, bg=PAL['bg'], padx=40, pady=20)
        ctrl.pack(fill='x')
        tk.Label(ctrl, text='Selection:', fg=PAL['text'], bg=PAL['bg']).grid(row=0, column=0, padx=10)
        mode_cb = ttk.Combobox(ctrl, textvariable=self.mode, values=['Concave Mirror', 'Convex Mirror', 'Convex Lens', 'Concave Lens'], state='readonly')
        mode_cb.grid(row=0, column=1, padx=10)
        mode_cb.bind('<<ComboboxSelected>>', lambda e: self._update())
        tk.Label(ctrl, text='Focal Length (f):', fg=PAL['text'], bg=PAL['bg']).grid(row=0, column=2, padx=10)
        self.f_scale = tk.Scale(ctrl, from_=10, to=50, orient='horizontal', bg=PAL['bg'], fg='white', highlightthickness=0, command=lambda x: self._update())
        self.f_scale.set(20)
        self.f_scale.grid(row=0, column=3, padx=10)
        tk.Label(ctrl, text='Object Distance (u):', fg=PAL['text'], bg=PAL['bg']).grid(row=0, column=4, padx=10)
        self.u_scale = tk.Scale(ctrl, from_=5, to=150, orient='horizontal', bg=PAL['bg'], fg='white', highlightthickness=0, command=lambda x: self._update())
        self.u_scale.set(40)
        self.u_scale.grid(row=0, column=5, padx=10)
        self.out_lbl = tk.Label(self, text='IMAGE DATA: Wait...', font=('Consolas', 11), fg=PAL['ray'], bg=PAL['bg'])
        self.out_lbl.pack(pady=10)
        self.canvas = tk.Canvas(self, bg='#050508', highlightthickness=1, highlightbackground='#1A1C25')
        self.canvas.pack(fill='both', expand=True, padx=40, pady=20)
        self._update()