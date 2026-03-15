# Generated method: NCERTOmniSimulator._launch_sublab
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
    def _launch_sublab(self, mod_name):
        self.game.record_experiment(mod_name)
        self._render_game_status()
        try:
            import subprocess
            path = os.path.join('userland', 'apps', f'{mod_name}.py')
            if os.path.exists(path):
                subprocess.Popen([sys.executable, path])
            else:
                SystemMonitor.log_incident('SublabLauncher', f'Module {mod_name} not found at {path}')
        except Exception as e:
            SystemMonitor.log_incident('SublabLauncher', str(e))
            messagebox.showerror('Error', f'Link failure: {e}')