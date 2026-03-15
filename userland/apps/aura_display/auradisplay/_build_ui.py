# Generated method: AuraDisplay._build_ui
import tkinter as tk
from tkinter import ttk, messagebox

class AuraDisplay:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='AURA VISUAL MATRIX', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🚀 APPLY NEURAL SHIFT', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='black', relief='flat', padx=15, pady=8, command=self._apply_matrix).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        ctrl_fr = tk.Frame(self.workspace, bg=PAL['panel'], padx=20, pady=20)
        ctrl_fr.pack(fill='x', pady=10)
        tk.Label(ctrl_fr, text='LUMEN GAIN PARAMETERS', font=('Inter', 12, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        bf = tk.Frame(ctrl_fr, bg=PAL['panel'], pady=10)
        bf.pack(fill='x')
        tk.Label(bf, text='ABSOLUTE LUMINANCE', font=('Inter', 9), fg=PAL['text'], bg=PAL['panel'], width=20, anchor='w').pack(side='left')
        ttk.Scale(bf, from_=0, to=100, variable=self.brightness, style='Aura.Horizontal.TScale', length=300).pack(side='left', padx=10)
        tk.Label(bf, textvariable=self.brightness, font=('Inter', 9, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(side='left')
        blf = tk.Frame(ctrl_fr, bg=PAL['panel'], pady=10)
        blf.pack(fill='x')
        tk.Label(blf, text='NEURAL BLUE ATTENUATION', font=('Inter', 9), fg=PAL['text'], bg=PAL['panel'], width=20, anchor='w').pack(side='left')
        ttk.Scale(blf, from_=0, to=100, variable=self.blue_filter, style='Aura.Horizontal.TScale', length=300).pack(side='left', padx=10)
        tk.Label(blf, textvariable=self.blue_filter, font=('Inter', 9, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(side='left')
        sync_fr = tk.Frame(self.workspace, bg=PAL['panel'], padx=20, pady=20)
        sync_fr.pack(fill='x', pady=10)
        tk.Label(sync_fr, text='BIOMETRIC RYHTHM LOCK', font=('Inter', 12, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        cb = tk.Checkbutton(sync_fr, text='Engage Geo-Temporal Rhythm (Auto-Adjust via Solar Arc)', variable=self.circadian_sync, bg=PAL['panel'], fg=PAL['success'], selectcolor=PAL['bg'], activebackground=PAL['panel'], font=('Inter', 9))
        cb.pack(anchor='w')
        self.status = tk.Label(self, text='AURA MATRIX ONLINE | WAITING FOR CALIBRATION CUBE', bg=PAL['sidebar'], fg=PAL['text'], font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')