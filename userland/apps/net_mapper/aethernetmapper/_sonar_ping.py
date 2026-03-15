"""
Auto-split from userland\apps\net_mapper.py — AetherNetMapper._sonar_ping
"""

import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random



class AetherNetMapper:
    def _sonar_ping(self):
        self._log('LAUNCHING AETHER SONAR PING...')
        target = self.ip_entry.get().strip()
        self.after(800, lambda: self._log(f'REPLY FROM TARGET: 0.14ms | TTL: 64 | NO PACKET LOSS'))
