# Generated method: ContextEngine._activate_mode
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random

class ContextEngine:
    def _activate_mode(self, mode_name, color):
        if self.active_mode == mode_name:
            self._log(f'>>> {mode_name} ALREADY ACTIVE.', PAL['warning'])
            return
        self._log(f'>>> SHIFTING PARADIGM TO: {mode_name}')
        self.active_mode = mode_name
        self.status.config(text=f'CURRENT PARADIGM: {self.active_mode}', bg=color, fg='black' if color in [PAL['warning'], PAL['success'], '#FFD60A', PAL['accent']] else 'white')
        if 'GAME' in mode_name:
            self._log('    [+] GPU VRAM Unlocked. Allocating 95% to foreground.')
            self._log('    [+] Background Indexing: SUSPENDED.')
            self._log('    [+] TCP/IP Stack: Optimized for Latency.')
        elif 'DeX' in mode_name:
            self._log('    [+] External Display Detected (HDMI/DP).')
            self._log('    [+] Shifting UI to Multi-Window Sovereign Desktop Environment.')
        elif 'BEDTIME' in mode_name:
            self._log('    [+] Aura Matrix: Blue Light Attenuated (0%).')
            self._log('    [+] Notifications: Suppressed (Zenith Focus).')
        elif 'DRIVING' in mode_name:
            self._log('    [+] GPS Velocity > 25mph Detected.')
            self._log('    [+] Voice Telemetry Enabled. Input Mode: Audio.')
        elif 'SURVIVAL' in mode_name:
            self._log('    [+] WARNING: Shutting down Kernel GUI.')
            self._log('    [+] CPU Underclocked by 75%. Est Battery: 400 Hours.')
        elif 'KIOSK' in mode_name:
            self._log('    [+] Process isolated. System interrupts disabled.')
        messagebox.showinfo('Mode Shift', f'Context Engine shifted state to:\n[{mode_name}]\n\nAll automated sub-routines engaged.')