# Generated method: OmniToolsApp._build_qr_tab
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _build_qr_tab(self) -> None:
        tk.Label(self.tab_qr, text='QR Code Generator  (pure-Python, offline)', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        gf = tk.Frame(self.tab_qr, bg=PAL['panel'], padx=15, pady=15)
        gf.pack(fill='x', pady=5)
        tk.Label(gf, text='Data / URL:', bg=PAL['panel'], fg=PAL['dim']).pack(side='left')
        self.qr_data_entry = tk.Entry(gf, width=45, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.qr_data_entry.pack(side='left', padx=8)
        tk.Button(gf, text='GENERATE', bg=PAL['success'], fg='black', font=('Inter', 9, 'bold'), command=self._generate_qr).pack(side='left')
        self.qr_canvas = tk.Canvas(self.tab_qr, bg=PAL['panel'], width=420, height=420, highlightthickness=0)
        self.qr_canvas.pack(pady=15)
        tk.Label(self.tab_qr, text='Native QR rendering – no Pillow / qrcode library required.', font=('Inter', 8), fg=PAL['dim'], bg=PAL['bg']).pack()