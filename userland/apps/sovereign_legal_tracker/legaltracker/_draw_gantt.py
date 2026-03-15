# Generated method: LegalTracker._draw_gantt
import tkinter as tk
from tkinter import ttk, messagebox
import sys
import os
from typing import Dict, Any, List, Optional
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT
from sigma_core.legal.legal_engine import LegalEngine

class LegalTracker:
    def _draw_gantt(self):
        y = 50
        for i, stage in enumerate(self.stages):
            color = PAL['success'] if stage['status'] == 'COMPLETED' else PAL['warning'] if stage['status'] == 'ONGOING' else PAL['text_tertiary']
            tag = f'stage_{i}'
            self.canvas.create_text(50, y, text=stage['name'], anchor='w', font=FONT['body_bold'], fill=PAL['text_primary'], tags=tag)
            self.canvas.create_text(50, y + 20, text=stage['act'], anchor='w', font=FONT['caption'], fill=PAL['text_secondary'], tags=tag)
            self.canvas.create_rectangle(300, y - 10, 1000, y + 10, fill=PAL['surface_variant'], outline='', tags=tag)
            progress_width = 700 if stage['status'] == 'COMPLETED' else 350 if stage['status'] == 'ONGOING' else 0
            self.canvas.create_rectangle(300, y - 10, 300 + progress_width, y + 10, fill=color, outline='', tags=tag)
            self.canvas.create_text(1050, y, text=f"[{stage['status']}] (CLICK FOR STATUTORY NOTE)", anchor='w', font=FONT['caption'], fill=color, tags=tag)

            def show_note(event, s=stage):
                self._show_statutory_note(s)

            def highlight(event, t=tag):
                self.canvas.itemconfig(self.canvas.find_withtag(t)[-1], font=('Arial', 8, 'underline', 'bold'))

            def unhighlight(event, t=tag):
                self.canvas.itemconfig(self.canvas.find_withtag(t)[-1], font=FONT['caption'])
            self.canvas.tag_bind(tag, '<Button-1>', show_note)
            self.canvas.tag_bind(tag, '<Enter>', highlight)
            self.canvas.tag_bind(tag, '<Leave>', unhighlight)
            y += 80