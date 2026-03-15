# Generated method: ProjectCenterPage.build
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ProjectCenterPage:
    def build(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True, padx=20, pady=10)
        tab_bar = tk.Frame(body, bg=PAL['bg2'], height=40)
        tab_bar.pack(fill='x', pady=(0, 10))
        tab_bar.pack_propagate(False)
        self.view_container = tk.Frame(body, bg=PAL['bg'])
        self.view_container.pack(fill='both', expand=True)
        self.views = {}
        for name, icon in [('Kanban', '??'), ('Scrum', '??'), ('Gantt', '??'), ('Reports', '??')]:
            b = tk.Button(tab_bar, text=f'{icon} {name}', font=FONT_SMALL, fg=PAL['text'], bg=PAL['bg2'], relief='flat', padx=15, command=lambda n=name: self._show_view(n))
            b.pack(side='left', fill='y')
        self._show_view('Kanban')