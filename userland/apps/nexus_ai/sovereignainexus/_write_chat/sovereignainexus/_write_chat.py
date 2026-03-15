# Generated method: SovereignAINexus._write_chat
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json

class SovereignAINexus:
    def _write_chat(self, author, msg):
        self.chat_display.configure(state='normal')
        ts = time.strftime('[%H:%M]')
        if author == 'Nexus':
            self.chat_display.insert('end', f'{ts} ', 'sys')
            self.chat_display.insert('end', 'NEXUS: ', 'agent')
            self.chat_display.insert('end', f'{msg}\n\n', 'user')
        else:
            self.chat_display.insert('end', f'{ts} ', 'sys')
            self.chat_display.insert('end', 'YOU: ', 'user')
            self.chat_display.insert('end', f'{msg}\n\n', 'user')
        self.chat_display.see('end')
        self.chat_display.configure(state='disabled')