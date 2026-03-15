# Generated method: MeshTalk._setup_styles
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class MeshTalk:
    def _setup_styles(self):
        style = ttk.Style()
        style.theme_use('clam')
        style.configure('Mesh.TNotebook', background=PAL['bg'], borderwidth=0)
        style.configure('Mesh.TNotebook.Tab', background=PAL['sidebar'], foreground=PAL['text'], padding=[15, 8], font=('Inter', 9, 'bold'))
        style.map('Mesh.TNotebook.Tab', background=[('selected', PAL['accent'])])