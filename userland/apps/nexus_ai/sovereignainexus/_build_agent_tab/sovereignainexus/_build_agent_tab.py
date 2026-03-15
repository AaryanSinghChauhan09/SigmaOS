# Generated method: SovereignAINexus._build_agent_tab
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json

class SovereignAINexus:
    def _build_agent_tab(self):
        tab = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(tab, text='  🤖 AI Agent  ')
        self.chat_display = scrolledtext.ScrolledText(tab, bg=PAL['card'], fg=PAL['text'], font=('Segoe UI', 10), borderwidth=0, padx=15, pady=15, insertbackground='white')
        self.chat_display.pack(fill='both', expand=True, pady=(10, 10))
        self.chat_display.tag_config('agent', foreground=PAL['accent'], font=('Segoe UI Bold', 10))
        self.chat_display.tag_config('user', foreground='white', font=('Segoe UI', 10))
        self.chat_display.tag_config('sys', foreground=PAL['dim'], font=('Segoe UI Italics', 9))
        input_fr = tk.Frame(tab, bg=PAL['bg'], pady=10)
        input_fr.pack(fill='x')
        self.chat_input = tk.Entry(input_fr, bg=PAL['card'], fg='white', insertbackground='white', font=('Segoe UI', 11), borderwidth=0, relief='flat', highlightthickness=1, highlightbackground=PAL['border'])
        self.chat_input.pack(side='left', fill='x', expand=True, ipady=8, padx=(0, 10))
        self.chat_input.bind('<Return>', lambda e: self._handle_chat())
        send_btn = tk.Button(input_fr, text='SEND', font=('Segoe UI', 9, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=20, command=self._handle_chat)
        send_btn.pack(side='right')