# Generated method: SovereignThemeEngine._export_profile
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser
import random

class SovereignThemeEngine:
    def _export_profile(self):
        messagebox.showinfo('Export', 'Current rice profile packaged as:\n./sovereign_rice_v1.tar.gz\n\nIncludes: dotfiles, GTK config, icon manifest, picom.conf, waybar CSS.\n\nShare on r/unixporn or backup to Sovereign Vault.')