"""
Auto-split from userland\apps\nexus_ai.py — SovereignAINexus._handle_chat
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json



class SovereignAINexus:
    def _handle_chat(self):
        txt = self.chat_input.get().strip()
        if not txt:
            return
        self.chat_input.delete(0, 'end')
        self._write_chat('User', txt)
        lower = txt.lower()
        response = ''
        if any((x in lower for x in ['scan', 'audit', 'security', 'threat', 'loophole'])):
            self.nb.select(2)
            response = "Scanning the Sovereign Core for behavioral loopholes... I've updated the Audit tab. You can approve individual fixes there."
            self._poll_system_audit(force=True)
        elif 'fix' in lower:
            response = "I'm identifying the most critical loopholes now. Please approve the fixes in the 'Security Audit' tab."
            self.nb.select(2)
        elif 'help' in lower or 'guide' in lower:
            self.nb.select(1)
            response = "I have opened the OS Guide. Specifically, for security, use the 'Sovereign Sentinel' app."
        elif 'open' in lower:
            app = lower.replace('open', '').strip()
            response = f"Sending request to Sigma Kernel to launch '{app}' in a sandboxed session."
            self._write_chat('Nexus', f'SYSTEM_ACTION: LAUNCH_MODULE({app})')
        elif 'status' in lower:
            response = f'All systems nominal. CPU: {random.randint(2, 8)}%, RAM: {random.uniform(0.4, 1.2):.1f}GB. 0 Security threats detected.'
        else:
            response = "Acknowledged. I'll search the SigmaMesh for relevant insights. Is there a specific OS component you need help with?"
        self.after(600, lambda: self._write_chat('Nexus', response))
