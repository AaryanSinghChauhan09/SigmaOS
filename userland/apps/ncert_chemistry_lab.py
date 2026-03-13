"""
SigmaOS NCERT Chemistry Lab v7.0 — The Ultimate Lab Manual
Classes 6–12 | Every Core NCERT Experiment & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Secondary Chemistry: Atomic, Acidic & Reaction basics"
    EXP_DATA = {
        "pH & Indicators": ("ph_logic", [("H+ Conc (M)", "0.001")]),
        "Atomic Structure": ("atom_build", [("Protons", "6"), ("Mass Number", "12")]),
        "Mole Calculator": ("moles", [("Given Mass (g)", "44"), ("Molar Mass (g/mol)", "44")]),
        "Displacement Reactivity": ("displacement", [("Metal (Fe/Cu/Zn)", "Fe"), ("Solution (CuSO4/ZnSO4)", "CuSO4")]),
        "Salt Distillation": ("distill", [("Salt Weight (g)", "50"), ("Water (ml)", "500")]),
        "Combustion Logic": ("combustion", [("Fuel", "Magnesium")]),
        "Saturated Solutions": ("solubility", [("Solute (g)", "36"), ("Water (ml)", "100"), ("Temp (C)", "25")]),
        "Acid-Base Neutralization": ("neutral", [("Acid Molarity", "0.1"), ("Acid Vol (ml)", "20"), ("Base Molarity", "0.1")]),
    }

    @staticmethod
    def ph_logic(h):
        h = float(h)
        ph = -math.log10(h)
        return {"pH": _r(ph, 2), "Litmus": "Red" if ph < 7 else "Blue", "Universal": "Orange/Red" if ph < 4 else ("Purple/Blue" if ph > 10 else "Green")}

    @staticmethod
    def atom_build(p, a):
        p, a = float(p), float(a)
        return {"Protons": int(p), "Electrons": int(p), "Neutrons": int(a-p)}

    @staticmethod
    def moles(m, mm):
        m, mm = float(m), float(mm)
        return {"Amt (mol)": _r(m/mm, 4), "Molecules": f"{_r(m/mm * 6.022e23):.3e}"}

    @staticmethod
    def displacement(m, s):
        m, s = m.lower(), s.lower()
        series = ["potassium", "sodium", "calcium", "magnesium", "aluminium", "zinc", "iron", "lead", "hydrogen", "copper", "mercury", "silver", "gold"]
        try:
            m_idx = series.index(m)
            # detect metal in solution
            s_metal = s.replace("so4","").replace("cl2","").replace("no3","").strip()
            s_idx = series.index(s_metal)
            if m_idx < s_idx: return {"Status": "REACTION OCCURS", "Note": f"{m.capitalize()} displaces {s_metal.capitalize()}"}
            return {"Status": "NO REACTION", "Note": f"{m.capitalize()} is less reactive"}
        except: return {"Error": "Metal not in reactivity series"}

    @staticmethod
    def distill(s, w):
        return {"Result": "Pure water recovered", "Residue": f"{s}g of Salt", "Process": "Evaporation followed by Condensation"}

    @staticmethod
    def combustion(f):
        f = f.lower()
        if "magnesium" in f: return {"Note": "Burns with Dazzling White Flame", "Product": "Magnesium Oxide (Basic)"}
        return {"Note": "Reacts with Oxygen to release Heat/Light"}

    @staticmethod
    def solubility(s, w, t):
        s, w, t = float(s), float(w), float(t)
        # NaCl solubility is approx 36g/100ml at 25C
        limit = (36 + (t-25)*0.1) * (w/100)
        if s >= limit: return {"State": "Saturated", "Un-dissolved": _r(s-limit, 2)}
        return {"State": "Unsaturated", "Capacity left": _r(limit-s, 2)}

    @staticmethod
    def neutral(ma, va, mb):
        ma, va, mb = float(ma), float(va), float(mb)
        vb = (ma * va) / mb
        return {"Base Vol needed (ml)": _r(vb, 2), "Product": "Salt + Water", "Thermicity": "Exothermic"}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Analytical & Physical Labs"
    EXP_DATA = {
        "Preparation of Std Soln": ("std_soln", [("Solute (Na2CO3/Oxalic)", "Na2CO3"), ("Target M", "0.1"), ("Vol (ml)", "250")]),
        "Thermochemical Calorie": ("enthalpy", [("Mass of Water", "100"), ("dT (C)", "5")]),
        "Equilibrium Shift": ("equilibrium", [("Add Fe3+ (1/0)", "1"), ("Add SCN- (1/0)", "0")]),
        "Nernst Cell Potential": ("nernst", [("E0 Cell (V)", "1.1"), ("n (electrons)", "2"), ("Q (Quotient)", "0.01")]),
        "Rate of Reaction": ("rate", [("k rate", "0.01"), ("[A]0", "1.0"), ("Time (s)", "100")]),
        "Functional Group Test": ("functional", [("Compound (A/B/C)", "A"), ("Result", "Red Litmus turns Blue")]),
        "EAN for Complexes": ("ean", [("Z", "26"), ("Ox State", "2"), ("Coord Number", "6")]),
        "Paper Chromatography": ("chromato", [("Dist of Solvent (cm)", "10"), ("Dist of Spot (cm)", "6")]),
    }

    @staticmethod
    def std_soln(s, m, v):
        m, v = float(m), float(v)
        weights = {"na2co3": 106, "oxalic": 126} # Oxalic Dihydrate
        mm = weights.get(s.lower(), 100)
        mass = m * mm * (v/1000)
        return {"Mass needed (g)": _r(mass, 4), "Protocol": f"Dissolve in {v}ml distilled water"}

    @staticmethod
    def enthalpy(m, dt):
        m, dt = float(m), float(dt)
        q = m * 4.184 * dt # Joules
        return {"Heat Absorbed Q (J)": _r(q, 1), "Note": "Assumed water specific heat = 4.184"}

    @staticmethod
    def equilibrium(fe, scn):
        if int(fe): return {"Shift": "FORWARD", "Color": "Darker Blood Red", "Reason": "Added reactant"}
        if int(scn): return {"Shift": "FORWARD", "Color": "Darker Blood Red"}
        return {"Status": "Dynamic Equilibrium"}

    @staticmethod
    def nernst(e0, n, q):
        e0, n, q = float(e0), float(n), float(q)
        e = e0 - (0.0591 / n) * math.log10(q)
        return {"E Cell (V)": _r(e, 4)}

    @staticmethod
    def rate(k, a0, t):
        k, a0, t = float(k), float(a0), float(t)
        # 1st order: [A] = [A]0 * e^-kt
        at = a0 * math.exp(-k * t)
        return {"Final [A]t": _r(at, 4), "Conversion %": _r((a0-at)/a0 * 100, 2)}

    @staticmethod
    def functional(c, r):
        r = r.lower()
        if "red litmus turns blue" in r: return {"Inference": "Amine or Basic group present"}
        if "effervescence" in r: return {"Inference": "Carboxylic Acid Group (-COOH) present"}
        return {"Inference": "Requires further Specific Test (e.g. Tollen's)"}

    @staticmethod
    def ean(z, ox, cn):
        z, ox, cn = float(z), float(ox), float(cn)
        res = z - ox + 2*cn
        return {"EAN": int(res), "Stability": "Electronic configuration of Noble Gas" if res in [36, 54, 86] else "Less Stable"}

    @staticmethod
    def chromato(ds, dr):
        ds, dr = float(ds), float(dr)
        # Rf = distance travelled by solute / distance travelled by solvent
        rf = dr / ds
        return {"Rf Value": _r(rf, 3)}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
