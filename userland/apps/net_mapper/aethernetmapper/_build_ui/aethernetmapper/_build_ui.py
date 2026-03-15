# Generated method: AetherNetMapper._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random

class AetherNetMapper:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='AETHER NET MAPPER', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        nav_btns = [('🌐 SONAR PING', self._sonar_ping), ('🚨 ROGUE DETECT', self._rogue_scan)]
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        conf_fr = tk.Frame(self.workspace, bg=PAL['panel'], padx=15, pady=15)
        conf_fr.pack(fill='x', pady=(0, 20))
        tk.Label(conf_fr, text='TARGET VECTOR (IP/DOMAIN):', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(side='left')
        self.ip_entry = tk.Entry(conf_fr, font=('Consolas', 12), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.ip_entry.pack(side='left', padx=10, fill='x', expand=True)
        self.ip_entry.insert(0, sys_ip())
        tk.Button(conf_fr, text='INITIATE PORT RESONANCE', font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='black', relief='flat', padx=15, pady=6, command=self._start_scan).pack(side='right')
        self.content_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.content_fr.pack(fill='both', expand=True)
        self.radar_fr = tk.Frame(self.content_fr, bg=PAL['panel'], width=400, padx=15, pady=15)
        self.radar_fr.pack(side='left', fill='both', padx=(0, 10))
        self.radar_fr.pack_propagate(False)
        tk.Label(self.radar_fr, text='TOPOGRAPHY MATRIX', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.canvas = tk.Canvas(self.radar_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True, pady=10)
        self._draw_radar(0)
        self.term_fr = tk.Frame(self.content_fr, bg=PAL['panel'], padx=15, pady=15)
        self.term_fr.pack(side='left', fill='both', expand=True, padx=(10, 0))
        tk.Label(self.term_fr, text='PACKET INTERCEPT LOG', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.terminal = tk.Text(self.term_fr, bg=PAL['bg'], fg=PAL['success'], font=('Consolas', 10), relief='flat')
        self.terminal.pack(fill='both', expand=True, pady=10)
        self.terminal.insert(tk.END, '>>> AETHER NETWORK ENGINE ONLINE.\n')
        self.terminal.config(state=tk.DISABLED)