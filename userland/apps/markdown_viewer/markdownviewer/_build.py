"""
Auto-split from userland\apps\markdown_viewer.py — MarkdownViewer._build
"""

import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os



class MarkdownViewer:
    def _build(self):
        tb = tk.Frame(self, bg=PAL['panel'], height=50)
        tb.pack(fill='x')
        tb.pack_propagate(False)
        tk.Label(tb, text='📄 MARKDOWN VIEWER', fg=PAL['accent'], bg=PAL['panel'], font=('Segoe UI Bold', 13)).pack(side='left', padx=18, pady=10)
        tk.Button(tb, text='📂 Open', bg=PAL['card'], fg=PAL['text'], font=('Segoe UI', 9), relief='flat', padx=12, command=self._open).pack(side='left', padx=4, pady=10)
        tk.Button(tb, text='💾 Save HTML', bg=PAL['card'], fg=PAL['text'], font=('Segoe UI', 9), relief='flat', padx=12, command=self._save_html).pack(side='left', padx=4, pady=10)
        self._file_lbl = tk.Label(tb, text='No file open', fg=PAL['dim'], bg=PAL['panel'], font=('Segoe UI', 8))
        self._file_lbl.pack(side='right', padx=18)
        pane = tk.PanedWindow(self, orient='horizontal', bg=PAL['bg'], sashwidth=4, sashrelief='flat')
        pane.pack(fill='both', expand=True, padx=8, pady=8)
        raw_fr = tk.Frame(pane, bg=PAL['bg'])
        pane.add(raw_fr, minsize=300)
        tk.Label(raw_fr, text='SOURCE', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 8, 'bold'), pady=4).pack(anchor='w')
        self._editor = scrolledtext.ScrolledText(raw_fr, bg=PAL['card'], fg=PAL['text'], font=('Cascadia Code', 10), borderwidth=0, padx=12, pady=12, insertbackground='white')
        self._editor.pack(fill='both', expand=True)
        self._editor.bind('<KeyRelease>', lambda e: self._render(self._editor.get('1.0', 'end')))
        rnd_fr = tk.Frame(pane, bg=PAL['bg'])
        pane.add(rnd_fr, minsize=400)
        tk.Label(rnd_fr, text='PREVIEW', fg=PAL['dim'], bg=PAL['bg'], font=('Segoe UI', 8, 'bold'), pady=4).pack(anchor='w')
        self._view = scrolledtext.ScrolledText(rnd_fr, bg='#0A0C18', fg=PAL['text'], font=('Segoe UI', 11), borderwidth=0, padx=20, pady=20, state='disabled', wrap='word', insertbackground='white')
        self._view.pack(fill='both', expand=True)
        self._setup_tags()
