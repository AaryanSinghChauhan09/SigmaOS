# Generated method: SovereignShell._boot_sequence
import tkinter as tk
from tkinter import scrolledtext, messagebox, ttk
import subprocess
import os
import sys
import random

class SovereignShell:
    def _boot_sequence(self):
        self._write('Sovereign OS [Version 4.0.Apex]\n', PAL['accent'])
        self._write('(c) 2026 Sigma Sovereign. All rights reserved.\n\n', PAL['dim'] if 'dim' in PAL else PAL['text'])
        self._write('Establishing Neural Ingress... [OK]\n')
        self._write('Verified Bit-Sovereign Environment.\n\n')
        self._insert_prompt()