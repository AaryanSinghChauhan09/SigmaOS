"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._refresh_audit
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _refresh_audit(self):
        self.audit_tree.delete(*self.audit_tree.get_children())
        events = [('2026-03-05 19:01:00', 'Kernel boot verified', 'sigma_kernel', 'INFO', 'Logged'), ('2026-03-05 19:01:05', 'VFS mounted OK', 'sigma_fs', 'INFO', 'Logged'), ('2026-03-05 19:02:12', 'UAL sandbox created', 'sigma_browser', 'INFO', 'Sandboxed'), ('2026-03-05 19:10:44', 'Port scan detected', 'external_ip', 'WARNING', 'Blocked'), ('2026-03-05 19:12:00', 'Mesh key rotation', 'sovereign_mesh', 'INFO', 'Executed'), ('2026-03-05 19:15:30', 'IDS: 0 anomalies', 'ids_engine', 'INFO', 'Verified')]
        for ev in events:
            tag = 'warn' if ev[3] == 'WARNING' else 'info'
            self.audit_tree.insert('', 'end', values=ev, tags=(tag,))
        self.audit_tree.tag_configure('warn', foreground=PAL['accent'])
        self.audit_tree.tag_configure('info', foreground=PAL['text'])
