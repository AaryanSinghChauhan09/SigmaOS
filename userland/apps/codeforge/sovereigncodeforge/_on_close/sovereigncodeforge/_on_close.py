# Generated method: SovereignCodeForge._on_close
import tkinter as tk
from tkinter import ttk, filedialog, messagebox, scrolledtext
import os, re, sys, subprocess, threading
from typing import Any, Optional

class SovereignCodeForge:
    def _on_close(self):
        if self._unsaved:
            if messagebox.askyesno('Unsaved Changes', 'You have unsaved changes. Save before closing?'):
                self.save_file()
        if self._proc and self._proc.poll() is None:
            self._proc.terminate()
        self.destroy()