# Generated method: Biology_Classes_11_12.pedigree
import math, random

class Biology_Classes_11_12:
    @staticmethod
    def pedigree(f, m):
        f, m = (f.lower(), m.lower())
        if 'affected' in f and 'carrier' in m:
            return {'Risk': '50% Affected', 'Type': 'X-Linked Possible'}
        if 'affected' in f and 'affected' in m:
            return {'Risk': '100% Affected', 'Status': 'Homozygous'}
        return {'Risk': 'Calculating...', 'Action': 'Check Gen2'}