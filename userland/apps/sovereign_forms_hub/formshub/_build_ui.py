# Generated method: FormsHub._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import json
import os
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class FormsHub:
    def _build_ui(self):
        self.sidebar = tk.Frame(self, bg=PAL['surface'], width=350, padx=25, pady=25)
        self.sidebar.pack(side='left', fill='y')
        self.sidebar.pack_propagate(False)
        tk.Label(self.sidebar, text='STATUTORY ACTS', font=FONT['caption'], fg=PAL['text_secondary'], bg=PAL['surface']).pack(anchor='w', pady=(0, 20))
        for template in self.template_list:
            t_id = str(template['id'])

            def make_cmd(fid: str) -> Callable[[], Any]:
                return lambda: self._load_form(fid)
            btn = tk.Button(self.sidebar, text=template['title'], font=FONT['body'], fg=PAL['text_primary'], bg=PAL['surface_variant'], relief='flat', anchor='w', padx=15, pady=10, command=make_cmd(t_id))
            btn.pack(fill='x', pady=5)
            tk.Label(self.sidebar, text=template['act'], font=FONT['caption'], fg=PAL['primary'], bg=PAL['surface']).pack(anchor='w', padx=5)
        self.main_area = tk.Frame(self, bg=PAL['background'], padx=50, pady=40)
        self.main_area.pack(side='left', fill='both', expand=True)
        self.form_header = tk.Frame(self.main_area, bg=PAL['background'])
        self.form_header.pack(fill='x', pady=(0, 30))
        self.title_lbl = tk.Label(self.form_header, text='Select a Statutory Form', font=FONT['h2'], fg=PAL['text_primary'], bg=PAL['background'])
        self.title_lbl.pack(side='left')
        self.form_container = tk.Frame(self.main_area, bg=PAL['background'])
        self.form_container.pack(fill='both', expand=True)
        self.canvas_view = tk.Canvas(self.form_container, bg=PAL['background'], highlightthickness=0)
        self.scrollbar = ttk.Scrollbar(self.form_container, orient='vertical', command=self.canvas_view.yview)
        self.scroll_frame = tk.Frame(self.canvas_view, bg=PAL['background'])
        self.canvas_view.create_window((0, 0), window=self.scroll_frame, anchor='nw')
        self.canvas_view.configure(yscrollcommand=self.scrollbar.set)
        self.canvas_view.pack(side='left', fill='both', expand=True)
        self.scrollbar.pack(side='right', fill='y')
        self.scroll_frame.bind('<Configure>', lambda e: self.canvas_view.configure(scrollregion=self.canvas_view.bbox('all')))
        self.footer = tk.Frame(self.main_area, bg=PAL['background'], pady=20)
        self.footer.pack(side='bottom', fill='x')
        tk.Button(self.footer, text='💾 SAVE DRAFT', bg=PAL['primary'], fg=PAL['background'], font=FONT['body_bold'], relief='flat', padx=30, pady=12, command=self._save_draft).pack(side='right', padx=10)