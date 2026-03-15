# Generated method: WelcomeAssistant._next
import tkinter as tk
from tkinter import ttk
import time
from typing import Any, List, Dict

class WelcomeAssistant:
    def _next(self):
        self.step += 1
        if self.step < len(self.content):
            c = self.content[self.step]
            self.icon_lbl.config(text=c['icon'], fg=c['color'])
            self.title_lbl.config(text=c['title'])
            self.desc_lbl.config(text=c['desc'])
            target = (self.step + 1) * (100 / len(self.content))
            self._animate_progress(target)
            if self.step == len(self.content) - 1:
                self.next_btn.config(text=f"ENTER SIGMAOS {ICONS.get('bootloader', '🚀')}")
        else:
            self._finalize()