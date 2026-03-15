# Generated method: AegisPermissions.__init__
import tkinter as tk
from tkinter import ttk, messagebox

class AegisPermissions:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Aegis Shield')
        self.geometry('900x700')
        self.configure(bg=PAL['bg'])
        self.apps = [('pdf_forge.py', ['Kernel Read', 'File Write'], ['Net Access', 'Camera']), ('omni_search.py', ['Disk Indexing', 'Memory Read'], ['Net Access']), ('energy_core.py', ['Sensors', 'Hardware Power'], ['File Write', 'Clipboard']), ('Pulse_Browser', ['Net Access', 'Microphone'], ['Kernel Read', 'GPS Lock'])]
        self._setup_styles()
        self._build_ui()