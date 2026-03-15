"""
Auto-split from userland\apps\ncert_omni_simulator.py — NCERTOmniSimulator._build_ui
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
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['panel'], height=80)
        head.pack(fill='x')
        head.pack_propagate(False)
        user = self.settings.get('user_name', 'Researcher')
        tk.Label(head, text=f'⚛ OMNI-LAB • Welcome, {user}', font=('Segoe UI', 24, 'bold'), fg=PAL['accent'], bg=PAL['panel']).pack(side='left', padx=30)
        search_ent = tk.Entry(head, textvariable=self.search_var, bg=PAL['bg'], fg='white', font=('Segoe UI', 11), relief='flat', width=40, insertbackground='white')
        search_ent.pack(side='right', padx=30, pady=25)
        search_ent.insert(0, 'Search NCERT Concepts (e.g. Optics, Titration)...')
        search_ent.bind('<FocusIn>', lambda e: search_ent.delete(0, tk.END))
        self.content_frame = tk.Frame(self, bg=PAL['bg'])
        self.content_frame.pack(fill='both', expand=True, padx=20, pady=20)
        side = tk.Frame(self.content_frame, bg=PAL['panel'], width=280)
        side.pack(side='left', fill='y', padx=(0, 20))
        side.pack_propagate(False)
        self.status_tray = tk.Frame(side, bg=PAL['card'], height=120)
        self.status_tray.pack(fill='x', padx=10, pady=10)
        self._render_game_status()
        tk.Label(side, text='CENTRAL COMMAND', font=('Segoe UI Bold', 10), fg=PAL['dim'], bg=PAL['panel']).pack(pady=(20, 10), anchor='w', padx=20)
        subjects = [('PHYSICS LAB', PAL['phys'], self._show_phys), ('CHEMISTRY SUITE', PAL['chem'], self._show_chem), ('BIOLOGY MAPS', PAL['bio'], self._show_bio), ('MATHEMATICA', PAL['math'], self._show_math), ('DIKSHA PORTAL', '#FFD700', self._show_diksha), ('PRIMARY HUB', PAL['accent'], self._show_primary), ('ANALYTICS HUB', '#10B981', self._show_analytics), ('COMMUNITY PLUGINS', PAL['dim'], self._show_plugins), ('OS SETTINGS', PAL['dim'], self._show_settings)]
        for name, color, cmd in subjects:
            btn = tk.Button(side, text=name, font=('Segoe UI Bold', 11), bg=PAL['card'], fg=color, relief='flat', anchor='w', padx=20, pady=15, cursor='hand2', command=cmd)
            btn.pack(fill='x', pady=5, padx=10)
        tk.Label(side, text='SYSTEM INTEG', font=('Segoe UI Bold', 10), fg=PAL['dim'], bg=PAL['panel']).pack(pady=(30, 5), anchor='w', padx=20)
        self.health_lbl = tk.Label(side, text='CPU: -- | RAM: --', font=('Consolas', 8), fg=PAL['chem'], bg=PAL['bg'], pady=10)
        self.health_lbl.pack(fill='x', padx=10)
        if self.settings.get('stealth_mode', False):
            self.status_tray.pack_forget()
            self.health_lbl.pack_forget()
        else:
            self.status_tray.pack(fill='x', padx=10, pady=10)
            self.health_lbl.pack(fill='x', padx=10)
        self._show_welcome()
