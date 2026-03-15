"""
Auto-split from userland\apps\spot_it.py — SpotItGame._build_ui
"""

import tkinter as tk
from tkinter import messagebox
import random
import time
import math



class SpotItGame:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['bg'])
        hdr.pack(fill='x', padx=20, pady=(14, 4))
        tk.Label(hdr, text='🔍 SPOT IT — FIND THE TARGET!', font=('Segoe UI', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        ctrl = tk.Frame(hdr, bg=PAL['bg'])
        ctrl.pack(side='right')
        tk.Label(ctrl, text='Level:', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 9)).pack(side='left')
        self.level_var = tk.IntVar(value=0)
        for i, lv in enumerate(LEVELS):
            tk.Radiobutton(ctrl, text=lv['name'], variable=self.level_var, value=i, bg=PAL['bg'], fg=PAL['text'], selectcolor=PAL['panel'], activebackground=PAL['bg'], command=lambda: None, font=('Segoe UI', 8)).pack(side='left', padx=3)
        main = tk.Frame(self, bg=PAL['bg'])
        main.pack(fill='both', expand=True, padx=20, pady=6)
        left = tk.Frame(main, bg=PAL['bg'], width=180)
        left.pack(side='left', fill='y')
        left.pack_propagate(False)
        tgt_card = tk.Frame(left, bg=PAL['card'], padx=16, pady=16)
        tgt_card.pack(fill='x', pady=(0, 12))
        tk.Label(tgt_card, text='FIND THIS', font=('Segoe UI', 9, 'bold'), fg=PAL['dim'], bg=PAL['card']).pack()
        self.tgt_canvas = tk.Canvas(tgt_card, width=120, height=120, bg=PAL['panel'], highlightthickness=0)
        self.tgt_canvas.pack(pady=8)
        self.lbl_tgt_name = tk.Label(tgt_card, text='—', font=('Segoe UI', 10, 'bold'), fg=PAL['accent'], bg=PAL['card'])
        self.lbl_tgt_name.pack()
        timer_card = tk.Frame(left, bg=PAL['card'], padx=16, pady=14)
        timer_card.pack(fill='x', pady=(0, 12))
        tk.Label(timer_card, text='TIME', font=('Segoe UI', 8, 'bold'), fg=PAL['dim'], bg=PAL['card']).pack()
        self.lbl_timer = tk.Label(timer_card, text='—', font=('Segoe UI', 30, 'bold'), fg=PAL['warning'], bg=PAL['card'])
        self.lbl_timer.pack()
        self.timer_bar_var = tk.DoubleVar(value=100)
        self.timer_bar = tk.Canvas(timer_card, width=140, height=8, bg=PAL['panel'], highlightthickness=0)
        self.timer_bar.pack()
        stats_card = tk.Frame(left, bg=PAL['card'], padx=16, pady=12)
        stats_card.pack(fill='x')
        self.lbl_score = self._mini_stat(stats_card, 'Score', '0')
        self.lbl_combo = self._mini_stat(stats_card, 'Combo 🔥', '×0')
        self.lbl_best = self._mini_stat(stats_card, 'Best Combo', '×0')
        self.lbl_round = self._mini_stat(stats_card, 'Round', '0')
        self.canvas = tk.Canvas(main, width=self.CANVAS_W, height=self.CANVAS_H, bg=PAL['panel'], highlightthickness=1, highlightbackground=PAL['border'])
        self.canvas.pack(side='left', fill='both', expand=True, padx=(14, 0))
        self.canvas.bind('<Button-1>', self._on_click)
        btn_fr = tk.Frame(self, bg=PAL['bg'])
        btn_fr.pack(pady=8)
        self.btn_play = tk.Button(btn_fr, text='▶  NEW ROUND', command=self._new_round, font=('Segoe UI', 12, 'bold'), bg=PAL['accent2'], fg='white', relief='flat', padx=28, pady=10, cursor='hand2')
        self.btn_play.pack(side='left', padx=8)
        self.btn_reset = tk.Button(btn_fr, text='↺  Reset', command=self._reset_game, font=('Segoe UI', 10), bg=PAL['panel'], fg=PAL['dim'], relief='flat', padx=18, pady=10, cursor='hand2')
        self.btn_reset.pack(side='left', padx=8)
        self.status = tk.Label(self, text='Press  ▶ NEW ROUND  to start.', bg=PAL['accent2'], fg='white', font=('Segoe UI', 9, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')
        self.canvas.create_text(self.CANVAS_W // 2, self.CANVAS_H // 2, text='Press  ▶ NEW ROUND  to begin!', font=('Segoe UI', 18, 'bold'), fill=PAL['dim'], tags='msg')
