"""
SigmaOS NCERT Chemistry Lab v4.0
Classes 6–12 | Every NCERT Chemistry experiment & calculation
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Classes 6–10 – Matter, Atoms, pH & Reactions"
    EXP_DATA = {
        "pH Calculator": ("calc_ph", [("H+ Concentration (M)", "0.001")]),
        "Atomic Data": ("atom_data", [("Electrons/Z", "11"), ("Atomic Mass", "23")]),
        "Mole Calculator": ("moles", [("Given Mass (g)", "44"), ("Molar Mass (g/mol)", "44")]),
        "Titration": ("titration", [("M1 (Acid)", "0.1"), ("V1 (Acid)", "20"), ("V2 (Base)", "25")]),
        "Reactivity Series": ("reactivity", [("Metal 1", "Iron"), ("Metal 2", "Copper")]),
        "Separation Tech": ("separation", [("Mixture", "Salt and Water")]),
    }

    @staticmethod
    def calc_ph(h):
        h = float(h)
        p = -math.log10(h)
        return {"pH": _r(p, 2), "Nature": "Acidic" if p < 7 else ("Basic" if p > 7 else "Neutral")}

    @staticmethod
    def atom_data(z, mass):
        z, mass = int(z), int(mass)
        return {"Protons": z, "Neutrons": mass - z, "Electrons": z}

    @staticmethod
    def moles(m, mm):
        m, mm = float(m), float(mm)
        return {"Amount (mol)": _r(m/mm, 4)}

    @staticmethod
    def titration(m1, v1, v2):
        m1, v1, v2 = float(m1), float(v1), float(v2)
        m2 = (m1 * v1) / v2
        return {"Molarity M2": _r(m2, 4)}

    @staticmethod
    def reactivity(m1, m2):
        series = ["Potassium", "Sodium", "Calcium", "Magnesium", "Aluminium", "Zinc", "Iron", "Lead", "Hydrogen", "Copper", "Mercury", "Silver", "Gold"]
        try:
            i1 = series.index(m1.capitalize())
            i2 = series.index(m2.capitalize())
            return {"More Reactive": m1 if i1 < i2 else m2, "Displacement": "Yes" if i1 < i2 else "No"}
        except: return {"Error": "Metal not in series"}

    @staticmethod
    def separation(mix):
        data = {
            "salt and water": "Evaporation / Distillation",
            "oil and water": "Separating Funnel",
            "iron and sand": "Magnetic Separation",
            "chalk and water": "Filtration",
            "dye": "Chromatography"
        }
        return {"Method": data.get(mix.lower(), "Refer NCERT Science Class 9 Ch-2")}

class Chemistry_Classes_11_12:
    TITLE = "Classes 11–12 – Physical, Organic, Inorganic & Analytical"
    EXP_DATA = {
        "Ideal Gas Law": ("ideal_gas", [("P (atm)", "1"), ("V (L)", "22.4"), ("n", "1")]),
        "Packing Efficiency": ("packing", [("Type (SCC/BCC/FCC)", "FCC")]),
        "EAN Rule": ("ean", [("Atomic Num Z", "26"), ("Oxidation State", "2"), ("Coordination Num", "6")]),
        "Buffer pH": ("buffer", [("pKa/pKb", "4.74"), ("Salt Conc", "0.1"), ("Acid/Base Conc", "0.1")]),
        "Nernst Equation": ("nernst", [("E0 Cell (V)", "1.1"), ("n", "2"), ("Q (Quotient)", "0.01")]),
        "Rate Constant (1st Order)": ("rate_1st", [("Initial [A]0", "1.0"), ("Final [A]t", "0.5"), ("Time (s)", "600")]),
        "Faraday's Law": ("faraday", [("Current (A)", "2"), ("Time (s)", "965"), ("Eq Weight", "31.75")]),
        "Colloid Tyndall": ("tyndall", [("Solution Type", "Milk")]),
    }

    @staticmethod
    def ideal_gas(p, v, n):
        p, v, n = float(p), float(v), float(n)
        r = 0.0821
        t = (p * v) / (n * r)
        return {"Temp (K)": _r(t, 2)}

    @staticmethod
    def packing(style):
        data = {"scc": "52.4%", "bcc": "68%", "fcc": "74%"}
        return {"Efficiency": data.get(style.lower(), "Unknown")}

    @staticmethod
    def ean(z, ox, coord):
        z, ox, coord = int(z), int(ox), int(coord)
        # EAN = Z - Ox + 2*Coord
        res = z - ox + 2*coord
        return {"EAN": res, "Note": "Compare with nearest Noble Gas (36, 54, 86)"}

    @staticmethod
    def buffer(pka, salt, acid):
        pka, salt, acid = float(pka), float(salt), float(acid)
        # pH = pKa + log([Salt]/[Acid])
        ph = pka + math.log10(salt / acid)
        return {"pH": _r(ph, 3)}

    @staticmethod
    def nernst(e0, n, q):
        e0, n, q = float(e0), float(n), float(q)
        e = e0 - (0.0591 / n) * math.log10(q)
        return {"Cell Potential E (V)": _r(e, 4)}

    @staticmethod
    def rate_1st(a0, at, t):
        a0, at, t = float(a0), float(at), float(t)
        # k = (2.303/t) * log(a0/at)
        k = (2.303 / t) * math.log10(a0 / at)
        return {"Rate Constant k": f"{k:.4e}"}

    @staticmethod
    def faraday(i, t, w_eq):
        i, t, w_eq = float(i), float(t), float(w_eq)
        f = 96500
        w = (w_eq / f) * i * t
        return {"Mass (g)": _r(w, 5)}

    @staticmethod
    def tyndall(sol):
        colloids = ["milk", "smoke", "fog", "solution of soap", "starch solution"]
        if sol.lower() in colloids:
            return {"Tyndall Effect": "Visible", "Nature": "Colloidal"}
        return {"Tyndall Effect": "Not Visible", "Nature": "True Solution or Suspension"}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
