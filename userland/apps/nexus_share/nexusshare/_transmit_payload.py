# Generated method: NexusShare._transmit_payload
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading

class NexusShare:
    def _transmit_payload(self, node):
        res = messagebox.askyesno('Transmit Payload', f'Initialize tunnel to [{node}]?')
        if res:
            self.status.config(text=f'TRANSMITTING FRAGMENTS TO {node} | ENCRYPTING...', bg=PAL['accent'], fg='white')
            self.after(2000, lambda: self.status.config(text='TRANSMISSION 100% SUCCESSFUL | P2P TUNNEL COLLAPSED', bg=PAL['success'], fg='black'))