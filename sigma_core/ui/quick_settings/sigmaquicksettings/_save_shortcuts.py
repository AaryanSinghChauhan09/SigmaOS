# Generated method: SigmaQuickSettings._save_shortcuts
import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    def _save_shortcuts(self):
        import json
        try:
            data = json.loads(self.shortcut_text.get('1.0', 'end'))
            self.kernel.cfg.SHORTCUTS = data
            self.kernel.cfg.save()
            print('[INFO] QuickSettings: Shortcuts saved to config.')
        except Exception as e:
            print(f'[ERROR] QuickSettings: Invalid JSON – {e}')