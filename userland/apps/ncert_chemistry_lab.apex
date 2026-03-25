"""
SigmaOS NCERT Chemistry Lab v10.1 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, re

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
        "Salts Stability": ("salts", [("Compound", "NaCl")]),
        "Stoichiometry": ("stoich", [("Reactant Mass", "10"), ("Molar Mass R", "40"), ("Molar Mass P", "60")]),
        "VSEPR Shape": ("vsepr", [("Steric Num", "4"), ("Lone Pairs", "0")]),
    }

    @staticmethod
    def atom(z):
        d = {1:"H", 2:"He", 6:"C", 7:"N", 8:"O", 11:"Na", 17:"Cl", 26:"Fe", 79:"Au"}
        return {"Symbol": d.get(z, "?"), "Z": z}

    @staticmethod
    def ph(c, isa):
        p = -math.log10(c) if int(isa) else 14+math.log10(c)
        return {"pH": _r(p, 2), "Nature": "Acid" if p < 7 else "Base"}

    @staticmethod
    def molar_mass(f):
        mats = re.findall(r'([A-Z][a-z]?)(\d*)', f)
        total = 0; elem = {"H":1.0, "C":12.0, "O":16.0, "Na":23.0, "S":32.1, "Cl":35.5}
        for s, c in mats:
            n = int(c) if c else 1
            total += elem.get(s, 0) * n
        return {"Molar Mass": _r(total, 2)}

    @staticmethod
    def titration(m1, v1, v2):
        return {"M2": _r(m1*v1/v2, 4)}

    @staticmethod
    def vsepr(s, l):
        s, l = int(s), int(l)
        if s == 2: return {"Shape": "Linear"}
        if s == 3: return {"Shape": "Trigonal Planar" if l==0 else "Bent"}
        if s == 4: return {"Shape": "Tetrahedral" if l==0 else "Trigonal Pyramidal" if l==1 else "Bent"}
        return {"Shape": "Complex"}

    @staticmethod
    def salts(c):
        return {"Solubility": "High in Water" if "NaCl" in c or "K" in c else "Lookup Table"}

    @staticmethod
    def stoich(m, mmr, mmp):
        return {"Product Mass": _r((m/mmr)*mmp, 2)}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Exhaustive Lab Suite"
    EXP_DATA = {
        "Nernst Equation": ("nernst", [("E0 Cell", "1.1"), ("n", "2"), ("Q", "0.01")]),
        "Reaction Equilibrium": ("keq", [("Kc", "40"), ("Qc", "60")]),
        "Bond Energy": ("bond", [("B_react (kJ)", "500"), ("B_prod (kJ)", "650")]),
        "Arrhenius Rate": ("arrhenius", [("A", "1e11"), ("Ea (kJ)", "50"), ("T (K)", "300")]),
        "Osmosis (Pi)": ("osmosis", [("M", "0.1"), ("T (C)", "27"), ("i", "1")]),
        "EAN Finder": ("ean", [("Z", "26"), ("OxState", "2"), ("CN", "6")]),
        "Free Energy (G)": ("gibbs", [("H (kJ)", "100"), ("S (J/K)", "50"), ("T (K)", "298")]),
        "Boiling Elevation": ("boiling", [("Kb", "0.52"), ("m (molal)", "1"), ("i", "1")]),
        "Functional Groups": ("functional", [("Sample", "Vinegar")]),
        "Salt Analysis": ("salt_anal", [("Color", "Blue"), ("Flame", "Green")]),
        "Molarity Lab": ("molarity", [("Mass (g)", "4"), ("MW", "40"), ("Vol (L)", "1")]),
        "Chromatography": ("chromatog", [("Dist Solv (cm)", "10"), ("Dist Spot (cm)", "4.5")]),
        "Rate of Reaction": ("reaction_rate", [("Conc1 (M)", "0.5"), ("Conc2 (M)", "0.2"), ("Time (s)", "40")]),
        "Equilibrium (Kc)": ("equilibrium", [("ConcA (M)", "0.1"), ("ConcB (M)", "0.1"), ("ConcC (M)", "0.4")]),
        "Iodine Clock": ("iodine_clock", [("Conc (M)", "0.1"), ("Temp (C)", "25")]),
        "Sol Preparation": ("sol_prep", [("Type", "Gold Sol")]),
        "Neut-Enthalpy": ("enthalpy", [("Neutral (H+ moles)", "0.05"), ("Temp Change (K)", "2")]),
        "Anion Analysis": ("anion", [("Group", "1 (Dil. H2SO4)"), ("Substance", "Salt X")]),
        "Oxalic vs KMnO4": ("kmno4", [("Vol KMnO4", "18.5"), ("M_Oxalic", "0.05"), ("Vol_Oxalic", "20")]),
        "Tollen's Test": ("tollen", [("Sample", "Glucose")]),
        "Concentration Rate": ("conc_rate", [("Initial Conc", "1.0"), ("Final Conc", "0.8"), ("Time (s)", "100")]),
    }

    @staticmethod
    def nernst(e0, n, q):
        e = e0 - (0.0591/n)*math.log10(q)
        return {"E_Cell (V)": _r(e, 4)}

    @staticmethod
    def keq(kc, qc):
        if qc < kc: return {"Shift": "FORWARD"}
        if qc > kc: return {"Shift": "BACKWARD"}
        return {"Shift": "EQUILIBRIUM"}

    @staticmethod
    def bond(br, bp):
        dH = br - bp
        return {"delta_H": dH, "Type": "EXO" if dH<0 else "ENDO"}

    @staticmethod
    def arrhenius(a, ea, t):
        k = a * math.exp(-ea/(0.00831*t))
        return {"k": f"{k:.4e}"}

    @staticmethod
    def osmosis(m, tc, i):
        pi = i * m * 0.0821 * (tc+273.15)
        return {"Pi (atm)": _r(pi, 2)}

    @staticmethod
    def ean(z, ox, cn):
        res = z - ox + 2*cn
        return {"EAN": int(res)}

    @staticmethod
    def gibbs(h, s, t):
        g = h - t*(s/1000)
        return {"Delta G (kJ)": _r(g, 2), "Spontaneous": g < 0}

    @staticmethod
    def boiling(kb, m, i):
        dt = i * kb * m
        return {"Delta Tb": _r(dt, 3)}

    @staticmethod
    def functional(s):
        s = s.lower()
        if "vinegar" in s or "acetic" in s: return {"Group": "-COOH (Carboxylic Acid)", "Test": "Effervescence with NaHCO3"}
        if "alcohol" in s or "ethanol" in s: return {"Group": "-OH (Alcohol)", "Test": "Ester formation with Acid"}
        if "acetone" in s: return {"Group": ">C=O (Ketone)", "Test": "Sodium Nitroprusside"}
        return {"Group": "Unknown", "Action": "Perform Lucas Test"}

    @staticmethod
    def salt_anal(c, f):
        c, f = c.lower(), f.lower()
        if "blue" in c or "green" in f: return {"Cation": "Cu2+", "Confirmation": "Deep blue with Ammonia"}
        if "brick red" in f: return {"Cation": "Ca2+", "Confirmation": "White ppt with Ammonium Oxalate"}
        if "white" in c and "apple green" in f: return {"Cation": "Ba2+", "Confirmation": "Yellow ppt with K2CrO4"}
        return {"Cation": "Needs Wet Test", "Action": "Add NaOH"}

    @staticmethod
    def molarity(m, mw, v):
        mol = m/mw
        return {"Molarity (M)": _r(mol/v, 3)}

    @staticmethod
    def chromatog(ds, dsp):
        rf = dsp / ds
        return {"Rf Value": _r(rf, 3), "Status": "Success" if rf < 1 else "Error"}

    @staticmethod
    def reaction_rate(c1, c2, t):
        rate = abs(c2 - c1) / t
        return {"Avg Rate (M/s)": f"{rate:.4e}"}

    @staticmethod
    def equilibrium(a, b, c):
        # A + B <=> C
        kc = c / (a * b)
        return {"Kc": _r(kc, 2), "Prediction": "Stable" if kc > 1 else "Reactants Favored"}

    @staticmethod
    def iodine_clock(c, t):
        rate = c * (t/10)
        return {"Time to Blue (s)": _r(50/rate, 1), "Observation": "Clear -> Dark Blue"}

    @staticmethod
    def sol_prep(t):
        if "gold" in t.lower(): return {"Method": "Bredig's Arc", "Status": "Purple Sol"}
        if "ferric" in t.lower(): return {"Method": "Hydrolysis", "Result": "Reddish Brown Sol"}
        return {"Method": "Peptization", "Info": "Add electrolyte to precipitate"}

    @staticmethod
    def enthalpy(n, dt):
        # Simplified dH = -(m*c*dt)/n
        dh = -(100 * 4.184 * dt) / n # Assuming 100g water
        return {"delta_H (kJ/mol)": _r(dh/1000, 2), "Note": "Exothermic Neutralization"}

    @staticmethod
    def anion(g, s):
        if "1" in g: return {"Observation": "Brisk Effervescence", "Result": "CO3 2- (Carbonate)"}
        if "2" in g: return {"Observation": "Brown Fumes", "Result": "NO3 - (Nitrate)"}
        return {"Step": "Perform Confirmatory Test"}

    @staticmethod
    def kmno4(v1, m2, v2):
        # n1M1V1 = n2M2V2; n_kmno4=5, n_oxalic=2
        m1 = (2 * m2 * v2) / (5 * v1)
        return {"Molarity KMnO4": _r(m1, 4), "Endpoint": "Permanent Light Pink"}

    @staticmethod
    def tollen(s):
        if "aldehyde" in s.lower() or "glucose" in s.lower(): 
            return {"Observation": "Silver Mirror formed", "Result": "Aldehyde PRESENT"}
        return {"Observation": "No Mirror", "Result": "Ketone/Other"}

    @staticmethod
    def conc_rate(c1, c2, t):
        rate = abs(c2 - c1) / t
        return {"Rate (M/s)": f"{rate:.4e}", "Order": "Assumed First Order"}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
