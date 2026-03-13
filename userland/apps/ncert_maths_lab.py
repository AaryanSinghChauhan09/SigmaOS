"""
SigmaOS NCERT Mathematics Lab v5.0 — The Complete Series
Classes 1–12 | Comprehensive Calculation & Simulation Engine
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Classes 1–5: Numbers, Basic Ops & Shapes"
    EXP_DATA = {
        "Addition (Big Nums)": ("add", [("A", "456"), ("B", "789")]),
        "Number Names": ("words", [("N (0-999)", "42")]),
        "Multiplication Tables": ("table", [("Num", "13")]),
        "Clock Hands Angle": ("clock_angle", [("Hour", "3"), ("Min", "30")]),
        "Shape Properties": ("shape_info", [("Shape", "Square")]),
        "Fraction Visual": ("frac", [("Num", "1"), ("Den", "4")]),
    }

    @staticmethod
    def add(a, b):
        return {"Sum": a+b}

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
        return {"List": [f"{int(n)} x {i} = {int(n)*i}" for i in range(1, 11)]}

    @staticmethod
    def clock_angle(h, m):
        # |30h - 5.5m|
        a = abs(30*h - 5.5*m)
        return {"Angle (deg)": min(a, 360-a)}

    @staticmethod
    def shape_info(s):
        d = {"square": "4 sides equal, 4 angles 90", "circle": "No sides, no corners", "triangle": "3 sides"}
        return {"Data": d.get(s.lower(), "Refer Class 2-5")}

    @staticmethod
    def frac(n, d):
        return {"Decimal": n/d, "Percent": f"{(n/d)*100}%"}

class Maths_Classes_6_10:
    TITLE = "Classes 6–10: Algebra, Trig, Geometry & Data"
    EXP_DATA = {
        "Quadratic Formula": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Trig Ratios": ("trig_r", [("Angle (deg)", "30")]),
        "Surface Area/Vol": ("mensuration", [("Shape (Cone/Sphere/Cyl)", "Sphere"), ("Radius", "7")]),
        "LCM & HCF": ("gcd_lcm", [("A", "48"), ("B", "36")]),
        "Pythagoras Theorem": ("pyth", [("Base", "3"), ("Perp", "4")]),
        "Simple Interest": ("si", [("Principal", "1000"), ("Rate (%)", "5"), ("Time (yr)", "2")]),
        "Prob (Dice 2x)": ("dice_roll", [("Sum Target", "7")]),
    }

    @staticmethod
    def quad(a, b, c):
        d = b**2 - 4*a*c
        if d < 0: return {"Roots": "Complex"}
        return {"x1": _r((-b + math.sqrt(d))/(2*a)), "x2": _r((-b - math.sqrt(d))/(2*a))}

    @staticmethod
    def trig_r(deg):
        r = math.radians(deg)
        return {"sin": _r(math.sin(r)), "cos": _r(math.cos(r)), "tan": _r(math.tan(r))}

    @staticmethod
    def mensuration(s, r):
        s = s.lower()
        if "sphere" in s: return {"Area": _r(4*math.pi*r**2), "Vol": _r((4/3)*math.pi*r**3)}
        if "cone" in s: return {"Vol": _r((1/3)*math.pi*r**2 * 10), "Note": "Assumed height=10"}
        return {"Error": "Refer NCERT"}

    @staticmethod
    def gcd_lcm(a, b):
        g = math.gcd(int(a), int(b))
        return {"HCF": g, "LCM": int(a*b)//g}

    @staticmethod
    def pyth(a, b):
        return {"Hypotenuse": _r(math.sqrt(a**2 + b**2), 2)}

    @staticmethod
    def si(p, r, t):
        i = (p*r*t)/100
        return {"Interest": i, "Total": p+i}

    @staticmethod
    def dice_roll(t):
        hits = 0
        for _ in range(1000):
            if random.randint(1,6)+random.randint(1,6) == t: hits += 1
        return {"Frequency": f"{hits}/1000", "Prob %": _r((hits/1000)*100, 1)}

class Maths_Classes_11_12:
    TITLE = "Classes 11–12: Calculus, Vectors, Matrices & Prob"
    EXP_DATA = {
        "Matrix Multiple (2x2)": ("mmul", [("Mat1 (a,b;c,d)", "1,2;3,4"), ("Mat2", "5,6;7,8")]),
        "Vector Ops": ("vops", [("A (i,j,k)", "1,2,3"), ("B", "4,5,6")]),
        "Differentiate x^n": ("diff", [("n", "3")]),
        "Integrate x^n": ("integ", [("n", "2")]),
        "Bayes' Rule": ("bayes", [("PA", "0.5"), ("PB|A", "0.8"), ("PB|notA", "0.2")]),
        "Set Operations": ("sets", [("SetA", "1,2,3"), ("SetB", "3,4,5")]),
        "Binomial Prob": ("bin", [("n trials", "10"), ("p prob", "0.5"), ("k success", "5")]),
    }

    @staticmethod
    def mmul(m1, m2):
        def p(s): return [[float(x) for x in row.split(",")] for row in s.split(";")]
        a = p(m1); b = p(m2)
        res = [[sum(a[i][k]*b[k][j] for k in range(2)) for j in range(2)] for i in range(2)]
        return {"Matrix": res}

    @staticmethod
    def vops(a_str, b_str):
        a = [float(x) for x in a_str.split(",")]
        b = [float(x) for x in b_str.split(",")]
        dot = sum(x*y for x,y in zip(a, b))
        return {"Dot Product": dot, "Sum": [x+y for x,y in zip(a, b)]}

    @staticmethod
    def diff(n):
        return {"d/dx": f"{int(n)}x^{int(n-1)}"}

    @staticmethod
    def integ(n):
        return {"integral": f"(x^{int(n+1)})/{int(n+1)} + C"}

    @staticmethod
    def bayes(pa, pba, pbna):
        p_not_a = 1-pa
        num = pba * pa
        den = num + (pbna * p_not_a)
        return {"PA|B": _r(num/den, 4)}

    @staticmethod
    def sets(a, b):
        s1 = set(a.split(",")); s2 = set(b.split(","))
        return {"Union": list(s1 | s2), "Intersection": list(s1 & s2)}

    @staticmethod
    def bin(n, p, k):
        c = math.comb(int(n), int(k))
        res = c * (p**k) * ((1-p)**(n-k))
        return {"Result": _r(res, 6)}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
