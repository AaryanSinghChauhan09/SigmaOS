# Generated method: DataAnalyzerPage._parse_csv
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import csv
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_MED

class DataAnalyzerPage:
    def _parse_csv(self, path):
        self.tree.delete(*self.tree.get_children())
        try:
            with open(path, mode='r', encoding='utf-8') as f:
                reader = csv.reader(f)
                header = next(reader, None)
                if not header:
                    return
                self.tree['columns'] = header
                for col in header:
                    self.tree.heading(col, text=col)
                    self.tree.column(col, width=120, minwidth=50)
                rows = 0
                for row_data in reader:
                    self.tree.insert('', 'end', values=row_data)
                    rows += 1
                    if rows > 1000:
                        break
                self.stat_rows.config(text=f'Rows: {rows} (Preview cap)')
                self.stat_cols.config(text=f'Cols: {len(header)}')
                self.controller._notify('Data Analyzer', f'Dataset parsed: {rows} rows, {len(header)} cols.', 'OK')
        except Exception as e:
            self.controller._notify('Data Error', str(e), 'ERR')