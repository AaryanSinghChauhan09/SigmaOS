# Generated method: DikshaPortal.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import json

class DikshaPortal:
    def __init__(self, parent=None):
        super().__init__(parent)
        self.title('SigmaOS • Diksha Academic Portal')
        self.geometry('1000x700')
        self.configure(bg='#0A0B10')
        self.styles = {'bg': '#0A0B10', 'accent': '#4F46E5', 'card': '#1E1B4B', 'text': '#F8FAFC'}
        self._build_interface()