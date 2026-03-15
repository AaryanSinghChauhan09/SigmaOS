"""
Auto-split from userland\apps\event_matrix.py — EventMatrix._simulate_logs
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class EventMatrix:
    def _simulate_logs(self):
        log_types = [('INFO', 'kernel_v4', 'Sovereign Boot Manager initiated sequence.'), ('INFO', 'sigma_net', 'WLAN Interface handshake successful (Ring 1).'), ('WARN', 'omni_lens_api', 'High VRAM utilization detected on Node 3.'), ('ERROR', 'disk_scout', 'Bad sector healed in encrypted partition.'), ('CRIT', 'sec_daemon', 'Unauthorized memory read blocked by Aegis Shield.'), ('INFO', 'forge_macro', "Sequence 'Morning Sovereignty' executed in 0.04ms."), ('INFO', 'userland', 'Session authenticated via Biometric Neural Hash.')]
        for _ in range(15):
            l_type, src, msg = random.choice(log_types)
            ts = time.strftime('%Y-%m-%d %H:%M:%S', time.gmtime(time.time() - random.randint(10, 10000)))
            self.tree.insert('', 'end', values=(l_type, ts, src, msg))
