# Generated method: MeshLudo._roll
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class MeshLudo:
    def _roll(self):
        v = self.engine.roll_dice()
        faces = ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅']
        self.dice_lbl.config(text=faces[v - 1])
        self.engine.move_piece(self.engine.turn, 0)
        self._render_pieces()
        self.status.config(text=f"{self.engine.turn}'S STRATEGIC TURN", fg=PAL[self.engine.turn.lower()])