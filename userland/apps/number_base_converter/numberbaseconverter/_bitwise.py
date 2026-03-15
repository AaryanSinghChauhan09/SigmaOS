# Generated method: NumberBaseConverter._bitwise
import tkinter as tk
from tkinter import ttk

class NumberBaseConverter:
    def _bitwise(self, op):
        try:
            a = int(self._vars['dec'][0][0].get())
            if op == 'NOT':
                bits = int(self._bits.get())
                r = ~a & (1 << bits) - 1
                self._result.config(text=f'NOT {a} = {r}  (BIN: {bin(r)})')
            else:
                b_str = tk.simpledialog.askstring('Operand B', 'Enter second number (decimal):') if hasattr(tk, 'simpledialog') else ''
                if not b_str:
                    self._result.config(text='Enter B in Decimal field')
                    return
                b = int(b_str)
                ops = {'AND': a & b, 'OR': a | b, 'XOR': a ^ b, '<<': a << b, '>>': a >> b}
                r = ops[op]
                self._result.config(text=f'{a} {op} {b} = {r}  (BIN: {bin(r)}, HEX: {hex(r).upper()})')
        except Exception as ex:
            self._result.config(text=f'Error: {ex}')