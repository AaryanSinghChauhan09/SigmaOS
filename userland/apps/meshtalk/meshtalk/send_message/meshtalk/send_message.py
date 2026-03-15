# Generated method: MeshTalk.send_message
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class MeshTalk:
    def send_message(self, event=None):
        msg = self.msg_var.get()
        if msg:
            self._add_log('LOC_NODE', msg, color=PAL['accent'])
            self.msg_var.set('')
            if 'hello' in msg.lower():
                self.after(1000, lambda: self._add_log('REMOTE_NODE_4', 'Salutations from the Aether Mesh.'))