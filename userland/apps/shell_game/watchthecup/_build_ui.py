"""
Auto-split from userland\apps\shell_game.py — WatchTheCup._build_ui
"""

import tkinter as tk
from tkinter import messagebox
import random
import time



class WatchTheCup:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['bg'], height=60)
        hdr.pack(fill='x', padx=24, pady=(14, 0))
        hdr.pack_propagate(False)
        tk.Label(hdr, text='🎩 WATCH THE CUP!', font=('Segoe UI', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        diff_fr = tk.Frame(hdr, bg=PAL['bg'])
        diff_fr.pack(side='right')
        tk.Label(diff_fr, text='Difficulty:', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 9)).pack(side='left', padx=(0, 6))
        self.diff_var = tk.StringVar(value=self.difficulty)
        for d in SPEEDS:
            col = PAL['accent'] if d == self.difficulty else PAL['dim']
            tk.Radiobutton(diff_fr, text=d, variable=self.diff_var, value=d, bg=PAL['bg'], fg=col, selectcolor=PAL['panel'], activebackground=PAL['bg'], command=self._on_diff_change, font=('Segoe UI', 8)).pack(side='left', padx=3)
        self.canvas = tk.Canvas(self, width=760, height=400, bg=PAL['panel'], highlightthickness=0)
        self.canvas.pack(pady=(10, 4))
        self.lbl_instr = tk.Label(self, text='Press  ▶ PLAY  to start a round.', font=('Segoe UI', 13), fg=PAL['text'], bg=PAL['bg'])
        self.lbl_instr.pack(pady=4)
        stats_fr = tk.Frame(self, bg=PAL['card'])
        stats_fr.pack(fill='x', padx=24, pady=(4, 0))
        self.lbl_score = self._stat(stats_fr, 'Score', '0')
        self.lbl_streak = self._stat(stats_fr, 'Streak 🔥', '0')
        self.lbl_best = self._stat(stats_fr, 'Best Streak', '0')
        self.lbl_round = self._stat(stats_fr, 'Round', '0')
        btn_fr = tk.Frame(self, bg=PAL['bg'])
        btn_fr.pack(pady=10)
        self.btn_play = tk.Button(btn_fr, text='▶  PLAY', command=self._start_round, font=('Segoe UI', 12, 'bold'), bg=PAL['accent2'], fg='white', relief='flat', padx=28, pady=10, cursor='hand2')
        self.btn_play.pack(side='left', padx=8)
        self.btn_reset = tk.Button(btn_fr, text='↺  Reset Score', command=self._reset, font=('Segoe UI', 10), bg=PAL['panel'], fg=PAL['dim'], relief='flat', padx=18, pady=10, cursor='hand2')
        self.btn_reset.pack(side='left', padx=8)
        self.status = tk.Label(self, text='Ready. Pick a difficulty and press PLAY.', bg=PAL['accent2'], fg='white', font=('Segoe UI', 9, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')
