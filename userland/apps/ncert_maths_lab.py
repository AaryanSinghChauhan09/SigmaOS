"""
SigmaOS NCERT Mathematics Lab v6.0 — The Ultimate Series
Classes 1–12 | Every Core NCERT Topic & Interactive Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Primary Math: Numbers, Basic Operations & Shapes"
    EXP_DATA = {
        "Addition (Big Nums)": ("add", [("Number A", "1234"), ("Number B", "5678")]),
        "Place Value": ("place", [("Number", "4563")]),
        "Number Names": ("words", [("Number (0-1000)", "42")]),
        "Multiplication Tables": ("table", [("Number", "13")]),
        "Clock Angle": ("clock_angle", [("Hour", "3"), ("Minute", "15")]),
        "Shape Properties": ("shape_info", [("Shape", "Square")]),
        "Fraction Visual": ("frac", [("Numerator", "3"), ("Denominator", "4")]),
        "Metric Convert": ("metric", [("Value", "500"), ("From Unit", "cm"), ("To Unit", "m")]),
    }

    @staticmethod
    def add(a, b):
        return {"Sum": float(a)+float(b)}

    @staticmethod
    def place(n):
        s_raw = str(int(n))
        s = "".join(reversed(s_raw))
        labels = ["Ones", "Tens", "Hundreds", "Thousands", "Ten Thousands"]
        return {labels[i]: s[i] for i in range(len(s)) if i < len(labels)}

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
    def clock_angle(h, m):
        h, m = float(h), float(m)
        a = abs(30*h - 5.5*m)
        return {"Angle (deg)": min(a, 360-a)}

    @staticmethod
    def shape_info(s):
        d = {"square": "4 sides equal, 4 angles 90°", "circle": "No sides, no corners", "triangle": "3 sides"}
        return {"Data": d.get(s.lower(), "Refer Primary Math-Magic")}

    @staticmethod
    def frac(n, d):
        n, d = float(n), float(d)
        return {"Decimal": _r(n/d, 3), "Percent": f"{(n/d)*100}%"}

    @staticmethod
    def metric(v, f, t):
        u = {"km":1000, "m":1, "cm":0.01, "mm":0.001, "kg":1000, "g":1}
        f, t = f.lower(), t.lower()
        if f in u and t in u:
            return {"Result": _r(float(v) * u[f] / u[t], 3) + " " + t}
        return {"Error": "Invalid Units"}

class Maths_Classes_6_10:
    TITLE = "Secondary Math: Algebra, Geometry, Trig & Stats"
    EXP_DATA = {
        "Quadratic Solver": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Arithmetic Progression": ("ap", [("First term a", "2"), ("Diff d", "3"), ("n", "10")]),
        "LCM & HCF": ("gcd_lcm", [("Num A", "48"), ("Num B", "36")]),
        "Pythagoras Theorem": ("pyth", [("Base", "3"), ("Perp", "4")]),
        "Surface Area/Vol": ("mens", [("Shape (Cone/Sphere/Cyl)", "Sphere"), ("Radius", "7"), ("Height", "10")]),
        "Trig Ratios": ("trig_r", [("Angle (deg)", "30")]),
        "Coordinate Dist": ("dist", [("P1 (x,y)", "0,0"), ("P2 (x,y)", "3,4")]),
        "Statistics (Mean)": ("mean", [("Numbers (comma)", "10,20,30,40")]),
        "Dice Probability": ("dice", [("Sum Target", "7")]),
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
        return {"n-th term": a + (n-1)*d, "Sum of n": (n/2)*(2*a + (n-1)*d)}

    @staticmethod
    def gcd_lcm(a, b):
        a, b = int(a), int(b)
        g = math.gcd(a, b)
        return {"HCF": g, "LCM": (a*b)//g}

    @staticmethod
    def pyth(a, b):
        return {"Hypotenuse": _r(math.sqrt(float(a)**2 + float(b)**2), 2)}

    @staticmethod
    def mens(s, r, h):
        s, r, h = s.lower(), float(r), float(h)
        if "sphere" in s: return {"Area": _r(4*math.pi*r**2), "Vol": _r((4/3)*math.pi*r**3)}
        if "cone" in s: return {"Vol": _r((1/3)*math.pi*r**2 * h), "Area (CSA)": _r(math.pi*r*math.sqrt(r**2+h**2))}
        if "cyl" in s: return {"Vol": _r(math.pi*r**2 * h), "Area": _r(2*math.pi*r*(r+h))}
        return {"Error": "Refer NCERT"}

    @staticmethod
    def trig_r(deg):
        r = math.radians(float(deg))
        return {"sin": _r(math.sin(r)), "cos": _r(math.cos(r)), "tan": _r(math.tan(r))}

    @staticmethod
    def dist(p1, p2):
        x1, y1 = [float(x) for x in p1.split(",")]
        x2, y2 = [float(x) for x in p2.split(",")]
        return {"Dist": _r(math.sqrt((x2-x1)**2 + (y2-y1)**2), 3)}

    @staticmethod
    def mean(c):
        nums = [float(x.strip()) for x in c.split(",")]
        return {"Mean": _r(sum(nums)/len(nums), 2)}

    @staticmethod
    def dice(t):
        hits = 0; trials = 5000
        for _ in range(trials):
            if random.randint(1,6)+random.randint(1,6) == int(t): hits += 1
        return {"Simulated Prob": f"{hits}/{trials}", "Result %": _r((hits/trials)*100, 2)}

class Maths_Classes_11_12:
    TITLE = "Senior Math: Calculus, Vectors, Matrices & Prob"
    EXP_DATA = {
        "Matrix Multiple (2x2)": ("mmul", [("Mat1 (a,b;c,d)", "1,2;3,4"), ("Mat2", "5,6;7,8")]),
        "Matrix Determinant": ("mdet", [("Mat (a,b;c,d)", "1,2;3,4")]),
        "Vector Dot/Cross": ("vops", [("Vector A (i,j,k)", "1,2,3"), ("Vector B", "4,5,6")]),
        "Differentiate x^n": ("diff", [("n", "3")]),
        "Integration (x^n)": ("integ", [("n", "2"), ("Lower", "0"), ("Upper", "1")]),
        "Bayes' Law": ("bayes", [("P(A)", "0.5"), ("P(B|A)", "0.8"), ("P(B|notA)", "0.2")]),
        "LPP Linear solver": ("lpp", [("Weights (w1,w2)", "10,20"), ("Limit SUM", "100")]),
        "Set Operations": ("sets", [("Set A", "1,2,3"), ("Set B", "3,4,5")]),
    }

    @staticmethod
    def mmul(m1, m2):
        def p(s): return [[float(x) for x in row.split(",")] for row in s.split(";")]
        a, b = p(m1), p(m2)
        res = [[sum(a[i][k]*b[k][j] for k in range(2)) for j in range(2)] for i in range(2)]
        return {"Resulting Matrix": res}

    @staticmethod
    def mdet(m):
        [[a,b],[c,d]] = [[float(x) for x in row.split(",")] for row in m.split(";")]
        return {"Det": (a*d - b*c)}

    @staticmethod
    def vops(a_str, b_str):
        a = [float(x) for x in a_str.split(",")]
        b = [float(x) for x in b_str.split(",")]
        dot = sum(x*y for x,y in zip(a, b))
        cross = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
        return {"Dot Product": dot, "Cross Product": cross}

    @staticmethod
    def diff(n):
        n = int(n)
        return {"Derivative": f"{n}x^{n-1}"}

    @staticmethod
    def integ(n, l, u):
        n, l, u = float(n), float(l), float(u)
        # Integral = [x^(n+1) / (n+1)] from l to u
        val = (u**(n+1) / (n+1)) - (l**(n+1) / (n+1))
        return {"Integral (Definite)": _r(val, 4)}

    @staticmethod
    def bayes(pa, pba, pbna):
        pa, pba, pbna = float(pa), float(pba), float(pbna)
        num = pba * pa
        den = num + (pbna * (1-pa))
        return {"P(A|B)": _r(num/den, 4)}

    @staticmethod
    def lpp(w_str, limit):
        w1, w2 = [float(x) for x in w_str.split(",")]
        lim = float(limit)
        # Simplified: Maximize w1*x + w2*y where x+y <= lim
        v1, v2 = w1*lim, w2*lim
        return {"Max val": max(v1, v2), "At": f"(x={lim}, y=0)" if v1>v2 else f"(x=0, y={lim})"}

    @staticmethod
    def sets(a, b):
        s1 = set(x.strip() for x in a.split(","))
        s2 = set(x.strip() for x in b.split(","))
        return {"Union": sorted(list(s1|s2)), "Intersection": sorted(list(s1&s2)), "A-B": sorted(list(s1-s2))}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
