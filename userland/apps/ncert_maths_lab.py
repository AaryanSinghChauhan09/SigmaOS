"""
SigmaOS NCERT Mathematics Lab v7.0 — The Ultimate Lab Manual
Classes 1–12 | Every Core NCERT Formula & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Primary Mathematics Foundations"
    EXP_DATA = {
        "Addition (carrying)": ("add", [("Number 1", "48"), ("Number 2", "75")]),
        "Place Value Logic": ("place", [("Number", "4563")]),
        "Number Names": ("words", [("Number (0-1000)", "42")]),
        "Tables Generator": ("table", [("Num", "13")]),
        "Clock Angle": ("clock", [("Hours", "10"), ("Minutes", "10")]),
        "Geometry Shapes": ("shapes", [("Shape", "Circle")]),
        "Money Arithmetic": ("money", [("Price 1 (Rs)", "45.50"), ("Price 2 (Rs)", "20.25")]),
    }

    @staticmethod
    def add(a, b):
        return {"Sum": int(a)+int(b), "Visual": f"{a} + {b} = {int(a)+int(b)}"}

    @staticmethod
    def place(n):
        s = "".join(reversed(str(int(n))))
        lbl = ["Ones", "Tens", "Hundreds", "Thousands", "Ten Thousands"]
        return {lbl[i]: s[i] for i in range(len(s)) if i < len(lbl)}

    @staticmethod
    def words(n):
        u = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
        t = ["Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"]
        tens = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"]
        v = int(n)
        if v < 10: return {"Word": u[v]}
        if v < 20: return {"Word": t[v-10]}
        if v < 100: return {"Word": tens[v//10] + (" "+u[v%10] if v%10 else "")}
        return {"Word": str(v)}

    @staticmethod
    def table(n):
        n = int(n)
        return {"Table": [f"{n} x {i} = {n*i}" for i in range(1, 11)]}

    @staticmethod
    def clock(h, m):
        h, m = float(h), float(m)
        a = abs(30*h - 5.5*m)
        return {"Angle (deg)": min(a, 360-a)}

    @staticmethod
    def shapes(s):
        d = {"circle": "No corners, 1 curved side", "square": "4 equal sides, 4 corners", "triangle": "3 sides, 3 corners"}
        return {"Key Property": d.get(s.lower(), "Refer Class 2 Math-Magic")}

    @staticmethod
    def money(p1, p2):
        return {"Total Cost": f"₹ {_r(float(p1)+float(p2), 2)}"}

class Maths_Classes_6_10:
    TITLE = "Secondary Mathematics Lab"
    EXP_DATA = {
        "Quadratic Formula": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Arithmetic Progression": ("ap", [("a", "2"), ("d", "3"), ("n", "10")]),
        "LCM & HCF": ("gcd_lcm", [("Num A", "48"), ("Num B", "36")]),
        "Coordinate Geometry": ("coord", [("P1 (x,y)", "0,0"), ("P2 (x,y)", "3,4")]),
        "Surface Area/Vol": ("mensuration", [("Shape (Cone/Cyl/Sphere)", "Sphere"), ("Radius", "7"), ("Height", "10")]),
        "Trigonometric Solver": ("trig", [("Angle (deg)", "30")]),
        "Probability Statistics": ("stats", [("Data (comma)", "10,20,30,40"), ("Dice Target", "7")]),
    }

    @staticmethod
    def quad(a, b, c):
        a, b, c = float(a), float(b), float(c)
        d = b**2 - 4*a*c
        if d < 0: return {"Roots": "Complex"}
        return {"x1": _r((-b + math.sqrt(d))/(2*a)), "x2": _r((-b - math.sqrt(d))/(2*a))}

    @staticmethod
    def ap(a, d, n):
        a, d, n = float(a), float(d), float(n)
        an = a + (n-1)*d
        sn = (n/2) * (2*a + (n-1)*d)
        return {"n-th Term (an)": an, "Sum (sn)": sn}

    @staticmethod
    def gcd_lcm(a, b):
        a, b = int(a), int(b)
        g = math.gcd(a, b)
        return {"HCF": g, "LCM": (a*b)//g}

    @staticmethod
    def coord(p1, p2):
        x1, y1 = [float(x) for x in p1.split(",")]
        x2, y2 = [float(x) for x in p2.split(",")]
        dist = math.sqrt((x2-x1)**2 + (y2-y1)**2)
        mid = [(x1+x2)/2, (y1+y2)/2]
        return {"Distance": _r(dist, 3), "Midpoint": mid}

    @staticmethod
    def mensuration(s, r, h):
        s, r, h = s.lower(), float(r), float(h)
        if "sphere" in s: return {"Area": _r(4*math.pi*r**2), "Vol": _r((4/3)*math.pi*r**3)}
        if "cyl" in s: return {"Vol": _r(math.pi*r**2*h), "CSA": _r(2*math.pi*r*h)}
        if "cone" in s: return {"Vol": _r((1/3)*math.pi*r**2*h)}
        return {"Error": "Shape unknown"}

    @staticmethod
    def trig(d):
        r = math.radians(float(d))
        return {"sin": _r(math.sin(r)), "cos": _r(math.cos(r)), "tan": _r(math.tan(r))}

    @staticmethod
    def stats(c_str, d):
        nums = [float(x.strip()) for x in c_str.split(",")]
        mean = sum(nums)/len(nums)
        # Prob
        hits = 0; tri = 2000
        for _ in range(tri):
            if random.randint(1,6)+random.randint(1,6) == int(d): hits += 1
        return {"Mean": _r(mean, 2), "Dice Prob %": _r((hits/tri)*100, 2)}

class Maths_Classes_11_12:
    TITLE = "Senior Mathematics & Advanced Calculus"
    EXP_DATA = {
        "Matrix Operations": ("matrix", [("Mat A (a,b;c,d)", "1,2;3,4"), ("Mat B", "5,6;7,8")]),
        "Vector Dot/Cross": ("vectors", [("A (i,j,k)", "1,2,3"), ("B", "4,5,6")]),
        "Limits (x->a for x^n)": ("limits", [("Function (x^n)", "3"), ("Approaching a", "2")]),
        "Calculus (Diff/Integ)": ("calculus", [("Power n", "3"), ("Lower Bound", "0"), ("Upper Bound", "1")]),
        "Conditional Prob": ("prob_cond", [("PA", "0.5"), ("PB", "0.3"), ("PA_and_B", "0.15")]),
        "Sets Logic": ("sets", [("Set A", "1,2,3"), ("Set B", "3,4,5")]),
        "LPP Optimization": ("lpp", [("W1", "10"), ("W2", "15"), ("Constraint Sum", "100")]),
    }

    @staticmethod
    def matrix(m1, m2):
        def p(s): return [[float(x) for x in row.split(",")] for row in s.split(";")]
        a, b = p(m1), p(m2)
        det_a = a[0][0]*a[1][1] - a[0][1]*a[1][0]
        mult = [[sum(a[i][k]*b[k][j] for k in range(2)) for j in range(2)] for i in range(2)]
        return {"Det(A)": det_a, "A x B": mult}

    @staticmethod
    def vectors(a_str, b_str):
        a = [float(x) for x in a_str.split(",")]
        b = [float(x) for x in b_str.split(",")]
        dot = sum(x*y for x,y in zip(a, b))
        cross = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
        return {"Dot Product": dot, "Cross Product": cross}

    @staticmethod
    def limits(n, a):
        n, a = float(n), float(a)
        # lim x->a (x^n) = a^n
        return {"Limit result": _r(a**n, 4)}

    @staticmethod
    def calculus(n, l, u):
        n, l, u = float(n), float(l), float(u)
        diff = f"{int(n)}x^{int(n-1)}"
        integ = (u**(n+1)/(n+1)) - (l**(n+1)/(n+1))
        return {"Derivative d/dx": diff, "Definite Integral": _r(integ, 4)}

    @staticmethod
    def prob_cond(pa, pb, pab):
        pa, pb, pab = float(pa), float(pb), float(pab)
        p_a_given_b = pab / pb if pb > 0 else 0
        return {"P(A|B)": _r(p_a_given_b, 4)}

    @staticmethod
    def sets(a_str, b_str):
        s1 = set(x.strip() for x in a_str.split(","))
        s2 = set(x.strip() for x in b_str.split(","))
        return {"Union": sorted(list(s1|s2)), "Intersection": sorted(list(s1&s2)), "A-B": sorted(list(s1-s2))}

    @staticmethod
    def lpp(w1, w2, c):
        w1, w2, c = float(w1), float(w2), float(c)
        # Maximize w1*x + w2*y where x+y <= c
        # Vertex test: (0,c) or (c,0)
        v1, v2 = w1*c, w2*c
        return {"Max val": max(v1, v2), "At": "(c,0)" if v1 > v2 else "(0,c)"}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
