"""
SigmaOS NCERT Chemistry Lab v6.0 — The Ultimate Series
Classes 6–12 | Every Core NCERT Experiment & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Secondary Chemistry: Atoms, Acids, Bases & Periodic logic"
    EXP_DATA = {
        "pH & Indicators": ("ph_logic", [("H+ Conc (M)", "0.001")]),
        "Atomic Structure": ("atom_build", [("Protons", "6"), ("Mass Number", "12")]),
        "Mole Calculator": ("moles", [("Given Mass (g)", "44"), ("Molar Mass (g/mol)", "44")]),
        "Metal Reactivity": ("displacement", [("Metal (Fe/Cu/Zn/Mg)", "Fe"), ("Solution (CuSO4/ZnSO4/HCl)", "CuSO4")]),
        "Neutralization": ("neutral", [("Acid Molarity", "0.1"), ("Acid Vol (ml)", "20"), ("Base Molarity", "0.1")]),
        "Balancing (H2+O2)": ("balance", [("H2", "2"), ("O2", "1"), ("H2O", "2")]),
        "Carbon Bonds": ("carbon", [("C count", "2"), ("Double Bonds", "0")]),
        "Gas Evolution": ("gas_test", [("Gas (CO2/H2)", "CO2")]),
    }

    @staticmethod
    def ph_logic(h):
        h = float(h)
        ph = -math.log10(h)
        return {"pH": _r(ph, 2), "Litmus": "Red" if ph < 7 else "Blue", "Nature": "Acidic" if ph < 7 else "Basic"}

    @staticmethod
    def atom_build(p, a):
        p, a = float(p), float(a)
        return {"Protons": int(p), "Electrons": int(p), "Neutrons": int(a-p)}

    @staticmethod
    def moles(m, mm):
        m, mm = float(m), float(mm)
        return {"Amt (mol)": _r(m/mm, 4)}

    @staticmethod
    def displacement(m, s):
        series = ["potassium", "sodium", "calcium", "magnesium", "aluminium", "zinc", "iron", "lead", "hydrogen", "copper", "mercury", "silver", "gold"]
        m = m.lower()
        if m in series:
            return {"Note": f"Reactivity Rank: {series.index(m)+1} of 13", "Observation": "Will displace if more reactive than metal in " + s}
        return {"Error": "Metal not in list"}

    @staticmethod
    def neutral(ma, va, mb):
        ma, va, mb = float(ma), float(va), float(mb)
        # MaVa = MbVb
        vb = (ma * va) / mb
        return {"Base Vol required (ml)": _r(vb, 2)}

    @staticmethod
    def balance(h2, o2, h2o):
        h2, o2, h2o = float(h2), float(o2), float(h2o)
        if h2 == 2 and o2 == 1 and h2o == 2: return {"Status": "BALANCED"}
        return {"Status": "UNBALANCED", "Goal": "2H2 + O2 -> 2H2O"}

    @staticmethod
    def carbon(c, db):
        c, db = float(c), float(db)
        if db == 0: return {"Type": "Alkane", "Formula": f"C{int(c)}H{int(2*c+2)}"}
        return {"Type": "Alkene", "Formula": f"C{int(c)}H{int(2*c)}"}

    @staticmethod
    def gas_test(g):
        g = g.lower()
        if "co2" in g: return {"Test": "Passes through Lime water", "Result": "Turns Milky"}
        if "h2" in g: return {"Test": "Burn with splinter", "Result": "Pop sound"}
        return {"Info": "Refer Class 10 Ch-1"}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Solutions, Kinetics, Thermo & Coordination"
    EXP_DATA = {
        "Ideal Gas State": ("gas_law", [("P (atm)", "1"), ("V (L)", "22.4"), ("Moles", "1")]),
        "Thermodynamics": ("gibbs", [("dH (kJ)", "-50"), ("dS (J/K)", "50"), ("Temp (K)", "298")]),
        "Rate Constant": ("kinetic", [("k", "0.005"), ("Initial [A]0", "1.0"), ("Order (1/0)", "1")]),
        "Nernst Potential": ("nernst", [("E0 Cell (V)", "1.1"), ("n (electrons)", "2"), ("Q (Quotient)", "0.01")]),
        "Faraday's Yield": ("faraday", [("Current (A)", "2"), ("Time (s)", "965"), ("Eq Weight", "31.75")]),
        "Molarity/Molality": ("conc", [("Solute Mass (g)", "40"), ("Molar Mass", "40"), ("Vol/Mass Solvent", "1000"), ("Mode (M/m)", "M")]),
        "EAN Coordination": ("ean", [("At. Number Z", "26"), ("Ox State", "2"), ("Coord Num", "6")]),
        "Buffer pH": ("buffer", [("pKa", "4.74"), ("Salt Conc", "0.1"), ("Acid Conc", "0.1")]),
        "Osmotic Pressure": ("osmosis", [("Molarity", "0.5"), ("Temp (K)", "298")]),
    }

    @staticmethod
    def gas_law(p, v, n):
        p, v, n = float(p), float(v), float(n)
        r = 0.0821
        return {"Temp (K)": _r((p*v)/(n*r), 2)}

    @staticmethod
    def gibbs(dh, ds, t):
        dh, ds, t = float(dh), float(ds), float(t)
        dg = dh - (t * ds / 1000)
        return {"dG (kJ)": _r(dg, 2), "Spontaneous": "YES" if dg < 0 else "NO"}

    @staticmethod
    def kinetic(k, a0, order):
        k, a0, order = float(k), float(a0), float(order)
        if order == 1: return {"Half-life (s)": _r(0.693/k, 2)}
        return {"Half-life (s)": _r(a0 / (2*k), 2)}

    @staticmethod
    def nernst(e0, n, q):
        e0, n, q = float(e0), float(n), float(q)
        e = e0 - (0.0591/n) * math.log10(q)
        return {"E Cell (V)": _r(e, 4)}

    @staticmethod
    def faraday(i, t, w_eq):
        i, t, w_eq = float(i), float(t), float(w_eq)
        w = (i * t * w_eq) / 96500
        return {"Mass Yield (g)": _r(w, 5)}

    @staticmethod
    def conc(m, mm, v, mode):
        m, mm, v = float(m), float(mm), float(v)
        moles = m / mm
        if mode.upper() == "M": return {"Molarity (M)": _r(moles / (v/1000), 3)}
        return {"Molality (m)": _r(moles / (v/1000), 3)}

    @staticmethod
    def ean(z, ox, cn):
        z, ox, cn = float(z), float(ox), float(cn)
        res = z - ox + (2 * cn)
        return {"EAN": int(res)}

    @staticmethod
    def buffer(pka, salt, acid):
        pka, salt, acid = float(pka), float(salt), float(acid)
        # pH = pKa + log(S/A)
        ph = pka + math.log10(salt/acid)
        return {"pH": _r(ph, 3)}

    @staticmethod
    def osmosis(m, t):
        m, t = float(m), float(t)
        r = 0.0821
        pi = m * r * t
        return {"Pressure (atm)": _r(pi, 3)}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
