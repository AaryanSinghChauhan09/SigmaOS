# Generated method: OmniToolsApp._build_ui
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _build_ui(self) -> None:
        hdr = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        hdr.pack(side='top', fill='x', pady=15)
        tk.Label(hdr, text='🛠  OMNITOOLS – ALL-IN-ONE UTILITY SUITE', font=('Inter', 18, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        tk.Button(hdr, text='🔄 REFRESH OFFLINE DB', font=('Inter', 9, 'bold'), bg=PAL['warning'], fg='black', relief='flat', padx=12, pady=7, command=self._refresh_offline_db).pack(side='right')
        ws = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        ws.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(ws, style='Omni.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.tab_timer = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tab_converter = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tab_calc = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tab_qr = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tab_fin = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tab_misc = tk.Frame(self.tabs, bg=PAL['bg'], padx=20, pady=20)
        self.tabs.add(self.tab_timer, text='⏱  TIMER & POMODORO')
        self.tabs.add(self.tab_converter, text='🔄  CONVERTERS')
        self.tabs.add(self.tab_calc, text='🧮  CALCULATORS')
        self.tabs.add(self.tab_qr, text='📱  QR CODE')
        self.tabs.add(self.tab_fin, text='💰  FINANCE')
        self.tabs.add(self.tab_misc, text='⚙  MISC')
        self._build_timer_tab()
        self._build_converter_tab()
        self._build_calculator_tab()
        self._build_qr_tab()
        self._build_finance_tab()
        self._build_misc_tab()
        self.status = tk.Label(self, text='Ready – 100 % offline, zero third-party dependencies', bg=PAL['accent_dim'], fg='white', font=('Inter', 9, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')