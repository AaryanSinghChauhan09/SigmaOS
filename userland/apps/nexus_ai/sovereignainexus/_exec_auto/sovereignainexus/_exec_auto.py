# Generated method: SovereignAINexus._exec_auto
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json

class SovereignAINexus:
    def _exec_auto(self):
        txt = self.chat_input.get().strip()
        if not txt:
            messagebox.showwarning('Auto-Task', 'Enter an OS intent in the chat input first.')
            return
        plan = self.task_agent.plan_task(txt)
        self._write_chat('Nexus', f"AUTONOMOUS PLAN GENERATED:\n{plan['title']}\n\n" + '\n'.join([f'• {s}' for s in plan['steps']]))

        def _exec():
            self.after(0, lambda: self._write_chat('Nexus', 'EXECUTING SOVEREIGN PLAN...'))
            for step in plan['steps']:
                msg = f'  [DONE] {step}'
                self.after(200, lambda m=msg: self._write_chat('Nexus', m))
                time.sleep(0.8)
            self.after(100, lambda: self._write_chat('Nexus', 'TASK COMPLETE. SYSTEM OPTIMIZED.'))
            self.after(200, lambda: self._notify_done(plan['title']))
        threading.Thread(target=_exec, daemon=True).start()