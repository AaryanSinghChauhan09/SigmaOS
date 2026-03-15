"""
Auto-split from userland\apps\ncert_omni_simulator.py — NCERTOmniSimulator._show_welcome
"""

import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import math, json, os, sys
from sigma_core.app_discovery import AppDiscovery
from userland.system_api.settings_manager import SettingsManager
from sigma_core.gamification_engine import GamificationEngine
from sigma_core.system_monitor import SystemMonitor
from sigma_core.data_visualizer import DataVisualizer
from sigma_core.plugin_hub import PluginHub
from sigma_core.privacy_sentinel import PrivacySentinel
from userland.system_api.sigma_analytics import SovereignAnalytics
from userland.apps.diksha_portal import DikshaPortal



class NCERTOmniSimulator:
    def _show_welcome(self):
        self._clear_area()
        welcome = tk.Frame(self.main_area, bg=PAL['bg'])
        welcome.pack(expand=True)
        tk.Label(welcome, text='CHOOSE A RESEARCH DOMAIN', font=('Segoe UI', 20, 'bold'), fg=PAL['text'], bg=PAL['bg']).pack()
        tk.Label(welcome, text='Unified Simulator accessing 500+ NCERT data points', font=('Segoe UI', 11), fg=PAL['dim'], bg=PAL['bg']).pack(pady=10)
        grid = tk.Frame(welcome, bg=PAL['bg'])
        grid.pack(pady=40)
        discovered = AppDiscovery.find_apps()
        for name_obj, mod_obj in discovered.items():
            name = str(name_obj)
            mod = str(mod_obj)
            if name in ['Periodic Table', 'Logic Lab', 'Optics Bench', 'Titration', 'Physio Master']:
                color = PAL['chem'] if 'Table' in name else PAL['phys']
                desc = 'Specialized Research Simulation'
                c = tk.Frame(grid, bg=PAL['card'], width=200, height=200, padx=20, pady=20)
                c.pack(side='left', padx=15)
                c.pack_propagate(False)
                tk.Label(c, text=str(name), font=('Segoe UI Bold', 12), fg=color, bg=PAL['card']).pack(pady=10)
                tk.Label(c, text=desc, font=('Segoe UI', 8), fg=PAL['dim'], bg=PAL['card'], wraplength=160).pack()
                tk.Button(c, text='LAUNCH', font=('Segoe UI Bold', 8), bg=color, fg='black', relief='flat', command=self._mk_cmd(str(mod))).pack(side='bottom', pady=5)
