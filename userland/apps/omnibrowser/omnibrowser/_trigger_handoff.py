# Generated method: OmniBrowser._trigger_handoff
import tkinter as tk
from tkinter import ttk, messagebox, simpledialog
import random
import time
from typing import Any, List
from sigma_core.ui.fluid_design import ICONS

class OmniBrowser:
    def _trigger_handoff(self):
        token = f'BEAM-{random.randint(1000, 9999)}'
        messagebox.showinfo('Mobile Handoff', f'Encrypted Workspace Beam Active.\n\nToken: {token}\nStatus: WAITING FOR SIGMA_MOBILE SYNC...')