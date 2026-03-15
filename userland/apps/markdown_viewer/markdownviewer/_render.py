"""
Auto-split from userland\apps\markdown_viewer.py — MarkdownViewer._render
"""

import tkinter as tk
from tkinter import ttk, filedialog, scrolledtext
import re, os



class MarkdownViewer:
    def _render(self, md_text):
        v = self._view
        v.configure(state='normal')
        v.delete('1.0', 'end')
        in_code = False
        code_buf = []
        lines = md_text.split('\n')
        for line in lines:
            if line.strip().startswith('```'):
                if in_code:
                    v.insert('end', '\n'.join(code_buf) + '\n', 'code_block')
                    code_buf = []
                    in_code = False
                else:
                    in_code = True
                continue
            if in_code:
                code_buf.append(line)
                continue
            if line.startswith('### '):
                v.insert('end', line[4:] + '\n', 'h3')
                continue
            if line.startswith('## '):
                v.insert('end', line[3:] + '\n', 'h2')
                continue
            if line.startswith('# '):
                v.insert('end', line[2:] + '\n', 'h1')
                continue
            if re.match('^[-*_]{3,}$', line.strip()):
                v.insert('end', '─' * 58 + '\n', 'hr')
                continue
            if line.startswith('> '):
                v.insert('end', '│ ' + line[2:] + '\n', 'quote')
                continue
            if '|' in line and line.strip().startswith('|'):
                if re.match('^\\|[-| :]+\\|$', line.strip()):
                    continue
                cells = [c.strip() for c in line.strip().strip('|').split('|')]
                row_txt = '  '.join((f'{c:<18}' for c in cells))
                tag = 'table_head' if all((c.isupper() or len(c) < 5 for c in cells)) else 'table_row'
                v.insert('end', row_txt + '\n', tag)
                continue
            if re.match('^[\\-\\*\\+]\\s', line):
                self._insert_inline(v, '  •  ' + line[2:] + '\n', 'bullet')
                continue
            if re.match('^\\d+\\.\\s', line):
                self._insert_inline(v, '  ' + line + '\n', 'bullet')
                continue
            self._insert_inline(v, line + '\n')
        if code_buf:
            v.insert('end', '\n'.join(code_buf) + '\n', 'code_block')
        v.configure(state='disabled')
        try:
            cur = self._editor.get('1.0', 'end-1c')
            if cur != md_text.rstrip('\n'):
                self._editor.delete('1.0', 'end')
                self._editor.insert('1.0', md_text)
        except Exception:
            pass
