# Generated method: VaultKeep._reveal_secret
import tkinter as tk
from tkinter import ttk, messagebox
import random

class VaultKeep:
    def _reveal_secret(self, event):
        if not self.auth_state:
            messagebox.showerror('Access Denied', 'Vault is securely sealed. Authenticate first.')
            return
        item = self.tree.selection()
        if item:
            val = self.tree.item(item, 'values')[0]
            messagebox.showinfo('Decrypted', f'[{val}]\n\nPassword copied to Sovereign Memory buffer.')