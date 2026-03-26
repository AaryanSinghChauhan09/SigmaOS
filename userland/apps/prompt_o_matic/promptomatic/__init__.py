# Generated method: PromptOMatic.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import webbrowser
import urllib.parse
import time

class PromptOMatic:
    def __init__(self, master=None):
        super().__init__(master)
        self.title('Prompt-o-Matic Apex Pro')
        self.geometry('850x700')
        self.configure(bg=PAL['bg'])
        self.targets = {'ChatGPT': tk.BooleanVar(value=True), 'Claude': tk.BooleanVar(value=True), 'Gemini': tk.BooleanVar(value=True), 'DeepSeek': tk.BooleanVar(value=False), 'Llama-3': tk.BooleanVar(value=False)}
        self._build_ui()