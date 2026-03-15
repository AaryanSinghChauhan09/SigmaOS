# Generated method: StorePage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class StorePage:
    def _build_ui(self):
        is_child = self.gui._is_child_mode()
        ctrl_fr = tk.Frame(self, bg=PAL['bg'], pady=8)
        ctrl_fr.pack(fill='x', padx=20)
        self._search_v = tk.StringVar()
        search_e = ttk.Entry(ctrl_fr, textvariable=self._search_v, width=28)
        search_e.pack(side='left', padx=(0, 10))
        tk.Label(ctrl_fr, text='🔍', fg=PAL['dim'], bg=PAL['bg']).pack(side='left', padx=(0, 10))
        self._store_cat = tk.StringVar(value='All')
        categories = ['All', 'Games', 'Productivity', 'Media'] if is_child else ['All', 'AI', 'Games', 'Dev', 'Security', 'Productivity', 'Media']
        for cat in categories:
            b = tk.Button(ctrl_fr, text=cat, font=FONT_BOLD, bg=PAL['bg2'], fg=PAL['dim'], relief='flat', padx=10, pady=4, command=lambda c=cat: [self._store_cat.set(c), self._refresh_grid()])
            b.pack(side='left', padx=3)
        container = tk.Frame(self, bg=PAL['bg'])
        container.pack(fill='both', expand=True, padx=20, pady=10)
        self._canvas = tk.Canvas(container, bg=PAL['bg'], highlightthickness=0)
        self._canvas.pack(side='left', fill='both', expand=True)
        sb = ttk.Scrollbar(container, orient='vertical', command=self._canvas.yview)
        sb.pack(side='right', fill='y')
        self._canvas.configure(yscrollcommand=sb.set)
        self._grid = tk.Frame(self._canvas, bg=PAL['bg'])
        self._canvas.create_window((0, 0), window=self._grid, anchor='nw')
        self._grid.bind('<Configure>', lambda e: self._canvas.configure(scrollregion=self._canvas.bbox('all')))
        self._all_apps = [('♟ Happy Chess', 'Games', 'Brain Games for Kids.', '♟', 'sigma.game.chess', PAL['purple'], True), ('🎲 Fun Ludo', 'Games', 'Play Ludo with Friends.', '🎲', 'sigma.game.ludo', '#FF9F0A', True), ('🚪 Welcome Friend', 'Productivity', 'Learn how to use SigmaOS.', '🚪', 'sigma.sys.welcome', PAL['accent'], True), ('📝 Magic Writer', 'Productivity', 'Write and draw stories.', '📝', 'sigma.prod.writer', '#34C759', True), ('♫ Happy Musics', 'Media', 'Listen to happy music.', '♫', 'sigma.media.pulseplay', '#5AC8FA', True), ('🎨 Color Paint', 'Media', 'Paint beautiful pictures.', '🎨', 'sigma.media.aurapaint', '#FF6B96', True), ('🧬 OS Brain', 'AI', 'OS Guide.', '🧬', 'sigma.ai.nexus_ai', PAL['cyan'], False), ('⚡ AI Secret', 'AI', 'AI Power.', '⚡', 'sigma.ai.antigravity', '#3D9EFF', False), ('🔒 Safety Robot', 'Security', 'Safety Scan.', '🔒', 'sigma.sys.sentinel', '#FF453A', False)]
        self._search_v.trace_add('write', lambda *_: self._refresh_grid())
        self._refresh_grid()