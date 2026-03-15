# Generated method: TextCleaner._set_status
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import re
import string
import random

class TextCleaner:
    def _set_status(self, msg, color=PAL['accent']):
        self.status.config(text=msg.upper(), bg=color)