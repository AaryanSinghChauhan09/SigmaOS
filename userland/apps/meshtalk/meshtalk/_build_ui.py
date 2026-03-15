"""
Auto-split from userland\apps\meshtalk.py — MeshTalk._build_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time



class MeshTalk:
    def _build_ui(self):
        self.panes = ttk.PanedWindow(self, orient='horizontal')
        self.panes.pack(fill='both', expand=True)
        self.sidebar = tk.Frame(self.panes, bg=PAL['sidebar'], width=240, padx=20, pady=25)
        self.panes.add(self.sidebar, weight=1)
        self.sidebar.pack_propagate(False)
        tk.Label(self.sidebar, text='AETHER MESH', font=('Inter', 12, 'bold'), fg=PAL['accent'], bg=PAL['sidebar']).pack(anchor='w')
        tk.Label(self.sidebar, text='SECURE CHANNELS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar'], pady=(25, 10)).pack(anchor='w')
        for node in ['#general-mesh', '#dev-nodes', '#quantum-sync', '#aether-chat']:
            tk.Label(self.sidebar, text=node, font=('Inter', 10), fg=PAL['text'], bg=PAL['sidebar'], pady=8, cursor='hand2').pack(anchor='w')
        tk.Label(self.sidebar, text='NODES ONLINE', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar'], pady=(25, 10)).pack(anchor='w')
        for user in [('● Admin_Prime', PAL['success']), ('● Mesh_Node_A', PAL['success']), ('● Ghost_7', PAL['dim'])]:
            tk.Label(self.sidebar, text=user[0], font=('Inter', 9), fg=user[1], bg=PAL['sidebar'], pady=5).pack(anchor='w')
        self.chat_pane = ttk.PanedWindow(self.panes, orient='vertical')
        self.panes.add(self.chat_pane, weight=4)
        self.header = tk.Frame(self.chat_pane, bg=PAL['bg'], height=60, padx=25, pady=20)
        self.chat_pane.add(self.header, weight=1)
        tk.Label(self.header, text='#general-mesh', font=('Inter', 14, 'bold'), fg=PAL['text'], bg=PAL['bg']).pack(side='left')
        tk.Label(self.header, text='| P2P Tunnel: ENCRYPTED (AES-GCM)', font=('Inter', 9), fg=PAL['success'], bg=PAL['bg']).pack(side='left', padx=15, pady=8)
        self.log_fr = tk.Frame(self.chat_pane, bg=PAL['chat_bg'])
        self.chat_pane.add(self.log_fr, weight=6)
        self.chat_log = scrolledtext.ScrolledText(self.log_fr, bg=PAL['chat_bg'], fg=PAL['text'], font=('Inter', 11), state='disabled', borderwidth=0, padx=30, pady=30, insertbackground='white')
        self.chat_log.pack(fill='both', expand=True)
        self.entry_fr = tk.Frame(self.chat_pane, bg=PAL['bg'], pady=20, padx=25)
        self.chat_pane.add(self.entry_fr, weight=2)
        self.msg_var = tk.StringVar()
        self.entry = tk.Entry(self.entry_fr, textvariable=self.msg_var, bg='#000', fg=PAL['text'], font=('Inter', 11), insertbackground='white', borderwidth=0, highlightthickness=1, highlightbackground=PAL['border'], padx=15)
        self.entry.pack(fill='both', pady=5)
        self.entry.bind('<Return>', self.send_message)
        tk.Label(self.entry_fr, text='Neural Decryption: SHIM_ACTIVE | P2P LATENCY: 1.2ms', font=('Inter', 7, 'bold'), fg=PAL['dim'], bg=PAL['bg'], pady=5).pack(anchor='w')
        self.right_bar = tk.Frame(self.panes, bg=PAL['sidebar'], width=260, padx=20, pady=25)
        self.panes.add(self.right_bar, weight=1)
        self.right_bar.pack_propagate(False)
        tk.Label(self.right_bar, text='MESH HEALTH', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        self.health_canvas = tk.Canvas(self.right_bar, width=220, height=100, bg=PAL['sidebar'], highlightthickness=0)
        self.health_canvas.pack(pady=20)
        self._animate_health(0)
        self._item_val(self.right_bar, 'Active Nodes', '3,402', PAL['success'])
        self._item_val(self.right_bar, 'Mesh Encryption', 'SHA3-X-P2P', PAL['accent'])
