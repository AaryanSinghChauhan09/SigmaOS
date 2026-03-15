# Generated method: GuideApex._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext
import time

class GuideApex:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=40, pady=30)
        head.pack(fill='x')
        tk.Label(head, text='📖 GUIDE APEX', font=('Inter Bold', 24), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        tk.Label(head, text='Mastering Sovereign Computing', font=('Inter', 10), fg=PAL['dim'], bg=PAL['bg']).pack(side='left', padx=20, pady=10)
        self.nb = ttk.Notebook(self)
        self.nb.pack(fill='both', expand=True, padx=40, pady=(0, 40))
        self._build_intro_tab()
        self._build_kernel_tab()
        self._build_security_tab()
        self._build_ai_tab()