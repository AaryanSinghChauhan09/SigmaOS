# Generated method: LudoEngine.__init__
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class LudoEngine:
    def __init__(self):
        self.turn = 'RED'
        self.dice_val = 1
        self.piece_states = {'RED': [0, 0, 0, 0], 'GREEN': [0, 0, 0, 0], 'BLUE': [0, 0, 0, 0], 'YELLOW': [0, 0, 0, 0]}
        self.yard_coords = {'RED': [(60, 60)], 'GREEN': [(440, 60)], 'BLUE': [(60, 440)], 'YELLOW': [(440, 440)]}
        self.history = []