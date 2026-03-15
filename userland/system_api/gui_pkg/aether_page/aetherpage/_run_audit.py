# Generated method: AetherPage._run_audit
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_LOGO, FONT_MONO

class AetherPage:
    def _run_audit(self):
        res = self.kernel.verify_merkle_integrity('sigma_core')
        status = 'VERIFIED' if res else 'INTEGRITY_COMPROMISED'
        self.gui._notify('SECURITY', f'Merkle Audit: {status}', 'OK' if res else 'ERR')