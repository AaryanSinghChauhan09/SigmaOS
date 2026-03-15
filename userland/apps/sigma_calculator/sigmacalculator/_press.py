"""
Auto-split from userland\apps\sigma_calculator.py — SigmaCalculator._press
"""

import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List



class SigmaCalculator:
    def _press(self, label):
        OP_MAP = {'×': '*', '÷': '/', '−': '-'}
        if label == 'C':
            self._expr = ''
            self._update('0')
            return
        if label == '=':
            self._evaluate()
            return
        if label == '±':
            if self._expr and (not self._expr.startswith('-')):
                self._expr = '-' + self._expr
            elif self._expr.startswith('-'):
                self._expr = self._expr[1:]
            self._update(self._expr or '0')
            return
        if label == 'MC':
            self._memory = 0.0
            self._mem_lbl.config(text='M: 0')
            return
        if label == 'MR':
            self._expr += str(self._memory)
            self._update(self._expr)
            return
        if label == 'MS':
            self._memory = self._safe_eval()
            self._mem_lbl.config(text=f'M: {self._memory}')
            return
        if label == 'M+':
            self._memory += self._safe_eval()
            self._mem_lbl.config(text=f'M: {self._memory}')
            return
        if label == 'M-':
            self._memory -= self._safe_eval()
            self._mem_lbl.config(text=f'M: {self._memory}')
            return
        fn_map = {'sin': 'math.sin(math.radians(', 'cos': 'math.cos(math.radians(', 'tan': 'math.tan(math.radians(', 'log': 'math.log10(', 'ln': 'math.log(', '√': 'math.sqrt(', 'x²': '(', 'π': str(math.pi), 'e': str(math.e)}
        if label in fn_map:
            if label == 'x²':
                self._expr = f'({self._expr})**2' if self._expr else '0**2'
            elif label in ('π', 'e'):
                self._expr += fn_map[label]
            else:
                self._expr += fn_map[label]
                self._update(self._expr)
                return
        else:
            self._expr += OP_MAP.get(label, label)
        self._update(self._expr)
