# Generated method: LinuxParityPage._log_linux
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_BOLD, FONT_SMALL

class LinuxParityPage:
    def _log_linux(self, msg):
        self._log(self._linux_log, str(msg), 'INFO')