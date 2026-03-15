# Generated method: PackageWeaver._populate_pkgs
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class PackageWeaver:
    def _populate_pkgs(self):
        self.tree.delete(*self.tree.get_children())
        for p in self.pkgs:
            self.tree.insert('', 'end', values=p)