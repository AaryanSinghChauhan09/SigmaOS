# Generated method: ProjectFlow._set_status
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def _set_status(self, msg, color=PAL['accent']):
        self.status.config(text=msg.upper(), bg=color)