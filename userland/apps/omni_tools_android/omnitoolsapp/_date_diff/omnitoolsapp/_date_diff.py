# Generated method: OmniToolsApp._date_diff
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _date_diff(self) -> None:
        today = datetime.date.today()
        messagebox.showinfo('Date', f'Today: {today}\nDay of year: {today.timetuple().tm_yday}\nWeek: {today.isocalendar()[1]}')