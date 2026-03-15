# Generated method: Biology_Classes_11_12.bod
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def bod(do0, do5):
        bod_val = do0 - do5
        return {'BOD (mg/L)': bod_val, 'Quality': 'Clean' if bod_val < 3 else 'Polluted'}