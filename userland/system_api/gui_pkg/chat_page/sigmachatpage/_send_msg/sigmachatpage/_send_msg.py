# Generated method: SigmaChatPage._send_msg
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SigmaChatPage:
    def _send_msg(self):
        text = self.msg_entry.get()
        if not text or text == 'Type a secure message...':
            return
        if self.engine:
            res = self.engine.send_broadcast(text)
            self._insert_log('Me', text, is_me=True)
            self.msg_entry.delete(0, 'end')
            self._notify('Sovereign Chat', res, 'OK')