# Generated method: PhysiologyHub._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import math, random

class PhysiologyHub:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['panel'], height=80)
        hdr.pack(fill='x')
        tk.Label(hdr, text='💓 HUMAN PHYSIOLOGY MASTER', font=('Segoe UI Bold', 20), fg=PAL['heart'], bg=PAL['panel']).pack(pady=20)
        tabs = ttk.Notebook(self)
        tabs.pack(fill='both', expand=True, padx=20, pady=20)
        circ = tk.Frame(tabs, bg=PAL['bg'])
        tabs.add(circ, text='CIRCULATORY')
        self._setup_circ(circ)
        resp = tk.Frame(tabs, bg=PAL['bg'])
        tabs.add(resp, text='RESPIRATORY')
        self._setup_resp(resp)
        neur = tk.Frame(tabs, bg=PAL['bg'])
        tabs.add(neur, text='NEURAL')
        self._setup_neur(neur)