# Generated method: SystemProfiler._force_refresh
import tkinter as tk
from tkinter import ttk, messagebox
import platform
import random

class SystemProfiler:
    def _force_refresh(self):
        self._update_telemetry()
        self.status.config(text='TELEMETRY FEED REFRESHED.', bg=PAL['success'])