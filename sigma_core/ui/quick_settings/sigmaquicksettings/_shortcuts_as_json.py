# Generated method: SigmaQuickSettings._shortcuts_as_json
import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    def _shortcuts_as_json(self):
        import json
        return json.dumps(self.kernel.cfg.SHORTCUTS, indent=2)