# Generated method: SovereignArcade._xo_move
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _xo_move(self, idx):
        if self.xo_board[idx] == '':
            self.xo_board[idx] = self.xo_turn
            self.xo_btns[idx].config(text=self.xo_turn, fg=PAL['accent'] if self.xo_turn == 'X' else '#32D74B')
            if self._check_xo():
                self.xo_status.config(text=f'WINNER: {self.xo_turn}', fg='#32D74B')
            else:
                self.xo_turn = 'O' if self.xo_turn == 'X' else 'X'
                self.xo_status.config(text=f"{self.xo_turn}'s STRATEGIC VECTOR")