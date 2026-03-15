# Generated method: Maths_Classes_1_5.place
import math, random

class Maths_Classes_1_5:
    @staticmethod
    def place(n, d):
        s = str(n)
        if str(d) in s:
            p = len(s) - s.find(str(d)) - 1
            return {'Value': d * 10 ** p}
        return {'Error': 'Digit not found'}