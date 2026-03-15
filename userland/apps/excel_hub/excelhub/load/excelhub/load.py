# Generated method: ExcelHub.load
import tkinter as tk
from tkinter import ttk, scrolledtext, filedialog, messagebox
import random
import time

class ExcelHub:
    def load(self):
        filedialog.askopenfilename()
        self._add_log('SYSTEM', 'DATASET HYDRATED INTO MEMORY.', PAL['accent'])