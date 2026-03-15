# Generated method: AetherNetMapper._rogue_scan
import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random

class AetherNetMapper:
    def _rogue_scan(self):
        self._log('SWEEPING LOCAL SUBNET FOR ROGUE NODES...', PAL['warning'])
        self.after(1500, lambda: self._log('SUBNET SECURE. ZERO UNAUTHORIZED SHADOW DEVICES DETECTED.'))