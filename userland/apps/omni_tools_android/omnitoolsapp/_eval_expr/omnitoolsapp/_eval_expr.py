# Generated method: OmniToolsApp._eval_expr
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _eval_expr(self) -> None:
        expr = self.expr_entry.get()
        allowed = {k: getattr(math, k) for k in dir(math) if not k.startswith('__')}
        try:
            result = eval(expr, {'__builtins__': {}}, allowed)
            self.expr_result.config(text=f'Result: {fmt(result)}', fg=PAL['success'])
        except Exception:
            self.expr_result.config(text='Error: invalid expression', fg=PAL['danger'])