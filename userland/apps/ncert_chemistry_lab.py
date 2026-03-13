"""
SigmaOS NCERT Chemistry Lab v5.0 — The Complete Series
Classes 6–12 | Comprehensive Chemical Lab simulations
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Classes 6–10: Matter, Atoms, Acids & Organic Basics"
    EXP_DATA = {
        "pH & Indicators": ("ph_logic", [("H+ Conc (M)", "0.001")]),
        "Atomic Structure": ("atom_build", [("Protons", "6"), ("Mass Number", "12")]),
        "Chemical Formula": ("formula", [("Valency 1", "1"), ("Valency 2", "2")]),
        "Balanced Eq Check": ("balance", [("Reactant H2", "2"), ("Reactant O2", "1"), ("Product H2O", "2")]),
        "Metals Reactivity": ("displacement", [("Fe into CuSO4 (1/0)", "1")]),
        "Brownian Motion": ("particles", [("Temperature (C)", "25")]),
        "Hydrocarbon Type": ("carbon", [("C count", "2"), ("Double Bond (1/0)", "1")]),
    }

    @staticmethod
    def ph_logic(h):
        ph = -math.log10(h)
        return {"pH": _r(ph, 2), "Litmus": "Red" if ph < 7 else "Blue", "Type": "Acid" if ph < 7 else "Base"}

    @staticmethod
    def atom_build(p, a):
        return {"Electrons": p, "Neutrons": a-p, "Element": "Check Periodic Table Z="+str(int(p))}

    @staticmethod
    def formula(v1, v2):
        return {"Ratio": f"A{int(v2)}B{int(v1)}", "Rule": "Criss-cross valencies"}

    @staticmethod
    def balance(h2, o2, h2o):
        # 2H2 + O2 = 2H2O
        if h2/2 == o2/1 == h2o/2: return {"Status": "BALANCED"}
        return {"Status": "UNBALANCED", "Correct Ratio": "2:1:2"}

    @staticmethod
    def displacement(react):
        if int(react): return {"Result": "Blue solution turns Green", "Solid": "Brown Red Copper deposited"}
        return {"Result": "No color change"}

    @staticmethod
    def particles(t):
        speed = math.sqrt(t + 273) * 1.5
        return {"Relative Motion": _r(speed, 2), "Observation": "Particles move faster as Temp rises"}

    @staticmethod
    def carbon(c, db):
        if db: return {"Name": "Alkene (e.g. Ethene)", "Formula": f"C{int(c)}H{int(2*c)}"}
        return {"Name": "Alkane (e.g. Ethane)", "Formula": f"C{int(c)}H{int(2*c+2)}"}

class Chemistry_Classes_11_12:
    TITLE = "Classes 11–12: Thermodynamics, Kinetics, Solutions & Coordination"
    EXP_DATA = {
        "Ideal Gas State": ("gas_law", [("P (atm)", "1"), ("V (L)", "22.4"), ("Moles", "1")]),
        "Thermodynamics (Gibbs)": ("gibbs", [("dH (kJ)", "-50"), ("dS (J/K)", "50"), ("Temp (K)", "298")]),
        "Kinetics (Half-life)": ("kinetic", [("k rate", "0.005"), ("Initial [A]", "1.0")]),
        "Osmotic Pressure": ("osmosis", [("Molarity", "0.5"), ("Temp (K)", "298")]),
        "Nernst Cell": ("nernst", [("E0 (V)", "1.1"), ("n elec", "2"), ("[Prod]/[React]", "0.1")]),
        "Faraday Yield": ("faraday", [("Current (A)", "2"), ("Time (s)", "965"), ("Eq Wt", "31.75")]),
        "EAN Coordination": ("ean", [("Z", "26"), ("Ox State", "3")]),
        "Ebullioscopy": ("boil", [("Kb", "0.52"), ("Molality", "1")]),
    }

    @staticmethod
    def gas_law(p, v, n):
        r = 0.0821
        temp = (p*v)/(n*r)
        return {"Temp (K)": _r(temp, 2)}

    @staticmethod
    def gibbs(dh, ds, t):
        dg = dh - (t * ds / 1000)
        return {"dG (kJ)": _r(dg, 2), "Spontaneous": "YES" if dg < 0 else "NO"}

    @staticmethod
    def kinetic(k, a0):
        t12 = 0.693/k
        return {"Half-life (s/min)": _r(t12, 2)}

    @staticmethod
    def osmosis(m, t):
        r = 0.0821
        pi = m * r * t
        return {"Pressure (atm)": _r(pi, 3)}

    @staticmethod
    def nernst(e0, n, q):
        e = e0 - (0.0591/n) * math.log10(q)
        return {"E Cell (V)": _r(e, 4)}

    @staticmethod
    def faraday(i, t, w_eq):
        w = (i*t*w_eq)/96500
        return {"Mass deposited (g)": _r(w, 5)}

    @staticmethod
    def ean(z, ox):
        # EAN = Z - Ox + 2*Coord (assumed 6)
        res = z - ox + 12
        return {"EAN": res}

    @staticmethod
    def boil(kb, m):
        return {"Elevation dTb (K)": _r(kb*m, 3)}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
