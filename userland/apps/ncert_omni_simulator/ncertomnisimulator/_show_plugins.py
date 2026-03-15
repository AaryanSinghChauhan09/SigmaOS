"""
Auto-split from userland\apps\ncert_omni_simulator.py — NCERTOmniSimulator._show_plugins
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
    def _show_plugins(self):
        self._clear_area()
        pane = tk.Frame(self.main_area, bg=PAL['card'], padx=40, pady=40)
        pane.pack(expand=True)
        tk.Label(pane, text='COMMUNITY PLUGIN VAULT', font=('Segoe UI Bold', 16), fg=PAL['dim'], bg=PAL['card']).pack(pady=20)
        plugins = PluginHub.list_plugins()
        if not plugins:
            tk.Label(pane, text='No Community Plugins Found.', fg=PAL['dim'], bg=PAL['card']).pack()
        else:
            for p in plugins:
                btn = tk.Button(pane, text=f'LOAD {p.upper()}', bg=PAL['bg'], fg='white', relief='flat', padx=20, pady=10)
                btn.pack(fill='x', pady=5)
