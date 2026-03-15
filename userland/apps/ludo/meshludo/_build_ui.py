# Generated method: MeshLudo._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class MeshLudo:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=40, pady=30)
        head.pack(fill='x')
        tk.Label(head, text='MESH LUDO PRO', font=('Inter Bold', 26), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.status = tk.Label(head, text="RED'S STRATEGIC TURN", font=('Inter', 12, 'bold'), fg=PAL['red'], bg=PAL['bg'])
        self.status.pack(side='right', pady=10)
        body = tk.Frame(self, bg=PAL['bg'], padx=40)
        body.pack(fill='both', expand=True)
        board_container = tk.Frame(body, bg='#1A1B23', padx=4, pady=4)
        board_container.pack(side='left')
        self.canv = tk.Canvas(board_container, width=600, height=600, bg='#050508', highlightthickness=0)
        self.canv.pack()
        self._draw_board()
        side = tk.Frame(body, bg=PAL['panel'], width=300, padx=25, pady=25)
        side.pack(side='right', fill='y', padx=(30, 0))
        side.pack_propagate(False)
        tk.Label(side, text='NEURAL HEURISTICS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self.dice_lbl = tk.Label(side, text='⚀', font=('Inter', 120), fg='white', bg=PAL['panel'])
        self.dice_lbl.pack(pady=20)
        tk.Button(side, text='ROLL SECURE DICE', font=('Inter', 10, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=40, pady=18, command=self._roll).pack(fill='x')
        tk.Label(side, text='LOG: VECTOR QUANTIZATION', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel'], pady=20).pack(anchor='w')
        self.log_txt = tk.Text(side, bg='#000', fg=PAL['text'], font=('JetBrains Mono', 8), borderwidth=0, padx=10, pady=10, height=15)
        self.log_txt.pack(fill='both', expand=True)
        tk.Label(self, text='P2P MESH: ACTIVE | LATENCY: 0.8ms | ENCRYPTION: SHA-3 | USP: ZERO-LATENCY PIECE QUANTIZATION', font=('Inter', 8, 'bold'), bg=PAL['panel'], fg=PAL['dim'], pady=8).pack(side='bottom', fill='x')