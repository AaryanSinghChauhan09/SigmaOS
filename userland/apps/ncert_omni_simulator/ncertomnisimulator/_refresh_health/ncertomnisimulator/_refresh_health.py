# Generated method: NCERTOmniSimulator._refresh_health
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
    def _refresh_health(self):
        report = SystemMonitor.get_health_report()
        sa_report = self.analytics.capture_metrics()
        self.health_lbl.config(text=f"CPU: {sa_report['cpu_usage']}% | RAM: {sa_report['ram_usage']}% | Status: {sa_report['system_state']}")
        if sa_report['cpu_usage'] > 80:
            self.health_lbl.config(fg='#F87171')
        else:
            self.health_lbl.config(fg=PAL['chem'])
        return report