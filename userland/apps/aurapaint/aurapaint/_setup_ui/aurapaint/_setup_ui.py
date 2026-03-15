# Generated method: AuraPaint._setup_ui
import tkinter as tk
from tkinter import colorchooser, messagebox, ttk, simpledialog, filedialog
import random
import os
from typing import Any

class AuraPaint:
    def _setup_ui(self):
        self.top_bar = tk.Frame(self, bg=PAL['toolbar'], height=60, padx=20)
        self.top_bar.pack(side='top', fill='x')
        tk.Label(self.top_bar, text=f"{ICONS.get('paint', '🎨')} AURAPAINT PRO", font=('Inter', 18, 'bold'), fg=PAL['accent'], bg=PAL['toolbar']).pack(side='left')
        btn_fr = tk.Frame(self.top_bar, bg=PAL['toolbar'])
        btn_fr.pack(side='right')
        tools = [(f"{ICONS.get('fs', '📁')} NEW", self.clear), (f"{ICONS.get('snapshots', '💾')} EXPORT", self.save), (f"{ICONS.get('intelligence', '✨')} AI-GEN", self._ai_gen)]
        for txt, cmd in tools:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg='#252529', fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=10, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.side_fr = tk.Frame(self.workspace, bg=PAL['toolbar'], width=80, padx=10, pady=20)
        self.side_fr.pack(side='left', fill='y', padx=(0, 10))
        self.side_fr.pack_propagate(False)
        tools_list = [('✍️', 'pen'), ('🖌️', 'brush'), (ICONS.get('governor', '📐'), 'line'), ('⬛', 'rect'), ('⚪', 'circle'), ('🧽', 'eraser')]
        for icon, name in tools_list:
            tk.Button(self.side_fr, text=icon, font=('Segoe UI Emoji', 20), bg=PAL['toolbar'], fg='white', relief='flat', command=lambda n=name: self.set_tool(n)).pack(pady=10)
        self.canvas_fr = tk.Frame(self.workspace, bg='#000', highlightthickness=1, highlightbackground=PAL['border'])
        self.canvas_fr.pack(side='left', fill='both', expand=True)
        self.canvas = tk.Canvas(self.canvas_fr, bg=PAL['canvas'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True)
        self.canvas.bind('<B1-Motion>', self.draw)
        self.canvas.bind('<Button-1>', self.start_draw)
        self.prop_fr = tk.Frame(self.workspace, bg=PAL['toolbar'], width=220, padx=20, pady=20)
        self.prop_fr.pack(side='right', fill='y', padx=(10, 0))
        self.prop_fr.pack_propagate(False)
        tk.Label(self.prop_fr, text=f"{ICONS.get('hal', '⚙️')} PROPERTIES", font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['toolbar']).pack(anchor='w')
        self.color_box = tk.Button(self.prop_fr, bg=self.curr_color, width=15, height=2, relief='flat', command=self.pick_color)
        self.color_box.pack(pady=20)
        tk.Label(self.prop_fr, text='BRUSH SIZE', font=('Inter', 8), fg=PAL['dim'], bg=PAL['toolbar']).pack(anchor='w')
        self.size_scale = ttk.Scale(self.prop_fr, from_=1, to=100, orient='horizontal', command=self.set_size)
        self.size_scale.set(self.brush_size)
        self.size_scale.pack(fill='x', pady=10)
        tk.Label(self.prop_fr, text=f"{ICONS.get('fabric', '🕸️')} LAYERS", font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['toolbar'], pady=20).pack(anchor='w')
        layers = ['Background', 'Neural_Overlay', 'Vector_Mask']
        for l in layers:
            f = tk.Frame(self.prop_fr, bg='#252529', pady=5, padx=10)
            f.pack(fill='x', pady=2)
            tk.Label(f, text=f'👁️ {l}', font=('Inter', 9), fg='white', bg='#252529').pack(side='left')
        self.status = tk.Label(self, text='', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')