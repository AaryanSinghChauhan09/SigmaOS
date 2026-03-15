# Generated method: SovereignStrategist._handle_click
import tkinter as tk
from tkinter import messagebox, ttk
import random
import time
from typing import Dict, Any, List, Optional, Tuple, cast
import os
import sys

class SovereignStrategist:
    def _handle_click(self, r, c):
        if self._selected:
            sr, sc = self._selected
            if self.engine.execute_move(sr, sc, r, c):
                self._selected = None
                self._draw_board()
                self._run_analysis()
                self.after(600, self._ai_move)
            else:
                p = self.engine.board[r][c]
                if p and cast(str, p).startswith(self.engine.turn):
                    self._selected = (r, c)
                    self._draw_board()
        else:
            p = self.engine.board[r][c]
            if p and cast(str, p).startswith(self.engine.turn):
                self._selected = (r, c)
                self._draw_board()