"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.zener
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def zener(vin, vz):
        if vin < vz:
            return {'V_out (V)': vin, 'Status': 'Normal Forward/Reverse'}
        return {'V_out (V)': vz, 'Status': 'Breakdown/Regulated'}
