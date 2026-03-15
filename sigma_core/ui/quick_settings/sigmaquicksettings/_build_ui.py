# Generated method: SigmaQuickSettings._build_ui
import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    def _build_ui(self):
        ttk.Label(self.frame, text='Quick Settings', font=('Helvetica', 16, 'bold')).pack(pady=10)
        ttk.Label(self.frame, text='Performance Mode:').pack(anchor='w')
        self.mode_var = tk.StringVar(value=self.kernel.cfg.ACTIVE_MODE)
        mode_combo = ttk.Combobox(self.frame, textvariable=self.mode_var, state='readonly')
        mode_combo['values'] = list(self.kernel.cfg.MODES.keys())
        mode_combo.pack(fill='x', pady=5)
        mode_combo.bind('<<ComboboxSelected>>', self._apply_mode)
        self.dev_var = tk.BooleanVar(value=self.kernel.cfg.DEVELOPER_MODE)
        dev_chk = ttk.Checkbutton(self.frame, text='Developer Mode (extra logs & debug console)', variable=self.dev_var, command=self._toggle_dev)
        dev_chk.pack(anchor='w', pady=5)
        ttk.Label(self.frame, text='Global Shortcuts (editable JSON):').pack(anchor='w', pady=(15, 5))
        self.shortcut_text = tk.Text(self.frame, height=8)
        self.shortcut_text.pack(fill='both', expand=True)
        self.shortcut_text.insert('1.0', self._shortcuts_as_json())
        ttk.Button(self.frame, text='Save Shortcuts', command=self._save_shortcuts).pack(pady=5)