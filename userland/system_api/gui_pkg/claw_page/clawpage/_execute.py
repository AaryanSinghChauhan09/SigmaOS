# Generated method: ClawPage._execute
import tkinter as tk
from tkinter import ttk, scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_TITLE, FONT_MED, FONT_MONO
from sigma_core.ai.sovereign_claw import SovereignClaw

class ClawPage:
    def _execute(self):
        prompt = self.prompt_ent.get()
        if not prompt:
            return
        self.prompt_ent.delete(0, tk.END)
        self.log.insert(tk.END, f'\n> {prompt}\n', 'user')
        self.status_lbl.config(text='● EXECUTING INTENT', fg=PAL['gold'])

        def _task():
            res = self.claw.execute_prompt(prompt)
            self.after(500, lambda: self.log.insert(tk.END, f'Claw: {res}\n', 'sys'))
            self.after(550, lambda: self.status_lbl.config(text='● STANDBY', fg=PAL['green']))
            self._notify('Claw Agent', 'Intent execution sequence finalized.', 'OK')
        import threading
        threading.Thread(target=_task, daemon=True).start()