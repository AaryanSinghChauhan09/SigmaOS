# Generated method: ExplorerPage.build
import tkinter as tk
from tkinter import ttk, messagebox
import os
import subprocess
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ExplorerPage:
    def build(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        path_fr = tk.Frame(body, bg=PAL['bg'])
        path_fr.pack(fill='x', pady=(0, 10))
        ttk.Button(path_fr, text='??', width=3, command=self._go_up).pack(side='left')
        self.path_ent = ttk.Entry(path_fr, textvariable=self.current_path)
        self.path_ent.pack(side='left', fill='x', expand=True, padx=5)
        self.path_ent.bind('<Return>', lambda e: self._load_dir())
        ttk.Button(path_fr, text='REFRESH', command=self._load_dir).pack(side='left')
        ttk.Button(path_fr, text='OS ROOT', command=lambda: [self.current_path.set(self.kernel._root), self._load_dir()]).pack(side='left', padx=5)
        paned = tk.PanedWindow(body, orient='horizontal', bg=PAL['border'], sashwidth=4)
        paned.pack(fill='both', expand=True)
        side = tk.Frame(paned, bg=PAL['bg2'], width=200)
        paned.add(side)
        self.gui._card(side, 'Quick Access').pack(fill='x')
        for loc in ['Desktop', 'Downloads', 'Documents', 'SigmaCore']:
            b = tk.Button(side, text=f'?? {loc}', font=FONT_SMALL, fg=PAL['text'], bg=PAL['bg2'], relief='flat', anchor='w', padx=10)
            b.pack(fill='x')
        silo_card = self.gui._card(side, 'Active Silos')
        silo_card.master.pack(fill='x', pady=10)
        self.silo_list = tk.Frame(silo_card, bg=PAL['card'])
        self.silo_list.pack(fill='x')
        self.main_list = tk.Frame(paned, bg=PAL['bg'])
        paned.add(self.main_list)
        cols = ('Name', 'Size', 'Type', 'Integrity')
        self.tree = ttk.Treeview(self.main_list, columns=cols, show='headings', selectmode='browse')
        for c in cols:
            self.tree.heading(c, text=c)
        self.tree.column('Name', width=300)
        self.tree.column('Integrity', width=100)
        self.tree.pack(fill='both', expand=True)
        self.tree.bind('<Double-1>', self._on_double_click)
        self.tree.bind('<Button-3>', self._show_context_menu)
        self._load_dir()