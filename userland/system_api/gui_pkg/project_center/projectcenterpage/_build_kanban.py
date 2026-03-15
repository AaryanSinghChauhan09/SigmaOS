# Generated method: ProjectCenterPage._build_kanban
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ProjectCenterPage:
    def _build_kanban(self, parent):
        cols = ['Backlog', 'In Progress', 'Review', 'Done']
        for c in cols:
            fr = tk.Frame(parent, bg=PAL['bg2'], width=200)
            fr.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(fr, text=c, font=FONT_BOLD, fg=PAL['gold'], bg=PAL['bg2']).pack(pady=10)
            for i in range(2):
                card = self.gui._card(fr, f'Task {c[0]}{i + 1}')
                card.master.pack(fill='x', pady=5)
                tk.Label(card, text='Assigned to Agent_01', font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack(anchor='w')