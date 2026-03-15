# Generated file: full_adder
import tkinter as tk
from tkinter import ttk

def full_adder(A, B, Cin):
    S = A ^ B ^ Cin
    Cout = A & B | B & Cin | A & Cin
    return (S, Cout)