# Generated method: VaultKeep.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random

class VaultKeep:
    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title('Sovereign Vault Keep')
        self.geometry('950x650')
        self.configure(bg=PAL['bg'])
        self.secrets = [('GitHub Access', 'sovereign_repo_admin', '********', '2 days ago'), ('AWS Root', 'admin1', '********', '1 hour ago'), ('Ethereum Wallet', 'N/A (Seed Phrase)', '********', '5 mins ago'), ('Sigma Kernel Key', 'admin', '********', 'System Boot'), ('Banking API', 'client_usr_99', '********', '3 wks ago')]
        self.auth_state = False
        self._setup_styles()
        self._build_ui()