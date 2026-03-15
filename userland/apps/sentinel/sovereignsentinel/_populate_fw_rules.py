"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._populate_fw_rules
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _populate_fw_rules(self):
        rules = [('IN', 'TCP', '443', 'ALLOW', 'HTTPS Traffic'), ('IN', 'TCP', '80', 'ALLOW', 'HTTP Traffic'), ('IN', 'ANY', '22', 'BLOCK', 'SSH Brute-Force'), ('OUT', 'UDP', '1194', 'ALLOW', 'Mesh VPN'), ('BOTH', 'TCP', '0', 'BLOCK', 'Zero-Day Blackhole')]
        for r in rules:
            self.fw_tree.insert('', 'end', values=r)
