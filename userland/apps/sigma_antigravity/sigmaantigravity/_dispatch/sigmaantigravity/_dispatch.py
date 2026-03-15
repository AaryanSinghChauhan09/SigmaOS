# Generated method: SigmaAntigravity._dispatch
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import threading, webbrowser, urllib.parse, json, os, time, sys
from typing import Dict, Any, List, Optional

class SigmaAntigravity:
    def _dispatch(self):
        prompt = self.prompt_txt.get('1.0', 'end').strip()
        if not prompt:
            return
        selected = [name for name, var in self._sel_platforms.items() if var.get()]
        if not selected:
            return
        res = self.engine.dispatch_prompt(prompt, selected)
        self.log.insert('end', f"[{res['time']}] DISPATCHED: {len(selected)} nodes active.\n")
        self._update_history()