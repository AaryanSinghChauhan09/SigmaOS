"""
SigmaOS NCERT Mathematics Lab v8.0 — The Interactive series
Classes 1–12 | Every Core NCERT Topic & Interactive Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Primary Math: Interactive Learning"
    EXP_DATA = {
        "Shapes Builder": ("shapes", [("Sides", "4")]),
        "Pizza Fraction Slicer": ("fractions", [("Total Slices", "8"), ("Eaten Slices", "3")]),
        "Addition carry/sum": ("add", [("Number A", "148"), ("Number B", "75")]),
        "Pattern Hunter": ("pattern", [("Sequence (comma)", "2,4,6")]),
        "Number Names": ("words", [("N (0-1000)", "42")]),
        "Clock Angle": ("clock_angle", [("Hour", "3"), ("Min", "15")]),
        "Measurement Convert": ("metric", [("Value", "500"), ("Unit (cm/m/km/g/kg)", "cm"), ("To", "m")]),
    }

    @staticmethod
    def shapes(s):
        s = int(s)
        d = {0:"Circle (No sides)", 1: "Line", 3: "Triangle", 4: "Square/Rectangle", 5:"Pentagon", 6:"Hexagon"}
        return {"Shape": d.get(s, "Polygon with " + str(s) + " sides")}

    @staticmethod
    def fractions(t, e):
        t, e = float(t), float(e)
        return {"Fraction Remaining": f"{int(t-e)}/{int(t)}", "Percentage Left": f"{_r((1-e/t)*100, 1)}%"}

    @staticmethod
    def add(a, b):
        return {"Sum": float(a)+float(b)}

    @staticmethod
    def pattern(s):
        nums = [int(x.strip()) for x in s.split(",")]
        diff = nums[1]-nums[0]
        return {"Next": nums[-1] + diff}

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
    def clock_angle(h, m):
        h, m = float(h), float(m)
        a = abs(30*h - 5.5*m)
        return {"Small Angle": min(a, 360-a)}

    @staticmethod
    def metric(v, f, t):
        u = {"km":1000, "m":1, "cm":0.01, "mm":0.001, "kg":1000, "g":1}
        f, t = f.lower(), t.lower()
        if f in u and t in u:
            return {"Result": _r(float(v) * u[f] / u[t], 3) + " " + t}
        return {"Error": "Invalid Units"}

class Maths_Classes_6_10:
    TITLE = "Secondary Math: Functions & Logic"
    EXP_DATA = {
        "Graph Plotting (Line)": ("graph", [("Slope m", "2"), ("Constant c", "3"), ("x value", "5")]),
        "Quadratic Solver": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Pythagoras Verification": ("pyth", [("a", "3"), ("b", "4")]),
        "Tessellation Checker": ("tessell", [("Sides of regular polygon", "4")]),
        "Dice Probability": ("dice", [("Target Sum", "7"), ("Trials", "2000")]),
        "Trigonometry (Height)": ("trig_h", [("Angle (deg)", "30"), ("Distance from base", "10")]),
        "HCF & LCM": ("gcd_lcm", [("A", "48"), ("B", "36")]),
        "Coordinate Dist": ("dist", [("x1,y1", "0,0"), ("x2,y2", "3,4")]),
    }

    @staticmethod
    def graph(m, c, x):
        y = float(m)*float(x) + float(c)
        return {"Resulting y": y, "Eq": f"y = {m}x + {c}"}

    @staticmethod
    def quad(a, b, c):
        a, b, c = float(a), float(b), float(c)
        d = b**2 - 4*a*c
        if d < 0: return {"Roots": "Complex"}
        return {"x1": _r((-b + math.sqrt(d))/(2*a)), "x2": _r((-b - math.sqrt(d))/(2*a))}

    @staticmethod
    def pyth(a, b):
        c = math.sqrt(float(a)**2 + float(b)**2)
        return {"Hypotenuse c": _r(c, 2)}

    @staticmethod
    def tessell(n):
        n = int(n)
        angle = (n - 2) * 180 / n
        if 360 % angle == 0: return {"Status": "YES", "Interior Angle": angle}
        return {"Status": "NO", "Interior Angle": angle}

    @staticmethod
    def dice(t, tri):
        hits = 0; n = int(tri)
        for _ in range(n):
            if random.randint(1,6)+random.randint(1,6) == int(t): hits += 1
        return {"Prob %": _r((hits/n)*100, 2), "Frequency": f"{hits}/{n}"}

    @staticmethod
    def trig_h(deg, d):
        r = math.radians(float(deg))
        h = float(d) * math.tan(r)
        return {"Height of Object": _r(h, 2)}

    @staticmethod
    def gcd_lcm(a, b):
        a, b = int(a), int(b)
        g = math.gcd(a, b)
        return {"HCF": g, "LCM": (a*b)//g}

    @staticmethod
    def dist(p1, p2):
        x1, y1 = [float(x) for x in p1.split(",")]
        x2, y2 = [float(x) for x in p2.split(",")]
        return {"Distance": _r(math.sqrt((x2-x1)**2 + (y2-y1)**2), 3)}

class Maths_Classes_11_12:
    TITLE = "Senior Math: Advanced Calculus & Probability"
    EXP_DATA = {
        "Matrix Mult (2x2)": ("mmul", [("Mat A (a,b;c,d)", "1,2;3,4"), ("Mat B", "5,6;7,8")]),
        "Vector Cross Product": ("vcross", [("A (i,j,k)", "1,0,0"), ("B", "0,1,0")]),
        "Definite Integral x^n": ("integ", [("n", "2"), ("Limit lower", "0"), ("Upper", "1")]),
        "Binomial Dist": ("binomial", [("n trials", "10"), ("p prob", "0.5"), ("k successes", "5")]),
        "Normal Curve Logic": ("normal", [("Mean", "50"), ("Std Dev", "5"), ("x val", "55")]),
        "Set Union/Int": ("sets", [("Set A", "1,2,3"), ("Set B", "3,4,5")]),
        "LPP Optimization": ("lpp", [("W1", "10"), ("W2", "15"), ("Lim", "100")]),
    }

    @staticmethod
    def mmul(m1, m2):
        def p(s): return [[float(x) for x in row.split(",")] for row in s.split(";")]
        a, b = p(m1), p(m2)
        res = [[sum(a[i][k]*b[k][j] for k in range(2)) for j in range(2)] for i in range(2)]
        return {"Result": res}

    @staticmethod
    def vcross(a_str, b_str):
        a = [float(x) for x in a_str.split(",")]
        b = [float(x) for x in b_str.split(",")]
        cross = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
        return {"i,j,k": cross}

    @staticmethod
    def integ(n, l, u):
        n, l, u = float(n), float(l), float(u)
        val = (u**(n+1)/(n+1)) - (l**(n+1)/(n+1))
        return {"Value": _r(val, 4)}

    @staticmethod
    def binomial(n, p, k):
        n, p, k = int(n), float(p), int(k)
        c = math.comb(n, k)
        res = c * (p**k) * ((1-p)**(n-k))
        return {"P(X=k)": _r(res, 6)}

    @staticmethod
    def normal(m, s, x):
        m, s, x = float(m), float(s), float(x)
        z = (x - m) / s
        p = 0.5 * (1 + math.erf(z / math.sqrt(2)))
        return {"Z-score": _r(z, 2), "Percentile Approx": f"{_r(p*100, 1)}%"}

    @staticmethod
    def sets(a, b):
        s1 = set(x.strip() for x in a.split(","))
        s2 = set(x.strip() for x in b.split(","))
        return {"Union": sorted(list(s1|s2)), "Int": sorted(list(s1&s2))}

    @staticmethod
    def lpp(w1, w2, l):
        w1, w2, l = float(w1), float(w2), float(l)
        v1, v2 = w1*l, w2*l
        return {"Max val": max(v1, v2), "At": "(l,0)" if v1>v2 else "(0,l)"}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
