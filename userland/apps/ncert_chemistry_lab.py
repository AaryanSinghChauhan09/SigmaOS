"""
SigmaOS NCERT Chemistry Lab v10.0 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Secondary Chemistry: Advanced Micro-labs"
    EXP_DATA = {
        "Litmus Interaction": ("litmus", [("Solution", "Lemon")]),
        "Separation Logic": ("separation", [("Mixture", "Sand+Salt")]),
        "Atom Config": ("atom", [("Z", "11")]),
        "pH Analysis": ("ph", [("H+ Conc (M)", "0.001")]),
        "Gas Evolution": ("gas", [("Test", "Lime Water")]),
        "Mole Logic": ("mole", [("Mass (g)", "44"), ("Molar Mass", "44")]),
        "Reaction Equilibrium": ("equilibrium", [("Temp (C)", "25"), ("Pressure (atm)", "1")]),
        "Solution Molarity": ("molarity", [("Solute (g)", "4"), ("Molar Mass", "40"), ("Volume (ml)", "250")]),
    }

    @staticmethod
    def litmus(s):
        s = s.lower()
        if any(x in s for x in ["lemon", "acid", "vinegar"]): return {"Paper": "RED", "Nature": "Acid"}
        if any(x in s for x in ["soap", "base", "lime"]): return {"Paper": "BLUE", "Nature": "Base"}
        return {"Paper": "NO CHANGE", "Nature": "Neutral"}

    @staticmethod
    def separation(m):
        m = m.lower()
        if "sand" in m and "salt" in m: return {"Flow": "Filter -> Evaporate"}
        if "oil" in m and "water" in m: return {"Flow": "Separating Funnel"}
        return {"Flow": "Refer NCERT Ch-2"}

    @staticmethod
    def atom(z):
        z = int(z)
        cfg = "2," + str(min(8, z-2)) if z > 2 else str(z)
        if z > 10: cfg += "," + str(z-10)
        return {"Symbol": "Na" if z==11 else "H" if z==1 else "?", "Config": cfg}

    @staticmethod
    def ph(h):
        p = -math.log10(h)
        return {"pH": _r(p, 2), "Inference": "Strong Acid" if p < 3 else "Neutral" if 6.5<p<7.5 else "Base"}

    @staticmethod
    def gas(t):
        t = t.lower()
        if "lime" in t: return {"Gas": "CO2", "Obs": "Milky"}
        if "pop" in t: return {"Gas": "H2", "Obs": "Pop sound"}
        return {"Gas": "O2", "Obs": "Rekindle splinter"}

    @staticmethod
    def mole(m, mm):
        return {"Moles": _r(m/mm, 3), "Count": f"{_r((m/mm)*6.022e23, 2):.2e}"}

    @staticmethod
    def equilibrium(t, p):
        return {"Shift": "Le Chatelier predicts Forward if Exothermic/Low Temp"}

    @staticmethod
    def molarity(m, mm, v):
        mol = m / mm
        res = mol / (v/1000)
        return {"Molarity (M)": _r(res, 3)}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Exhaustive Lab Manual"
    EXP_DATA = {
        "Redox Titration": ("redox", [("M_permanganate", "0.02"), ("V_fas", "20"), ("V_perm", "20")]),
        "Kinetics (Order)": ("kinetics", [("Initial [A]", "1.0"), ("k_rate", "0.005"), ("Time (s)", "100")]),
        "Buffer pH": ("buffer", [("pKa", "4.74"), ("[Salt]", "0.1"), ("[Acid]", "0.1")]),
        "Nernst Potential": ("nernst", [("E0_Cell", "1.1"), ("n", "2"), ("Q_Quotient", "0.01")]),
        "Chromatography": ("chromato", [("Dist_Spot", "6"), ("Dist_Solvent", "10")]),
        "Functional Tester": ("functional", [("Reagent", "Tollen's")]),
        "Hardness of Water": ("hardness", [("EDTA_ml", "15"), ("Sample_ml", "50")]),
        "Osmotic Pressure": ("osmosis", [("Molarity", "0.1"), ("Temp (C)", "27"), ("i_factor", "1")]),
        "Rate Law (Thio+HCl)": ("thio_rate", [("Conc_Thiosulphate", "0.1"), ("Temp (C)", "25")]),
        "Enthalpy Change": ("enthalpy", [("dT (C)", "5"), ("Mass_Water (g)", "100")]),
    }

    @staticmethod
    def redox(mp, vf, vp):
        mf = (5 * mp * vp) / vf
        return {"Molarity FAS": _r(mf, 4), "Strength (g/L)": _r(mf * 392, 2)}

    @staticmethod
    def kinetics(a0, k, t):
        at = a0 * math.exp(-k * t)
        return {"Final [A]t": _r(at, 4), "Conversion %": _r((1-at/a0)*100, 2)}

    @staticmethod
    def buffer(pka, s, a):
        ph = pka + math.log10(s/a)
        return {"Buffer pH": _r(ph, 2)}

    @staticmethod
    def nernst(e0, n, q):
        e = e0 - (0.0591/n)*math.log10(q)
        return {"E_Cell (V)": _r(e, 4)}

    @staticmethod
    def chromato(ds, dr):
        return {"Rf Value": _r(ds/dr, 3)}

    @staticmethod
    def functional(r):
        r = r.lower()
        if "tollen" in r: return {"Result": "Silver Mirror", "Group": "Aldehyde"}
        if "fecl3" in r: return {"Result": "Violet", "Group": "Phenol"}
        return {"Result": "No specific match"}

    @staticmethod
    def hardness(edta, s):
        h = (edta * 1000) / s
        return {"Hardness (ppm)": _r(h, 2)}

    @staticmethod
    def osmosis(m, t_c, i):
        t = t_c + 273.15
        pi = i * m * 0.0821 * t
        return {"Osmotic P (atm)": _r(pi, 2)}

    @staticmethod
    def thio_rate(c, t):
        # r = k[c] exp(-Ea/RT)
        rate = c * 0.1 * (t/10) # simplified
        return {"Rate (1/s)": _r(rate, 4)}

    @staticmethod
    def enthalpy(dt, m):
        q = m * 4.184 * dt
        return {"Heat (J)": _r(q, 1)}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
