# Generated method: PeriodicTable._show_details
import tkinter as tk
from tkinter import messagebox

class PeriodicTable:
    def _show_details(self, zid):
        elem = next((e for e in ELEMENTS if e[0] == zid), None)
        if elem:
            z, sym, name, mass, group, period, cat = elem
            info = f'NAME: {name}\nSYMBOL: {sym}\nATOMIC NUMBER: {z}\nATOMIC MASS: {mass} u\nGROUP: {group}\nPERIOD: {period}\nCATEGORY: {cat.upper()}'
            messagebox.showinfo(f'Element {z}: {sym}', info)