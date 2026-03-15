# Generated method: ArcadePage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL

class ArcadePage:
    def build(self):
        is_child = self.controller._is_child_mode()
        title = 'KIDDIE PLAYGROUND' if is_child else 'SOVEREIGN ARCADE'
        subtitle = 'Safe & Fun Games for Little Champions!' if is_child else 'Zero-Telemetry Clean-Room Game Engine (64+ Logic Modules)'
        self.controller._build_page_header(self, title, subtitle)
        ctrl = tk.Frame(self, bg=PAL['nav_bg'], pady=10)
        ctrl.pack(fill='x', padx=10)
        search_head = '🔎 FIND TOY:' if is_child else '🔎 SEARCH:'
        tk.Label(ctrl, text=search_head, font=FONT_SMALL, fg=PAL['dim'], bg=PAL['nav_bg']).pack(side='left', padx=(20, 10))
        self.game_query = tk.StringVar()
        self.game_query.trace_add('write', lambda *args: self.refresh_game_grid())
        search_ent = tk.Entry(ctrl, textvariable=self.game_query, bg=PAL['bg'], fg=PAL['text'], insertbackground=PAL['accent'], font=FONT_SMALL, relief='flat', width=30)
        search_ent.pack(side='left', padx=5)
        tk.Label(ctrl, text='TYPE:' if is_child else 'MODE:', font=FONT_SMALL, fg=PAL['dim'], bg=PAL['nav_bg']).pack(side='left', padx=(30, 10))
        self.cat_filter = tk.StringVar(value='All')
        cat_list = ['All', 'Fun Logic', 'Puzzles', 'Coloring'] if is_child else ['All', 'Board Strategy', 'Puzzle / Logic', 'Brain Training', 'Action / Retro']
        cat_cb = ttk.Combobox(ctrl, textvariable=self.cat_filter, values=cat_list, state='readonly', width=15)
        cat_cb.pack(side='left', padx=5)
        cat_cb.bind('<<ComboboxSelected>>', lambda e: self.refresh_game_grid())
        self.scroll_fr = tk.Frame(self, bg=PAL['bg'])
        self.scroll_fr.pack(fill='both', expand=True, padx=10, pady=10)
        self.grid_inner = tk.Frame(self.scroll_fr, bg=PAL['bg'])
        self.grid_inner.pack(fill='both', expand=True)
        self.refresh_game_grid()