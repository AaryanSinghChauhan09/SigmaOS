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
        "Word Express": ("words", [("N", "7")]),
        "Addition Carry": ("add", [("A", "45"), ("B", "56")]),
        "Multiplication (Repeated)": ("mul", [("A", "5"), ("B", "6")]),
        "Fraction Sharing": ("share", [("Shaded", "1"), ("Total", "4")]),
        "Pattern Hunter": ("pattern", [("Seq", "2,4,6")]),
        "Sorting Order": ("sort", [("Data", "5,2,9,1,7")]),
        "Clock Angle": ("clock_angle", [("H", "3"), ("M", "30")]),
        "Shape Corners": ("shapes", [("SideCount", "4")]),
    }

    @staticmethod
    def words(n):
        d = {0:"Zero", 1:"One", 2:"Two", 3:"Three", 4:"Four", 5:"Five", 6:"Six", 7:"Seven", 8:"Eight", 9:"Nine"}
        return {"Word": d.get(int(n), "Out of range")}

    @staticmethod
    def add(a, b):
        return {"Sum": a+b, "Carry": (a%10+b%10)>=10}

    @staticmethod
    def mul(a, b):
        return {"Product": a*b}

    @staticmethod
    def share(s, t):
        return {"Fraction": f"{s}/{t}", "Percent": _r(s/t*100, 1)}

    @staticmethod
    def pattern(s):
        n = [int(x) for x in s.split(",")]
        return {"Next": n[-1]+(n[1]-n[0])}

    @staticmethod
    def sort(s):
        n = [int(x) for x in s.split(",")]
        return {"ASC": sorted(n), "DESC": sorted(n, reverse=True)}

    @staticmethod
    def clock_angle(h, m):
        a = abs(30*h - 5.5*m)
        return {"Angle": min(a, 360-a)}

    @staticmethod
    def shapes(s):
        d = {0:"Circle", 3:"Triangle", 4:"Square", 5:"Pentagon", 6:"Hexagon"}
        return {"Name": d.get(int(s), "Polygon")}

class Maths_Classes_6_10:
    TITLE = "Secondary Math: Analysis & Geometry"
    EXP_DATA = {
        "Identity (a+b)²": ("identity", [("a", "5"), ("b", "3")]),
        "Quadratic Solver": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Pythagoras Check": ("pyth", [("a", "3"), ("b", "4")]),
        "Mean/Median/SD": ("stats", [("Data", "10,20,20,30,40")]),
        "HCF & LCM": ("hcf_lcm", [("A", "48"), ("B", "36")]),
        "Trig Heights": ("trig", [("Angle", "30"), ("Dist", "10")]),
        "3D Mensuration": ("mensuration", [("Shape (Sphere/Cyl)", "Sphere"), ("r", "7"), ("h", "10")]),
        "Interest (SI/CI)": ("interest", [("P", "1000"), ("R", "5"), ("T", "2")]),
    }

    @staticmethod
    def identity(a, b):
        return {"(a+b)²": a**2+b**2+2*a*b, "(a-b)²": a**2+b**2-2*a*b}

    @staticmethod
    def quad(a, b, c):
        d = b**2-4*a*c
        if d < 0: return {"Roots": "Complex"}
        return {"x1": _r((-b+math.sqrt(d))/(2*a)), "x2": _r((-b-math.sqrt(d))/(2*a))}

    @staticmethod
    def pyth(a, b):
        return {"c": _r(math.sqrt(a**2+b**2), 2)}

    @staticmethod
    def stats(s):
        v = [float(x) for x in s.split(",")]
        m = sum(v)/len(v)
        sd = math.sqrt(sum((x-m)**2 for x in v)/len(v))
        return {"Mean": _r(m, 2), "SD": _r(sd, 2)}

    @staticmethod
    def hcf_lcm(a, b):
        g = math.gcd(a, b)
        return {"HCF": g, "LCM": (a*b)//g}

    @staticmethod
    def trig(a, d):
        r = math.radians(a)
        return {"Height": _r(d*math.tan(r), 2)}

    @staticmethod
    def mensuration(s, r, h):
        s = s.lower()
        if "sphere" in s: return {"Vol": _r(4/3*math.pi*r**3, 2), "Area": _r(4*math.pi*r**2, 2)}
        return {"Vol": _r(math.pi*r**2*h, 2), "Area": _r(2*math.pi*r*(r+h), 2)}

    @staticmethod
    def interest(p, r, t):
        si = p*r*t/100
        ci = p*(1+r/100)**t - p
        return {"SI": si, "CI": _r(ci, 2)}

class Maths_Classes_11_12:
    TITLE = "Senior Math: Advanced Calculus & 3D"
    EXP_DATA = {
        "Matrix Det/Inv": ("matrix", [("Mat (a,b;c,d)", "1,2;3,4")]),
        "Vector Project": ("vector", [("A", "1,2,3"), ("B", "4,5,6")]),
        "AP/GP Sum": ("progression", [("a", "1"), ("d_r", "2"), ("n", "10"), ("Type", "AP")]),
        "Prob (Bernoulli)": ("bernoulli", [("n", "10"), ("p", "0.5"), ("k", "5")]),
        "Binomial nCr": ("binom", [("n", "5"), ("r", "2")]),
        "Limit (x^n-a^n)": ("limit", [("n", "3"), ("a", "2")]),
        "De Morgan's": ("sets", [("Set A", "1,2,3"), ("Set B", "3,4,5"), ("Univ", "1,2,3,4,5,6")]),
        "3D Angle Planes": ("planes", [("P1 (a,b,c)", "1,2,3"), ("P2", "2,1,-1")]),
        "Complex Ops": ("complex", [("z1 (a,b)", "1,2"), ("z2 (a,b)", "3,4")]),
    }

    @staticmethod
    def matrix(m):
        r = [[float(x) for x in row.split(",")] for row in m.split(";")]
        det = r[0][0]*r[1][1] - r[0][1]*r[1][0]
        return {"Det": det, "Inv": [[r[1][1]/det, -r[0][1]/det], [-r[1][0]/det, r[0][0]/det]] if det else "None"}

    @staticmethod
    def vector(as_, bs):
        a = [float(x) for x in as_.split(",")]
        b = [float(x) for x in bs.split(",")]
        dot = sum(x*y for x,y in zip(a,b))
        mag_b = math.sqrt(sum(x**2 for x in b))
        return {"Projection": _r(dot/mag_b, 4)}

    @staticmethod
    def progression(a, dr, n, t):
        if "ap" in t.lower(): return {"nth": a+(n-1)*dr, "sum": n/2*(2*a+(n-1)*dr)}
        return {"nth": a*(dr**(n-1)), "sum": a*(dr**n-1)/(dr-1) if dr!=1 else a*n}

    @staticmethod
    def bernoulli(n, p, k):
        res = math.comb(int(n), int(k)) * (p**k) * ((1-p)**(n-k))
        return {"P(X=k)": _r(res, 6)}

    @staticmethod
    def binom(n, r):
        return {"nCr": math.comb(n, r)}

    @staticmethod
    def limit(n, a):
        return {"Value": n * (a**(n-1))}

    @staticmethod
    def sets(as_, bs, us):
        a = set(as_.split(",")); b = set(bs_.split(",")); u = set(us.split(","))
        return {"Verified": (u-(a|b)) == ((u-a)&(u-b))}

    @staticmethod
    def planes(p1, p2):
        n1 = [float(x) for x in p1.split(",")]; n2 = [float(x) for x in p2.split(",")]
        dot = sum(x*y for x,y in zip(n1,n2))
        m1 = math.sqrt(sum(x**2 for x in n1)); m2 = math.sqrt(sum(x**2 for x in n2))
        return {"Angle (deg)": _r(math.degrees(math.acos(abs(dot)/(m1*m2))), 2)}

    @staticmethod
    def complex(z1, z2):
        a, b = [float(x) for x in z1.split(",")]; c, d = [float(x) for x in z2.split(",")]
        return {"Add": f"{a+c}+{b+d}i", "Mul": f"{a*c-b*d}+{a*d+b*c}i"}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
