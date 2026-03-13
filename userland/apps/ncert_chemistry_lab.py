"""
SigmaOS NCERT Chemistry Lab v9.0 — The Comprehensive series
Classes 6–12 | Exhaustive NCERT Experiment & Calculator Suite
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Secondary Chemistry: Advanced Micro-labs"
    EXP_DATA = {
        "Virtual Litmus Test": ("litmus", [("Solution", "Lemon")]),
        "Separation Simulator": ("separation", [("Mixture", "Sand+Salt")]),
        "Physical/Chemical Change": ("change", [("Activity", "Burning Wood")]),
        "Atomic Structure": ("atom", [("Z (Atomic Number)", "11")]),
        "Reactivity Series": ("reactivity", [("Metal 1", "Iron"), ("Metal 2", "Copper")]),
        "pH Calculator": ("ph", [("H+ Concentration (M)", "0.001")]),
        "Gas Identification": ("gas", [("Test", "Lime Water")]),
        "Soap Preparation": ("soap", [("Oil (g)", "50")]),
        "Hardness of Water": ("hardness", [("EDTA (ml)", "15"), ("Sample (ml)", "50")]),
        "Crystallization": ("crystal", [("Solute", "Copper Sulphate"), ("Water (ml)", "100")]),
    }

    @staticmethod
    def litmus(sol):
        sol = sol.lower()
        if any(x in sol for x in ["lemon", "vinegar", "acid", "battery"]): return {"Paper Color": "RED", "Nature": "Acidic"}
        if any(x in sol for x in ["soap", "lime", "base", "detergent"]): return {"Paper Color": "BLUE", "Nature": "Basic"}
        return {"Paper Color": "NO CHANGE", "Nature": "Neutral"}

    @staticmethod
    def separation(mix):
        m = mix.lower()
        if "sand" in m and "salt" in m: return {"Methods": "Filtration followed by Evaporation"}
        if "oil" in m and "water" in m: return {"Methods": "Separating Funnel"}
        if "salt" in m and "ammonium" in m: return {"Methods": "Sublimation"}
        return {"Methods": "Refer NCERT Class 9 Ch-2 Filtration/Centrifugation"}

    @staticmethod
    def change(act):
        act = act.lower()
        if any(x in act for x in ["burn", "rust", "cook", "curd", "digestion"]): return {"Type": "CHEMICAL CHANGE", "Note": "Irreversible"}
        return {"Type": "PHYSICAL CHANGE", "Note": "Reversible"}

    @staticmethod
    def atom(z):
        z = int(z)
        names = {1:"Hydrogen", 2:"Helium", 6:"Carbon", 7:"Nitrogen", 8:"Oxygen", 11:"Sodium", 17:"Chlorine", 20:"Calcium", 26:"Iron"}
        syms = {1:"H", 2:"He", 6:"C", 7:"N", 8:"O", 11:"Na", 17:"Cl", 20:"Ca", 26:"Fe"}
        return {"Element": names.get(z, "?"), "Symbol": syms.get(z, "?"), "Electrons": z, "Protons": z}

    @staticmethod
    def reactivity(m1, m2):
        series = ["Potassium", "Sodium", "Calcium", "Magnesium", "Aluminium", "Zinc", "Iron", "Lead", "Hydrogen", "Copper", "Mercury", "Silver", "Gold"]
        try:
            i1 = series.index(m1.capitalize())
            i2 = series.index(m2.capitalize())
            return {"More Reactive": m1 if i1 < i2 else m2, "Displacement": "Possible (m1 in m2 soln)" if i1 < i2 else "Impossible"}
        except: return {"Error": "Metal not in series"}

    @staticmethod
    def ph(h):
        h = float(h)
        p = -math.log10(h)
        return {"pH": _r(p, 2), "Nature": "Strong Acid" if p < 3 else ("Weak Acid" if p < 7 else ("Neutral" if p == 7 else "Base"))}

    @staticmethod
    def gas(t):
        t = t.lower()
        if "lime" in t: return {"Gas": "CO2", "Observation": "Turns lime water milky"}
        if "pop" in t: return {"Gas": "H2", "Observation": "Burns with pop sound"}
        if "splinter" in t: return {"Gas": "O2", "Observation": "Rekindles glowing splinter"}
        return {"Inference": "Requires specific test"}

    @staticmethod
    def soap(oil):
        oil = float(oil)
        return {"Yield (g)": _r(oil * 0.95, 2), "By-product": "Glycerol", "Process": "Saponification"}

    @staticmethod
    def hardness(edta, sample):
        edta, sample = float(edta), float(sample)
        h = (edta * 1000) / sample # simplified ppm CaCO3
        return {"Hardness (ppm)": _r(h, 2), "Type": "Hard" if h > 150 else "Soft"}

    @staticmethod
    def crystal(s, w):
        return {"Note": "Dissolve in hot water, filter, and cool slowly to obtain pure crystals"}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Exhaustive Lab Manual"
    EXP_DATA = {
        "Redox Titration": ("redox", [("M_permanganate", "0.02"), ("V_fas", "20"), ("V_permanganate", "20")]),
        "Kinetics (Order)": ("kinetics", [("Initial [A]", "1.0"), ("Rate Const k", "0.005"), ("Time (s)", "100")]),
        "Salt Analysis (Cation)": ("cation", [("Group (0-VI)", "0"), ("Test", "NaOH + Boil")]),
        "Organic Functional": ("organic", [("Test", "Tollen's")]),
        "Nernst Potential": ("nernst", [("E0 Cell (V)", "1.1"), ("n", "2"), ("Q (Quotient)", "0.01")]),
        "Ideal Gas Logic": ("ideal_gas", [("P (atm)", "1"), ("V (L)", "22.4")]),
        "Chromatography (Rf)": ("rf_val", [("Dist Solute (cm)", "6"), ("Dist Solvent (cm)", "10")]),
        "Osmotic Pressure": ("osmosis", [("Molarity", "0.1"), ("Temp (C)", "27"), ("i (van't Hoff)", "1")]),
        "Buffer pH": ("buffer", [("pKa", "4.74"), ("[Salt]", "0.1"), ("[Acid]", "0.1")]),
    }

    @staticmethod
    def redox(m_p, v_f, v_p):
        m_p, v_f, v_p = float(m_p), float(v_f), float(v_p)
        # 5 moles FAS : 1 mole Permanganate
        m_f = (5 * m_p * v_p) / v_f
        return {"Molarity FAS": _r(m_f, 4), "Strength (g/L)": _r(m_f * 392, 2)}

    @staticmethod
    def kinetics(a0, k, t):
        a0, k, t = float(a0), float(k), float(t)
        # [A] = [A]0 * exp(-kt) for 1st order
        at = a0 * math.exp(-k * t)
        return {"Final [A]t": _r(at, 4), "Conversion %": _r((1 - at/a0)*100, 2)}

    @staticmethod
    def cation(gp, t):
        gp, t = int(gp), t.lower()
        if gp == 0: return {"Cation": "NH4+", "Inference": "Ammonia smell on boiling with NaOH"}
        return {"Note": "Follow Group reagent sequence II-VI"}

    @staticmethod
    def organic(t):
        t = t.lower()
        if "tollen" in t: return {"Result": "Silver Mirror", "Group": "Aldehyde"}
        if "fecl3" in t: return {"Result": "Violet Color", "Group": "Phenol"}
        if "litmus" in t: return {"Result": "Red", "Group": "Carboxylic Acid"}
        return {"Note": "Specific group test required"}

    @staticmethod
    def nernst(e0, n, q):
        e0, n, q = float(e0), float(n), float(q)
        e = e0 - (0.0591 / n) * math.log10(q)
        return {"E Cell (V)": _r(e, 4)}

    @staticmethod
    def ideal_gas(p, v):
        p, v = float(p), float(v)
        r = 0.0821
        # PV = nRT -> T = PV/nR (for n=1)
        t = (p * v) / r
        return {"Temp (K)": _r(t, 2), "In C": _r(t - 273, 2)}

    @staticmethod
    def rf_val(du, ds):
        du, ds = float(du), float(ds)
        return {"Rf Value": _r(du/ds, 3)}

    @staticmethod
    def osmosis(m, t_c, i):
        m, t_c, i = float(m), float(t_c), float(i)
        r = 0.0821
        t = t_c + 273.15
        pi = i * m * r * t
        return {"Osmotic Pressure (atm)": _r(pi, 2)}

    @staticmethod
    def buffer(pka, s, a):
        pka, s, a = float(pka), float(s), float(a)
        ph = pka + math.log10(s/a)
        return {"Buffer pH": _r(ph, 2)}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
