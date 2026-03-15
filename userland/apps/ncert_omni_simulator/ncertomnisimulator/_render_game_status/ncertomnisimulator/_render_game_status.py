# Generated method: NCERTOmniSimulator._render_game_status
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
    def _render_game_status(self):
        for w in self.status_tray.winfo_children():
            w.destroy()
        st = self.game.get_status()
        tk.Label(self.status_tray, text=f"RANK: {self.settings.get('user_name', 'Researcher')}", fg=PAL['accent'], bg=PAL['card'], font=('Segoe UI Bold', 10)).pack(pady=5)
        tk.Label(self.status_tray, text=f"Level {st['Level']} Scientific Pioneer", fg='white', bg=PAL['card'], font=('Segoe UI', 9)).pack()
        prog = tk.Frame(self.status_tray, bg='#1A1E30', height=4)
        prog.pack(fill='x', padx=20, pady=10)
        fill_w = int(st['Total XP'] % 500 / 500 * 160)
        tk.Frame(prog, bg=PAL['accent'], width=fill_w, height=4).pack(side='left')
        tk.Label(self.status_tray, text=f"XP: {st['Total XP']} | Labs: {st['Labs Done']}", fg=PAL['dim'], bg=PAL['card'], font=('Segoe UI', 8)).pack()