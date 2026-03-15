# Generated method: OmniToolsApp._compass
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _compass(self) -> None:
        dirs = ['N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW']
        messagebox.showinfo('Compass', f'Simulated heading: {random.choice(dirs)}')