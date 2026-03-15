# Generated method: GuideApex._build_intro_tab
import tkinter as tk
from tkinter import ttk, scrolledtext
import time

class GuideApex:
    def _build_intro_tab(self):
        tab = tk.Frame(self.nb, bg=PAL['bg'], padx=25, pady=25)
        self.nb.add(tab, text='  🚀 Getting Started  ')
        txt = scrolledtext.ScrolledText(tab, bg=PAL['card'], fg=PAL['text'], font=('Segoe UI', 11), borderwidth=0, padx=30, pady=30)
        txt.pack(fill='both', expand=True)
        msg = "WELCOME, SOVEREIGN USER.\n\nSigmaOS is not just an operating system; it's a statement of user supremacy.\nThis guide will walkthrough the core pillars of your new environment.\n\n1. THE DASHBOARD: Your central hub for apps, telemetry, and system health.\n2. AI NEXUS (🧬): Your agentic partner. Ask it to 'audit security' or 'open browser'.\n3. THE APPS: Every tool is isolated in a Zero-Trust sandbox.\n4. VFS: A virtualized file system designed for project-based automation.\n\nPRO TIP: Use the 'Security Guardian' (SENTINEL) to monitor real-time thread activity and mesh connectivity.\n"
        txt.insert('1.0', msg)
        txt.configure(state='disabled')