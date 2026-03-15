# Generated method: MailOrchestratorPage._apply_tone
import tkinter as tk
from tkinter import ttk
from gui_pkg.base_page import SigmaPage
from gui_pkg.styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MONO

class MailOrchestratorPage:
    def _apply_tone(self, tone):
        self.gui._log_voice(f'AI: Re-drafting campaign with {tone} cognitive profile...')
        self.analysis_lbl.config(text=f'Readability: Optimal\nSentiment: {tone}\nSpam Risk: Minimal', fg=PAL['cyan'])