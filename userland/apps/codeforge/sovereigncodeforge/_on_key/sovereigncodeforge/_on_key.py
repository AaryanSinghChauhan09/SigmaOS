# Generated method: SovereignCodeForge._on_key
import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional

class SovereignCodeForge:
    def _on_key(self, event=None):
        self._update_nums()
        self._highlight()
        self._unsaved = True
        if event and event.char and event.char.isalnum():
            self._trigger_ac()
        else:
            self.ac_popup.place_forget()
        try:
            r, c = self.txt.index('insert').split('.')
            self.status.config(text=f"Ln {r}, Col {int(c) + 1}  |  Python  |  UTF-8  |  {('*Unsaved' if self._unsaved else 'Saved')}")
        except Exception:
            pass