"""
SigmaOS NCERT Chemistry Lab v10.0 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, random, re

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Secondary Chemistry: Advanced Micro-labs"
    EXP_DATA = {
        "Atom ID (Z)": ("atom", [("Z", "11")]),
        "pH Tester": ("ph", [("Conc (M)", "0.001"), ("Is Acid (1/0)", "1")]),
        "Molar Mass": ("molar_mass", [("Formula", "H2SO4")]),
        "Titration (V1M1=V2M2)": ("titration", [("M1", "0.1"), ("V1", "20"), ("V2", "22.5")]),
        "Gas Law (Ideal)": ("gas", [("P (atm)", "1"), ("V (L)", "22.4")]),
        "Stoichiometry": ("stoich", [("Reactant Mass", "10"), ("Molar Mass R", "40"), ("Molar Mass P", "60")]),
        "Separation flow": ("separation", [("Mixture", "Oil+Water")]),
        "Salt Indicators": ("indicator", [("Sol", "Lemon"), ("Ind (Litmus/Phenol)", "Litmus")]),
    }

    ELEMENTS = {"H":1.008, "He":4.003, "Li":6.941, "C":12.01, "N":14.01, "O":16.0, "Na":22.99, "S":32.06, "Cl":35.45, "K":39.1, "Ca":40.1, "Fe":55.8}

    @staticmethod
    def atom(z):
        return {"Symbol": "Na" if z==11 else "H" if z==1 else "?", "Z":z}

    @staticmethod
    def ph(c, isa):
        p = -math.log10(c) if int(isa) else 14 + math.log10(c)
        return {"pH": _r(p, 2), "Nature": "Acid" if p < 7 else "Base"}

    @staticmethod
    def molar_mass(f):
        mats = re.findall(r'([A-Z][a-z]?)(\d*)', f)
        total = 0
        for s, c in mats:
            n = int(c) if c else 1
            total += Chemistry_Classes_6_10.ELEMENTS.get(s, 0) * n
        return {"Molar Mass": _r(total, 3)}

    @staticmethod
    def titration(m1, v1, v2):
        return {"M2": _r(m1*v1/v2, 4)}

    @staticmethod
    def gas(p, v):
        # T = PV/nR (n=1)
        t = (p*v)/0.0821
        return {"Temp (K)": _r(t, 1)}

    @staticmethod
    def stoich(m, mmr, mmp):
        mol = m / mmr
        return {"Product Mass (g)": _r(mol * mmp, 2)}

    @staticmethod
    def separation(m):
        if "+" in m or "and" in m.lower(): return {"Methods": "Mechanical/Phase separation"}
        return {"Methods": "Refer Class 6 Ch-5"}

    @staticmethod
    def indicator(s, i):
        s = s.lower()
        if "litmus" in i.lower():
            return {"Color": "RED" if "lemon" in s else "BLUE" if "soap" in s else "NC"}
        return {"Color": "PINK" if "soap" in s else "Colorless"}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Exhaustive Lab Suite"
    EXP_DATA = {
        "Nernst Equation": ("nernst", [("E0 Cell", "1.1"), ("n", "2"), ("Q", "0.01")]),
        "Reaction Equilibrium": ("keq", [("Kc", "40"), ("Qc", "60")]),
        "Bond Energy": ("bond", [("B_react (kJ)", "500"), ("B_prod (kJ)", "650")]),
        "Rate (Arrhenius)": ("arrhenius", [("A", "1e11"), ("Ea (kJ)", "50"), ("T (K)", "300")]),
        "Osmosis (Pi)": ("osmosis", [("M", "0.1"), ("T (C)", "27"), ("i", "1")]),
        "Electrolysis (Yield)": ("faraday", [("I (A)", "2"), ("t (s)", "965"), ("EqWt", "31.7")]),
        "EAN Finder": ("ean", [("Z", "26"), ("OxState", "2"), ("CN", "6")]),
        "Functional Class": ("organic", [("Reagent", "Tollen's")]),
        "Solubility Product": ("ksp", [("Solubility (M)", "1e-5"), ("Type (AB/AB2)", "AB2")]),
    }

    @staticmethod
    def nernst(e0, n, q):
        e = e0 - (0.0591/n)*math.log10(q)
        return {"E_Cell (V)": _r(e, 4)}

    @staticmethod
    def keq(kc, qc):
        if qc < kc: return {"Shift": "FORWARD / Product favored"}
        if qc > kc: return {"Shift": "BACKWARD / Reactant favored"}
        return {"Shift": "EQUILIBRIUM"}

    @staticmethod
    def bond(br, bp):
        dH = br - bp
        return {"delta_H (kJ)": dH, "Type": "EXO" if dH<0 else "ENDO"}

    @staticmethod
    def arrhenius(a, ea, t):
        r = 0.008314 # kJ/mol.K
        k = a * math.exp(-ea/(r*t))
        return {"k rate": f"{k:.4e}"}

    @staticmethod
    def osmosis(m, tc, i):
        t = tc + 273.15
        pi = i * m * 0.0821 * t
        return {"Pi (atm)": _r(pi, 2)}

    @staticmethod
    def faraday(i, t, w):
        mass = (i * t * w) / 96500
        return {"Yield (g)": _r(mass, 5)}

    @staticmethod
    def ean(z, ox, cn):
        res = z - ox + 2*cn
        return {"EAN": int(res), "Stable": res in [36, 54, 86]}

    @staticmethod
    def organic(r):
        r = r.lower()
        if "tollen" in r: return {"Test": "Aldehyde", "Obs": "Silver Mirror"}
        if "fecl3" in r: return {"Test": "Phenol", "Obs": "Violet"}
        return {"Test": "Carboxyl", "Obs": "Effervescence"}

    @staticmethod
    def ksp(s, t):
        if t == "AB": k = s**2
        else: k = 4 * s**3
        return {"Ksp": f"{k:.4e}"}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
