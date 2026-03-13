"""
SigmaOS NCERT Mathematics Lab v4.0
Classes 1–12 | Every NCERT Math topic & Simulation
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Classes 1–5 – Numbers, Counting & Shapes"
    EXP_DATA = {
        "Addition with Carry": ("add", [("Num 1", "48"), ("Num 2", "75")]),
        "Place Value": ("place", [("Number", "4563")]),
        "Number Names": ("to_words", [("Number (0-99)", "42")]),
        "Shapes Analysis": ("shapes", [("Shape", "Square")]),
        "Clock Angle": ("clock", [("Hours", "10"), ("Minutes", "10")]),
    }

    @staticmethod
    def add(a, b):
        return {"Sum": int(a)+int(b), "Visual": f"{int(a)} + {int(b)} = {int(a)+int(b)}"}

    @staticmethod
    def place(n):
        s_raw = str(int(n))
        s = "".join(reversed(s_raw))
        labels = ["Units", "Tens", "Hundreds", "Thousands", "Ten Thousands"]
        return {labels[i]: s[i] for i in range(len(s)) if i < len(labels)}

    @staticmethod
    def to_words(n):
        units = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
        teens = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"]
        tens = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"]
        v = int(n)
        if 0 <= v < 10: return {"result": units[v]}
        if 10 <= v < 20: return {"result": teens[v-10]}
        if 20 <= v < 100:
            res = tens[v//10]
            if v%10: res += " " + units[v%10]
            return {"result": res}
        return {"result": str(v)}

    @staticmethod
    def shapes(name):
        d = {"square":"4 equal sides", "circle":"Round", "triangle":"3 sides"}
        return {"Info": d.get(name.lower(), "Refer NCERT")}

    @staticmethod
    def clock(h, m):
        h, m = int(h), int(m)
        angle = abs(30*h - 5.5*m)
        return {"Angle": min(angle, 360-angle)}

class Maths_Classes_6_10:
    TITLE = "Classes 6–10 – Algebra, Trig, Geometry & Stats"
    EXP_DATA = {
        "Quadratic Solver": ("quadratic", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Surface Area Cone": ("cone", [("Radius r", "7"), ("Height h", "24")]),
        "LCM & HCF": ("lcm_hcf", [("Num A", "48"), ("Num B", "36")]),
        "Coordinate Dist": ("dist", [("x1,y1", "0,0"), ("x2,y2", "3,4")]),
        "Probability Dice": ("dice", [("Target Sum", "7"), ("Rolls", "1000")]),
        "Prime Factors": ("primes", [("Number", "120")]),
        "Roman Numerals": ("roman", [("Number (1-10)", "4")]),
    }

    @staticmethod
    def quadratic(a, b, c):
        a, b, c = float(a), float(b), float(c)
        d = b**2 - 4*a*c
        if d < 0: return {"Roots": "Complex"}
        x1 = (-b + math.sqrt(d))/(2*a)
        x2 = (-b - math.sqrt(d))/(2*a)
        return {"x1": _r(x1), "x2": _r(x2)}

    @staticmethod
    def cone(r, h):
        r, h = float(r), float(h)
        l = math.sqrt(r**2 + h**2)
        vol = (1/3) * math.pi * r**2 * h
        return {"Volume": _r(vol, 2), "Slant l": _r(l, 2)}

    @staticmethod
    def lcm_hcf(a, b):
        a, b = int(a), int(b)
        h = math.gcd(a, b)
        return {"HCF": h, "LCM": abs(a*b)//h}

    @staticmethod
    def dist(p1, p2):
        x1, y1 = [float(x) for x in p1.split(",")]
        x2, y2 = [float(x) for x in p2.split(",")]
        d = math.sqrt((x2-x1)**2 + (y2-y1)**2)
        return {"Distance": _r(d, 3)}

    @staticmethod
    def dice(target, rolls):
        target, rolls = int(target), int(rolls)
        hits = 0
        for _ in range(rolls):
            if random.randint(1,6) + random.randint(1,6) == target: hits += 1
        return {"Frequency": hits, "Probability": _r(hits/rolls, 4)}

    @staticmethod
    def primes(n):
        n = int(n)
        factors = []
        d = 2
        temp = n
        while d*d <= temp:
            while temp % d == 0:
                factors.append(d)
                temp //= d
            d += 1
        if temp > 1: factors.append(temp)
        return {"Factors": factors}

    @staticmethod
    def roman(n):
        n = int(n)
        d = {1:"I", 2:"II", 3:"III", 4:"IV", 5:"V", 6:"VI", 7:"VII", 8:"VIII", 9:"IX", 10:"X"}
        return {"Roman": d.get(n, str(n))}

class Maths_Classes_11_12:
    TITLE = "Classes 11–12 – Vectors, Calculus, Matrices & LPP"
    EXP_DATA = {
        "Vector Ops": ("vector", [("A (x,y,z)", "1,2,3"), ("B (x,y,z)", "4,5,6")]),
        "Matrix Mult": ("matrix", [("A (2x2: a,b;c,d)", "1,2;3,4"), ("B (2x2)", "5,6;7,8")]),
        "Linear Programming": ("lpp", [("Weights (w1,w2)", "10,20"), ("Constraint (Max)", "100")]),
        "Differentiation": ("diff", [("Power of x", "3")]),
        "Integration (x^n)": ("integ", [("Power n", "2")]),
        "Sets Union": ("sets", [("Set A (comma)", "1,2,3"), ("Set B", "3,4,5")]),
    }

    @staticmethod
    def vector(a_str, b_str):
        A = [float(x) for x in a_str.split(",")]
        B = [float(x) for x in b_str.split(",")]
        dot = sum(x*y for x,y in zip(A, B))
        return {"Dot Product": dot, "Sum": [x+y for x,y in zip(A, B)]}

    @staticmethod
    def matrix(as_str, bs_str):
        def p(s): return [[float(x) for x in r.split(",")] for r in s.split(";")]
        A, B = p(as_str), p(bs_str)
        res = [[sum(A[i][k]*B[k][j] for k in range(len(B))) for j in range(len(B[0]))] for i in range(len(A))]
        return {"Result": res}

    @staticmethod
    def lpp(w_str, limit):
        w1, w2 = [float(x) for x in w_str.split(",")]
        limit = float(limit)
        # Simplified: Maximize w1*x + w2*y subject to x+y <= limit
        # Corner points (0, limit), (limit, 0)
        v1 = w2 * limit
        v2 = w1 * limit
        return {"Max Value": max(v1, v2), "At point": "(0, limit)" if v1 > v2 else "(limit, 0)"}

    @staticmethod
    def diff(n):
        n = int(n)
        return {"d/dx(x^n)": f"{n}x^{n-1}"}

    @staticmethod
    def integ(n):
        n = int(n)
        return {"Integral": f"(x^{n+1})/{n+1} + C"}

    @staticmethod
    def sets(a, b):
        A = set(x.strip() for x in a.split(","))
        B = set(x.strip() for x in b.split(","))
        return {"Union": sorted(list(A | B)), "Intersection": sorted(list(A & B))}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
