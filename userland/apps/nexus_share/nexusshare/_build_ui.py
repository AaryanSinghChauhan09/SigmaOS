# Generated method: NexusShare._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading

class NexusShare:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='NEXUS SHARE PROTOCOL', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='📡 BROADCAST RADAR', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._start_radar).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.drop_fr = tk.Frame(self.workspace, bg=PAL['panel'], padx=20, pady=40, cursor='hand2')
        self.drop_fr.pack(fill='x', pady=(0, 20))
        tk.Label(self.drop_fr, text='⬇️ DROP FRAGMENTS HERE ⬇️', font=('Inter', 16, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack()
        tk.Label(self.drop_fr, text='(Encrypted P2P Tunneling via Wi-Fi Direct / BT-LE)', font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(pady=(5, 0))
        self.drop_fr.bind('<Button-1>', self._mock_file_select)
        tk.Label(self.workspace, text='VISIBLE SOVEREIGN NODES', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(10, 5))
        self.nodes_canvas = tk.Canvas(self.workspace, bg=PAL['bg'], highlightthickness=0, height=200)
        self.nodes_canvas.pack(fill='both', expand=True)
        self._draw_nodes([])
        self.status = tk.Label(self, text='IDLE | AWAITING PAYLOAD', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')