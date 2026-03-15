# Generated method: PromptOMatic.dispatch
import tkinter as tk
from tkinter import ttk, messagebox
import webbrowser
import urllib.parse
import time

class PromptOMatic:
    def dispatch(self):
        prompt = self.txt.get('1.0', 'end-1c').strip()
        if not prompt:
            return
        selected = [n for n, v in self.targets.items() if v.get()]
        if not selected:
            return
        encoded = urllib.parse.quote(prompt)
        urls = {'ChatGPT': 'https://chatgpt.com/?q={}', 'Claude': 'https://claude.ai/chat?q={}', 'Gemini': 'https://gemini.google.com/app?q={}', 'DeepSeek': 'https://chat.deepseek.com/?q={}', 'Llama-3': 'https://www.meta.ai/?q={}'}
        for model in selected:
            target = urls[model].format(encoded)
            webbrowser.open(target)
        messagebox.showinfo('Command Center', f'Prompt successfully distributed to {len(selected)} neural nodes.')