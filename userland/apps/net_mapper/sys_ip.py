"""
Auto-split from userland\apps\net_mapper.py — sys_ip
"""

import tkinter as tk
from tkinter import ttk, messagebox
import socket
import threading
import random



def sys_ip():
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(('8.8.8.8', 80))
        ip = s.getsockname()[0]
        s.close()
        return ip
    except:
        return '127.0.0.1'
