# Generated method: SovereignAINexus._mark_done
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json

class SovereignAINexus:
    def _mark_done(self):
        sel = self.task_tree.selection()
        if sel:
            self.task_tree.item(sel[0], values=(self.task_tree.item(sel[0], 'values')[0], self.task_tree.item(sel[0], 'values')[1], self.task_tree.item(sel[0], 'values')[2], 'Completed', 'Nexus'))