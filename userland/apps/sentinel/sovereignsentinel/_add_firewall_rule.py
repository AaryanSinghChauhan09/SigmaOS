"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._add_firewall_rule
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _add_firewall_rule(self):
        self.fw_tree.insert('', 'end', values=('IN', 'TCP', '8080', 'ALLOW', 'Custom Rule'))
        self.status.config(text='Firewall rule applied to kernel-level packet filter.', bg=PAL['safe'])
