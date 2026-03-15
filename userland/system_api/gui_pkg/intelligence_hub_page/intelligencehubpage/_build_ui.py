"""
Auto-split from userland\system_api\gui_pkg\intelligence_hub_page.py — IntelligenceHubPage._build_ui
"""

import tkinter as tk
from tkinter import ttk
import random
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_MED, FONT_SMALL



class IntelligenceHubPage:
    def _build_ui(self):
        tabs = ttk.Notebook(self)
        tabs.pack(fill='both', expand=True, padx=20, pady=10)
        guardian = self.kernel.guardian
        ml_frame = self._create_tab(tabs, guardian.sanitize_text('🧠 AI & ML'))
        self._build_ml_section(ml_frame)
        math_frame = self._create_tab(tabs, guardian.sanitize_text('📐 Mathematics'))
        self._build_math_section(math_frame)
        stats_frame = self._create_tab(tabs, guardian.sanitize_text('📊 Statistics'))
        self._build_stats_section(stats_frame)
        gfx_frame = self._create_tab(tabs, guardian.sanitize_text('🎨 Graphics'))
        self._build_graphics_section(gfx_frame)
        hist_frame = self._create_tab(tabs, guardian.sanitize_text('📜 History'))
        self._build_history_section(hist_frame)
