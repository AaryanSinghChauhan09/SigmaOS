# Generated method: TextCleaner._copy
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import re
import string
import random

class TextCleaner:
    def _copy(self):
        self.clipboard_clear()
        self.clipboard_append(self.out_txt.get('1.0', 'end-1c'))
        messagebox.showinfo('Clipboard', 'Vector buffer captured and encrypted.')