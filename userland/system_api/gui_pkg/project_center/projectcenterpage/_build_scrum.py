# Generated method: ProjectCenterPage._build_scrum
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class ProjectCenterPage:
    def _build_scrum(self, parent):
        tk.Label(parent, text='Sprint 14: Neural Mesh Integration', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(pady=10)
        pb = ttk.Progressbar(parent, value=75, length=400)
        pb.pack(pady=10)
        tk.Label(parent, text='Velocity: 42 pts/sprint | Efficiency: High', font=FONT_SMALL, fg=PAL['teal'], bg=PAL['bg']).pack()