# Generated method: SearchPage._reindex
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL, FONT_MED

class SearchPage:
    def _reindex(self):
        msg = self.kernel.aeryn_search.reindex_system()
        self.gui._notify('Re-indexing', msg, 'OK')
        self.stats_lbl.config(text=self.kernel.aeryn_search.health_check())