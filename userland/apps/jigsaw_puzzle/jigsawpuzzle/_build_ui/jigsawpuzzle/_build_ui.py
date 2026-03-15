# Generated method: JigsawPuzzle._build_ui
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import random
import time
import os

class JigsawPuzzle:
    def _build_ui(self):
        hdr = tk.Frame(self, bg=PAL['bg'], pady=12)
        hdr.pack(fill='x', padx=20)
        tk.Label(hdr, text='🧩 JIGSAW PUZZLE', font=('Segoe UI', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        ctrl = tk.Frame(hdr, bg=PAL['bg'])
        ctrl.pack(side='right')
        tk.Label(ctrl, text='Grid:', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 10)).pack(side='left', padx=(0, 4))
        self.grid_var = tk.IntVar(value=self.grid_n)
        for n in GRID_OPTIONS:
            tk.Radiobutton(ctrl, text=f'{n}×{n}', variable=self.grid_var, value=n, bg=PAL['bg'], fg=PAL['text'], selectcolor=PAL['panel'], activebackground=PAL['bg'], activeforeground=PAL['accent'], command=self._on_grid_change, font=('Segoe UI', 9)).pack(side='left', padx=4)
        tk.Button(ctrl, text='📂 Load Image', command=self._load_image, bg=PAL['accent2'], fg='white', relief='flat', padx=14, pady=6, font=('Segoe UI', 9, 'bold'), cursor='hand2').pack(side='left', padx=(10, 0))
        tk.Button(ctrl, text='🔀 Shuffle', command=self._shuffle, bg=PAL['panel'], fg=PAL['text'], relief='flat', padx=14, pady=6, font=('Segoe UI', 9, 'bold'), cursor='hand2').pack(side='left', padx=6)
        tk.Button(ctrl, text='👁 Preview', command=self._preview, bg=PAL['panel'], fg=PAL['text'], relief='flat', padx=14, pady=6, font=('Segoe UI', 9, 'bold'), cursor='hand2').pack(side='left', padx=0)
        tk.Frame(self, bg=PAL['border'], height=1).pack(fill='x', padx=20)
        main = tk.Frame(self, bg=PAL['bg'])
        main.pack(fill='both', expand=True, padx=20, pady=14)
        self.canvas_frame = tk.Frame(main, bg=PAL['panel'], relief='flat', bd=2, highlightbackground=PAL['border'], highlightthickness=1)
        self.canvas_frame.pack(side='left', fill='both', expand=True)
        self.canvas = tk.Canvas(self.canvas_frame, bg=PAL['panel'], highlightthickness=0, cursor='fleur')
        self.canvas.pack(fill='both', expand=True)
        side = tk.Frame(main, bg=PAL['bg'], width=220)
        side.pack(side='right', fill='y', padx=(16, 0))
        side.pack_propagate(False)
        stats = tk.Frame(side, bg=PAL['card'], padx=18, pady=18)
        stats.pack(fill='x', pady=(0, 14))
        tk.Label(stats, text='STATS', font=('Segoe UI', 9, 'bold'), fg=PAL['dim'], bg=PAL['card']).pack(anchor='w')
        self.lbl_moves = self._stat_row(stats, 'Moves', '0')
        self.lbl_time = self._stat_row(stats, 'Time', '00:00')
        self.lbl_grid = self._stat_row(stats, 'Grid', '4×4')
        self.lbl_tiles = self._stat_row(stats, 'Tiles', '16')
        tk.Label(side, text='COMPLETION', font=('Segoe UI', 9, 'bold'), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 6))
        self.progress_var = tk.DoubleVar(value=0)
        self.progress_bar = ttk.Progressbar(side, variable=self.progress_var, maximum=100, length=190)
        self.progress_bar.pack(fill='x')
        self.lbl_progress = tk.Label(side, text='0% Complete', font=('Segoe UI', 10, 'bold'), fg=PAL['accent'], bg=PAL['bg'])
        self.lbl_progress.pack(pady=(6, 16))
        tips = tk.Frame(side, bg=PAL['card'], padx=14, pady=14)
        tips.pack(fill='x')
        tk.Label(tips, text='HOW TO PLAY', font=('Segoe UI', 9, 'bold'), fg=PAL['dim'], bg=PAL['card']).pack(anchor='w', pady=(0, 8))
        for tip in ['📂 Load any image', '🔀 Shuffle tiles', '🖱 Drag tiles to swap', '👁 Preview original', '🏆 Place all correctly!']:
            tk.Label(tips, text=tip, font=('Segoe UI', 8), fg=PAL['text'], bg=PAL['card'], wraplength=180, justify='left').pack(anchor='w', pady=1)
        self.status = tk.Label(self, text='Load an image to begin, or use the built-in demo.', bg=PAL['accent'], fg='black', font=('Segoe UI', 9, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')
        self.canvas.bind('<ButtonPress-1>', self._on_press)
        self.canvas.bind('<B1-Motion>', self._on_drag)
        self.canvas.bind('<ButtonRelease-1>', self._on_release)
        self._start_demo()
        self._tick_clock()