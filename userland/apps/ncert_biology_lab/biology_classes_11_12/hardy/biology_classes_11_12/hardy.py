# Generated method: Biology_Classes_11_12.hardy
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def hardy(p, n):
        q = 1 - p
        return {'AA': int(n * p ** 2), 'Aa': int(n * 2 * p * q), 'aa': int(n * q ** 2)}