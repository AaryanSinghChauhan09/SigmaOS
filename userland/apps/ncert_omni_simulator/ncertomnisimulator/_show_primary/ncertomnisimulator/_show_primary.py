# Generated method: NCERTOmniSimulator._show_primary
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
    def _show_primary(self):
        self._clear_area()
        pane = tk.Frame(self.main_area, bg=PAL['card'], padx=20, pady=20)
        pane.pack(expand=True)
        tk.Label(pane, text='PRIMARY KNOWLEDGE HUB', font=('Segoe UI Bold', 16), fg=PAL['accent'], bg=PAL['card']).pack(pady=20)
        tk.Button(pane, text='MATHEMATICS (1-5)', bg=PAL['math'], fg='black', relief='flat', command=lambda: self._launch_sublab('ncert_primary_maths'), width=30, pady=10).pack(pady=5)
        tk.Button(pane, text='SCIENCE (1-5)', bg=PAL['phys'], fg='black', relief='flat', command=lambda: self._launch_sublab('ncert_primary_science'), width=30, pady=10).pack(pady=5)