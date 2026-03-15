# Generated method: SigmaChatPage._on_msg_received
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SigmaChatPage:
    def _on_msg_received(self, msg):
        sender = msg.get('sid', 'UNKNOWN')
        text = msg.get('text', '')
        self._insert_log(sender, text)