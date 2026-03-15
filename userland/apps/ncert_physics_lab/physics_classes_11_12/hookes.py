"""
Auto-split from userland\apps\ncert_physics_lab.py — Physics_Classes_11_12.hookes
"""

import math, random



class Physics_Classes_11_12:
    @staticmethod
    def hookes(m, a, l, dl):
        stress = m * 9.81 / a
        strain = dl * 0.001 / l
        y = stress / strain
        return {"Young's Modulus (Pa)": f'{y:.4e}', 'Result': 'Elastic' if strain < 0.01 else 'Plastic'}
