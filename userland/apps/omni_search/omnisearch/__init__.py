# Generated method: OmniSearch.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import time

class OmniSearch:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Omni-Search')
        self.geometry('900x650')
        self.configure(bg=PAL['bg'])
        self.db = [('kernel.py', 'System Core', '85kb', 'C:/SigmaOS/sigma_core/'), ('Design_Specs.pdf', 'Encrypted Vault', '4.2mb', 'F:/Secured/'), ('Aura Display Config', 'System Setting', '--', 'Config Matrix'), ('Deploy Nodes', 'Macro Action', '0ms', 'Omni Automation'), ('vacation_photos.enc', 'Archived Drive', '12GB', 'Z:/Backup/')]
        self._setup_styles()
        self._build_ui()