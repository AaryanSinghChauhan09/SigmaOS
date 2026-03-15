# Generated method: LudoEngine.roll_dice
import tkinter as tk
from tkinter import ttk, messagebox
import random
import os
import sys
from sigma_core.games.ludo_engine import LudoEngine

class LudoEngine:
    def roll_dice(self):
        return random.randint(1, 6)