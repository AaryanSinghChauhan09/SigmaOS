"""
Auto-split from userland\apps\ncert_omni_simulator.py — NCERTOmniSimulator._clear_area
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
    def _clear_area(self):
        for w in self.main_area.winfo_children():
            w.destroy()
