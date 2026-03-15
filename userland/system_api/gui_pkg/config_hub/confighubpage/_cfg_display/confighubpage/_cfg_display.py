# Generated method: ConfigHubPage._cfg_display
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL

class ConfigHubPage:
    def _cfg_display(self, parent):
        tk.Label(parent, text='Display & Hybrid Compositor', font=FONT_TITLE, fg='white', bg=PAL['bg']).pack(anchor='w', pady=10)
        tk.Label(parent, text='Resolution: 1400x900 (Native High-DPI)', bg=PAL['bg'], fg=PAL['text']).pack(anchor='w')
        ttk.Checkbutton(parent, text='Enable 10-bit Color Depth (Pro Rendering)').pack(anchor='w', pady=5)
        ttk.Checkbutton(parent, text='Hyper-Jitter Suppression (Direct Compositing)').pack(anchor='w', pady=5)