# Generated method: SigmaCalculator._evaluate
import tkinter as tk
from tkinter import ttk
import math, cmath, re, json, os
from typing import Any, List

class SigmaCalculator:
    def _evaluate(self):
        try:
            raw = self._expr.replace('^', '**')
            result = eval(raw, {'__builtins__': {}, 'math': math, 'cmath': cmath}, {})
            result = round(float(result.real if isinstance(result, complex) else result), 10)
            mode = self._mode.get()
            if mode == 'HEX':
                display = hex(int(result)).upper()
            elif mode == 'BIN':
                display = bin(int(result))
            elif mode == 'OCT':
                display = oct(int(result))
            else:
                display = str(result).rstrip('0').rstrip('.') if '.' in str(result) else str(result)
            entry = f'{self._expr} = {display}'
            self._history.append(entry)
            self._save_history()
            self._refresh_history()
            self._hist_lbl.config(text=self._expr)
            self._expr = str(result)
            self._update(display)
        except Exception as ex:
            self._update(f'ERR: {ex}')
            self._expr = ''