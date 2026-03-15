# Generated method: OmniTweakDaemon._build_dotfiles_tab
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class OmniTweakDaemon:
    def _build_dotfiles_tab(self):
        tk.Label(self.tab_dot, text='DOTFILES MATRIX (.config OVERRIDES)', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        tk.Label(self.tab_dot, text='Symlink management for instant deployment of personalized terminal/editor profiles via Git.', font=('Inter', 9), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 20))
        dot_fr = tk.Frame(self.tab_dot, bg=PAL['bg'])
        dot_fr.pack(fill='both', expand=True)
        self.term = tk.Text(dot_fr, bg=PAL['panel'], fg=PAL['text'], font=('Consolas', 10), relief='flat')
        self.term.pack(fill='both', expand=True, pady=10)
        self.term.insert(tk.END, '>>> [OMNI-STOW MATRIX INITIALIZED]\n')
        self.term.insert(tk.END, 'Target: ~/.config/\n\n')
        configs = ['.zshrc', '.vimrc', 'kitty.conf', 'tmux.conf', 'sway_config']
        for c in configs:
            self.term.insert(tk.END, f'🔗 Symlinked {c} -> /sovereign_mnt/git/dotfiles/{c}\n')
        self.term.insert(tk.END, '\n[ALL DOTFILES SYNCED TO LOCAL GIT REPO]')
        self.term.config(state=tk.DISABLED)
        tk.Button(self.tab_dot, text='🚀 PUSH DOTFILES TO GITHUB', font=('Inter', 9, 'bold'), bg=PAL['accent_dim'], fg='white', relief='flat', pady=10, command=self._push_dots).pack(fill='x')