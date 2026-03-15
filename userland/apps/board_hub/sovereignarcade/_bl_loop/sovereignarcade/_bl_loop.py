# Generated method: SovereignArcade._bl_loop
import tkinter as tk
from tkinter import ttk, messagebox
import random, time, os, sys
from typing import Dict, Any, List, Optional

class SovereignArcade:
    def _bl_loop(self):
        if not self.bl_active:
            return
        self.bl_canv.move(self.ball, self.bl_vx, self.bl_vy)
        bx1, by1, bx2, by2 = self.bl_canv.coords(self.ball)
        if bx1 <= 0 or bx2 >= 400:
            self.bl_vx *= -1
        if by1 <= 0:
            self.bl_vy *= -1
        if by2 >= 500:
            self.bl_active = False
            messagebox.showinfo('Arcade', 'VOID REACHED.')
            self.bl_canv.coords(self.ball, 195, 470, 205, 480)
            return
        px1, py1, px2, py2 = self.bl_canv.coords(self.paddle)
        if by2 >= py1 and px1 <= (bx1 + bx2) / 2 <= px2:
            self.bl_vy *= -1
        for b in list(self.bricks):
            bbox = self.bl_canv.coords(b)
            if bbox[0] <= (bx1 + bx2) / 2 <= bbox[2] and bbox[1] <= (by1 + by2) / 2 <= bbox[3]:
                self.bl_canv.delete(b)
                self.bricks.remove(b)
                self.bl_vy *= -1
                break
        if not self.bricks:
            self.bl_active = False
            messagebox.showinfo('Arcade', 'MATRIX CLEARED.')
        self.after(16, self._bl_loop)