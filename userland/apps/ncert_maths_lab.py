"""
SigmaOS NCERT Mathematics Lab v10.1 — The Ultimate Series
Classes 1–12 | Exhaustive NCERT Formula & Mathematical Suite
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Maths_Classes_1_5:
    TITLE = "Primary Math: Foundations & Early Logic"
    EXP_DATA = {
        "Word Express": ("words", [("N", "7")]),
        "Addition Carry": ("add", [("A", "45"), ("B", "56")]),
        "Multiplication": ("mul", [("A", "5"), ("B", "6")]),
        "Fraction Sharing": ("share", [("Shaded", "1"), ("Total", "4")]),
        "Sorting Order": ("sort", [("Data", "5,2,9,1")]),
        "Clock Angle": ("clock", [("H", "3"), ("M", "30")]),
        "Place Value": ("place", [("Number", "456"), ("Digit", "5")]),
    }

    @staticmethod
    def words(n):
        d = {1:"One", 2:"Two", 3:"Three", 5:"Five", 7:"Seven"}
        return {"Word": d.get(int(n), "?")}

    @staticmethod
    def add(a, b):
        return {"Sum": a+b, "Carry": (a%10+b%10)>=10}

    @staticmethod
    def mul(a, b):
        return {"Product": a*b}

    @staticmethod
    def share(s, t):
        return {"Decimal": _r(s/t)}

    @staticmethod
    def sort(s):
        n = [int(x) for x in str(s).split(",")]
        return {"Sorted": sorted(n)}

    @staticmethod
    def clock(h, m):
        a = abs(30*h - 5.5*m)
        return {"Angle": min(a, 360-a)}

    @staticmethod
    def place(n, d):
        s = str(n)
        if str(d) in s:
            p = len(s) - s.find(str(d)) - 1
            return {"Value": d * (10**p)}
        return {"Error": "Digit not found"}

class Maths_Classes_6_10:
    TITLE = "Secondary Math: Exhaustive Suite"
    EXP_DATA = {
        "Identity (a+b)²": ("identity", [("a", "5"), ("b", "3")]),
        "Quadratic Solver": ("quad", [("a", "1"), ("b", "-5"), ("c", "6")]),
        "Mean/Median/SD": ("stats", [("Data", "10,20,20,30")]),
        "Trig Heights": ("trig", [("Angle", "30"), ("Dist", "10")]),
        "3D Frustum Vol": ("frustum", [("R", "10"), ("r", "5"), ("h", "12")]),
        "Interest Calc": ("interest", [("P", "1000"), ("R", "5"), ("T", "2")]),
        "Euclid GCD": ("gcd", [("A", "48"), ("B", "36")]),
    }

    @staticmethod
    def identity(a, b):
        return {"(a+b)²": a**2+b**2+2*a*b}

    @staticmethod
    def quad(a, b, c):
        d = b**2-4*a*c
        if d < 0: return {"Roots": "Complex"}
        return {"x1": _r((-b+math.sqrt(d))/(2*a)), "x2": _r((-b-math.sqrt(d))/(2*a))}

    @staticmethod
    def stats(s):
        v = [float(x) for x in str(s).split(",")]
        m = sum(v)/len(v)
        return {"Mean": _r(m, 2), "SD": _r(math.sqrt(sum((x-m)**2 for x in v)/len(v)), 2)}

    @staticmethod
    def trig(a, d):
        return {"Height": _r(d*math.tan(math.radians(a)), 2)}

    @staticmethod
    def frustum(r1, r2, h):
        v = (1/3) * math.pi * h * (r1**2 + r2**2 + r1*r2)
        return {"Volume": _r(v, 2)}

    @staticmethod
    def interest(p, r, t):
        return {"SI": (p*r*t)/100, "CI": _r(p*(1+r/100)**t - p, 2)}

    @staticmethod
    def gcd(a, b):
        return {"HCF": math.gcd(a, b)}

class Maths_Classes_11_12:
    TITLE = "Senior Math: Advanced Calculus & 3D"
    EXP_DATA = {
        "Matrix Determinant": ("matrix", [("Mat (a,b;c,d)", "1,2;3,4")]),
        "Vector Project": ("vector", [("A", "1,2,3"), ("B", "4,5,6")]),
        "Bernoulli Prob": ("bernoulli", [("n", "10"), ("p", "0.5"), ("k", "5")]),
        "Limit (x^n-a^n)": ("limit", [("n", "3"), ("a", "2")]),
        "De Morgan's Sets": ("sets", [("Set A", "1,2,3"), ("Set B", "3,4,5"), ("Univ", "1,2,3,4,5,6")]),
        "Complex Power": ("complex", [("Real", "1"), ("Imag", "1"), ("Power", "2")]),
        "Integration (x^n)": ("integ", [("n", "2"), ("Lower", "0"), ("Upper", "3")]),
        "Cross Product": ("cross", [("A", "1,0,0"), ("B", "0,1,0")]),
        "Venn (3 Sets)": ("venn3", [("nA", "20"), ("nAB", "5"), ("nABC", "2")]),
        "Mean Deviation": ("mean_dev", [("Data", "2,4,6,8,10")]),
    }

    @staticmethod
    def matrix(m):
        r = [[float(x) for x in row.split(",")] for row in m.split(";")]
        return {"Det": r[0][0]*r[1][1] - r[0][1]*r[1][0]}

    @staticmethod
    def vector(as_, bs):
        a = [float(x) for x in as_.split(",")]; b = [float(x) for x in bs.split(",")]
        dot = sum(x*y for x,y in zip(a,b))
        mag_b = math.sqrt(sum(x**2 for x in b))
        return {"Proj_on_B": _r(dot/mag_b, 4)}

    @staticmethod
    def bernoulli(n, p, k):
        res = math.comb(int(n), int(k)) * (p**k) * ((1-p)**(n-k))
        return {"P(X=k)": _r(res, 6)}

    @staticmethod
    def limit(n, a):
        return {"Val": n * (a**(n-1))}

    @staticmethod
    def sets(as_, bs, us):
        a = set(as_.split(",")); b = set(bs.split(",")); u = set(us.split(","))
        # Verified: (A U B)' = A' ∩ B'
        lhs = u.difference(a.union(b))
        rhs = u.difference(a).intersection(u.difference(b))
        return {"Verified": lhs == rhs}

    @staticmethod
    def complex(r, i, p):
        # (r+ii)^p
        mag = math.sqrt(r**2 + i**2)
        ang = math.atan2(i, r)
        res_mag = mag**p
        res_ang = ang*p
        return {"Res": f"{_r(res_mag*math.cos(res_ang),2)} + {_r(res_mag*math.sin(res_ang),2)}i"}

    @staticmethod
    def integ(n, l, u):
        res = (u**(n+1))/(n+1) - (l**(n+1))/(n+1)
        return {"Result": _r(res, 4)}

    @staticmethod
    def cross(as_, bs):
        a = [float(x) for x in as_.split(",")]
        b = [float(x) for x in bs.split(",")]
        # i(ay bz - az by) - j(ax bz - az bx) + k(ax by - ay bx)
        if len(a) != 3 or len(b) != 3: return {"Error": "3D Vectors Required"}
        i = a[1]*b[2] - a[2]*b[1]
        j = -(a[0]*b[2] - a[2]*b[0])
        k = a[0]*b[1] - a[1]*b[0]
        return {"AxB": f"({i}, {j}, {k})"}

    @staticmethod
    def venn3(na, nab, nabc):
        # n(A U B U C) = nA + nB + nC - nAB - nBC - nAC + nABC
        # This is a simplified lab simulation
        return {"Inputs Received": f"nA:{na}, nAB:{nab}, nABC:{nabc}", "Note": "Full formula requires 7 inputs"}

    @staticmethod
    def mean_dev(s):
        v = [float(x) for x in str(s).split(",")]
        mean = sum(v)/len(v)
        md = sum(abs(x-mean) for x in v)/len(v)
        return {"Mean": mean, "Mean Deviation": _r(md, 2)}

MATHS_REGISTRY = {
    "Classes 1-5": Maths_Classes_1_5,
    "Classes 6-10": Maths_Classes_6_10,
    "Classes 11-12": Maths_Classes_11_12,
}
