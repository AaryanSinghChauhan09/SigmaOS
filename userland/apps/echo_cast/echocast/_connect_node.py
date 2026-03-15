# Generated method: EchoCast._connect_node
import tkinter as tk
from tkinter import ttk, messagebox
import threading
import time
import random

class EchoCast:
    def _connect_node(self, node):
        res = messagebox.askyesno('Secure Handshake', f'Initiate Quantum Display Tunnel to [{node}]?')
        if res:
            self.status.config(text=f'SCREEN VECTOR ROUTED TO {node} | <1ms LATENCY', bg=PAL['accent'], fg='black')