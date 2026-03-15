"""
Auto-split from userland\apps\logic_simulator.py — half_adder
"""

import tkinter as tk
from tkinter import ttk



def half_adder(A, B):
    return (A ^ B, A & B)
