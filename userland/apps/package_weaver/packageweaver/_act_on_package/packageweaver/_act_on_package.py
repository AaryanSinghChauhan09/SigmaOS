# Generated method: PackageWeaver._act_on_package
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class PackageWeaver:
    def _act_on_package(self):
        item = self.tree.selection()
        if not item:
            return
        val = list(self.tree.item(item, 'values'))
        name = val[0]
        stat = val[3]
        if 'Installed' in stat and 'Upgrade' not in stat:
            c = messagebox.askyesno('Remove', f'Obliterate {name} and all associated config blobs?')
            if c:
                val[3] = 'Not Installed'
                self.tree.item(item, values=val)
                self.status.config(text=f'PURGED {name} FROM SYSTEM', bg=PAL['danger'], fg='white')
        else:
            c = messagebox.askyesno('Install', f'Pull and compile {name} from decentralized mesh?')
            if c:
                self.status.config(text=f'COMPILING {name}... GENERATING SANDBOX...', bg=PAL['warning'], fg='black')
                self.after(1500, lambda: self._finish_install(item, val))