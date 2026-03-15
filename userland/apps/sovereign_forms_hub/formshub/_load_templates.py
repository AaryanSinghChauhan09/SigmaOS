# Generated method: FormsHub._load_templates
import tkinter as tk
from tkinter import ttk, messagebox
import json
import os
from typing import Dict, Any, List, Optional, Union, Callable
from sigma_core.ui.fluid_design import PALETTE as PAL, TYPOGRAPHY as FONT

class FormsHub:
    def _load_templates(self):
        """Loads available forms from the Legal Form Engine."""
        if self.kernel and hasattr(self.kernel, 'legal_forms'):
            self.template_list = self.kernel.legal_forms.get_available_templates()
        else:
            self.template_list = [{'id': 'FIR', 'title': 'First Information Report', 'act': 'BNSS Sec 173'}, {'id': 'BAIL', 'title': 'Anticipatory Bail App.', 'act': 'BNSS Sec 482'}, {'id': 'BSA63', 'title': 'Digital Evidence Cert.', 'act': 'BSA Sec 63'}]