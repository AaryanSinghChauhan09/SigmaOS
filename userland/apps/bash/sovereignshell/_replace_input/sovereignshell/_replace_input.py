# Generated method: SovereignShell._replace_input
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

class SovereignShell:
    def _replace_input(self, text):
        self.terminal.delete('input_start', tk.END)
        self.terminal.insert('input_start', text)