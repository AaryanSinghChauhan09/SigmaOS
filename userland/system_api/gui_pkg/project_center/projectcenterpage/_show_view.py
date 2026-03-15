# Generated method: ProjectCenterPage._show_view
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ProjectCenterPage:
    def _show_view(self, name):
        for v in self.views.values():
            v.pack_forget()
        if name not in self.views:
            v = tk.Frame(self.view_container, bg=PAL['bg'])
            self.views[name] = v
            if name == 'Kanban':
                self._build_kanban(v)
            elif name == 'Scrum':
                self._build_scrum(v)
            else:
                tk.Label(v, text=f'{name} View: Optimizing for Apex v2.2...', fg=PAL['dim'], bg=PAL['bg']).pack(expand=True)
        self.views[name].pack(fill='both', expand=True)