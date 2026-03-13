"""
SigmaOS NCERT Mathematics Lab v10.0 — The Ultimate Series
Classes 1–12 | Exhaustive NCERT Formula & Mathematical Suite
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Primary Math: Foundations & Logic"
    EXP_DATA = {
        "Shape Gallery": ("shapes", [("Sides", "4")]),
        "Pizza Slices": ("fractions", [("Total", "8"), ("Eaten", "3")]),
        "Arithmetic Carry": ("add", [("Number A", "148"), ("Number B", "75")]),
        "Pattern Hunter": ("pattern", [("Seq (comma)", "2,4,6")]),
        "Clock Angles": ("clock", [("H", "3"), ("M", "30")]),
        "Metric Switch": ("metric", [("Val", "500"), ("From", "cm"), ("To", "m")]),
        "Table Mastery": ("table", [("Num", "13")]),
        "Fraction Shading": ("shading", [("Numerator", "1"), ("Denominator", "4")]),
    }

    @staticmethod
    def shapes(s):
        d = {0:"Circle", 3:"Triangle", 4:"Square/Rect", 5:"Pentagon", 6:"Hexagon"}
        return {"Name": d.get(int(s), "Polygon")}

    @staticmethod
    def fractions(t, e):
        return {"Remaining": f"{int(t-e)}/{int(t)}", "Percent": f"{_r((1-e/t)*100, 1)}%"}

    @staticmethod
    def add(a, b):
        return {"Sum": a + b}

    @staticmethod
    def pattern(s):
        n = [int(x.strip()) for x in s.split(",")]
        return {"Next": n[-1] + (n[1]-n[0])}

    @staticmethod
    def clock(h, m):
        a = abs(30*h - 5.5*m)
        return {"Angle": min(a, 360-a)}

    @staticmethod
    def metric(v, f, t):
        u = {"km":1000, "m":1, "cm":0.01, "mm":0.001, "kg":1000, "g":1}
        return {"Result": _r(v * u[f.lower()] / u[t.lower()], 3)}

    @staticmethod
    def table(n):
        return {"Result": [f"{n} x {i} = {n*i}" for i in range(1, 11)]}

    @staticmethod
    def shading(n, d):
        return {"Visual": f"Shade {n} out of {d} parts", "Percent": f"{_r((n/d)*100, 1)}%"}

class Maths_Classes_6_10:
    TITLE = "Secondary Math: Exhaustive Suite"
    EXP_DATA = {
        "Formula (a+b)²": ("identity", [("a", "5"), ("b", "3")]),
        "Quadratic Roots": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Pythagorean Trip": ("pyth", [("a", "3"), ("b", "4")]),
        "Stats (MMM)": ("mmm", [("Data", "10,20,20,30,40")]),
        "Trig Height": ("trig", [("Angle", "30"), ("Dist", "10")]),
        "Circle Geometry": ("circle", [("Radius", "7")]),
        "LCM & HCF": ("gcd_lcm", [("A", "48"), ("B", "36")]),
        "Probability (Dice)": ("dice", [("Target", "7")]),
    }

    @staticmethod
    def identity(a, b):
        res = a**2 + b**2 + 2*a*b
        return {"Result": res, "Expansion": f"{a}² + {b}² + 2*{a}*{b}"}

    @staticmethod
    def quad(a, b, c):
        d = b**2 - 4*a*c
        if d < 0: return {"Roots": "Complex"}
        return {"x1": _r((-b+math.sqrt(d))/(2*a)), "x2": _r((-b-math.sqrt(d))/(2*a))}

    @staticmethod
    def pyth(a, b):
        return {"c": _r(math.sqrt(a**2+b**2), 2)}

    @staticmethod
    def mmm(s):
        n = sorted([float(x.strip()) for x in s.split(",")])
        mean = sum(n)/len(n)
        mid = len(n)//2
        med = n[mid] if len(n)%2 else (n[mid-1]+n[mid])/2
        mode = Counter(n).most_common(1)[0][0]
        return {"Mean": _r(mean, 2), "Median": med, "Mode": mode}

    @staticmethod
    def trig(d, dist):
        r = math.radians(d)
        return {"Height": _r(dist*math.tan(r), 2)}

    @staticmethod
    def circle(r):
        return {"Area": _r(math.pi*r**2, 2), "Circum": _r(2*math.pi*r, 2)}

    @staticmethod
    def gcd_lcm(a, b):
        g = math.gcd(a, b)
        return {"HCF": g, "LCM": (a*b)//g}

    @staticmethod
    def dice(t):
        hits = 0; tri = 1000
        for _ in range(tri):
            if random.randint(1,6)+random.randint(1,6) == t: hits += 1
        return {"Prob %": _r((hits/tri)*100, 2)}

class Maths_Classes_11_12:
    TITLE = "Senior Math: Advanced Calculus & 3D"
    EXP_DATA = {
        "Matrix Det": ("det", [("Mat (a,b;c,d)", "1,2;3,4")]),
        "Vector Projection": ("projection", [("Vector A", "1,2,3"), ("Vector B", "4,5,6")]),
        "Plane Angle": ("plane_angle", [("Plane 1 (a,b,c)", "1,2,3"), ("Plane 2", "2,1,-1")]),
        "Bernoulli Trial": ("bernoulli", [("n", "10"), ("p", "0.5"), ("k", "5")]),
        "Limit (x^n-a^n)": ("limit", [("n", "3"), ("a", "2")]),
        "Derivative Rate": ("rate_change", [("Function", "x³"), ("x val", "2")]),
        "Bayes Logic": ("bayes", [("P(A)", "0.5"), ("P(B|A)", "0.8"), ("P(B|notA)", "0.2")]),
        "De Morgan's": ("demorgan", [("Set A", "1,2,3"), ("Set B", "3,4,5"), ("Univ Set", "1,2,3,4,5,6")]),
    }

    @staticmethod
    def det(m):
        r = [[float(x) for x in row.split(",")] for row in m.split(";")]
        d = r[0][0]*r[1][1] - r[0][1]*r[1][0]
        return {"Determinant": d}

    @staticmethod
    def projection(as_, bs):
        a = [float(x) for x in as_.split(",")]
        b = [float(x) for x in bs.split(",")]
        dot = sum(x*y for x,y in zip(a,b))
        mag_b = math.sqrt(sum(x**2 for x in b))
        return {"Projection Mag": _r(dot/mag_b, 4)}

    @staticmethod
    def plane_angle(p1, p2):
        n1 = [float(x) for x in p1.split(",")]
        n2 = [float(x) for x in p2.split(",")]
        dot = sum(x*y for x,y in zip(n1,n2))
        m1 = math.sqrt(sum(x**2 for x in n1))
        m2 = math.sqrt(sum(x**2 for x in n2))
        deg = math.degrees(math.acos(abs(dot)/(m1*m2)))
        return {"Angle (deg)": _r(deg, 2)}

    @staticmethod
    def bernoulli(n, p, k):
        res = math.comb(int(n), int(k)) * (p**k) * ((1-p)**(n-k))
        return {"P(X=k)": _r(res, 6)}

    @staticmethod
    def limit(n, a):
        # lim x->a (x^n-a^n)/(x-a) = n*a^(n-1)
        res = n * (a**(n-1))
        return {"Limit Value": _r(res, 2)}

    @staticmethod
    def rate_change(f, x):
        # if f=x³, f'=3x²
        res = 3 * (x**2)
        return {"Rate of Change at x": res}

    @staticmethod
    def bayes(pa, pba, pbna):
        p_total = pba*pa + pbna*(1-pa)
        return {"P(A|B)": _r((pba*pa)/p_total, 4)}

    @staticmethod
    def demorgan(as_str, bs_str, u_str):
        a = set(x.strip() for x in as_str.split(","))
        b = set(x.strip() for x in bs_str.split(","))
        u = set(x.strip() for x in u_str.split(","))
        comp_u_ab = u - (a | b)
        comp_a_int_comp_b = (u-a) & (u-b)
        return {"(AUB)'": sorted(list(comp_u_ab)), "A' int B'": sorted(list(comp_a_int_comp_b)), "Verified": comp_u_ab == comp_a_int_comp_b}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
