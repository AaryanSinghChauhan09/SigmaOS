# Generated method: Chemistry_Classes_11_12.reaction_rate
import math, re

class Chemistry_Classes_11_12:
    @staticmethod
    def reaction_rate(c1, c2, t):
        rate = abs(c2 - c1) / t
        return {'Avg Rate (M/s)': f'{rate:.4e}'}