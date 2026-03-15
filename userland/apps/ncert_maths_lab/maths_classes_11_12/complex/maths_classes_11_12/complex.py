# Generated method: Maths_Classes_11_12.complex
import math, random

class Maths_Classes_11_12:
    @staticmethod
    def complex(r, i, p):
        mag = math.sqrt(r ** 2 + i ** 2)
        ang = math.atan2(i, r)
        res_mag = mag ** p
        res_ang = ang * p
        return {'Res': f'{_r(res_mag * math.cos(res_ang), 2)} + {_r(res_mag * math.sin(res_ang), 2)}i'}