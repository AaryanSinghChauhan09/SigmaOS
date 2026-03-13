"""
SigmaOS NCERT Chemistry Lab v8.0 — The Interactive series
Classes 6–12 | Every Core NCERT Experiment & Calculator
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Chemistry_Classes_6_10:
    TITLE = "Secondary Chemistry: Interactive Micro-labs"
    EXP_DATA = {
        "Virtual Litmus Test": ("litmus", [("Solution (Lemon/Soap/Water)", "Lemon")]),
        "Separation Simulator": ("separation", [("Mixture (Sand+Salt/Oil+Water)", "Sand+Salt")]),
        "Physical/Chemical Change": ("change", [("Activity (Burning Wood/Melting Ice)", "Burning Wood")]),
        "Atomic Structure": ("atom", [("Z (Atomic Number)", "11")]),
        "Reactivity Series": ("reactivity", [("Metal 1", "Iron"), ("Metal 2", "Copper")]),
        "pH Calculator": ("ph", [("H+ Concentration (M)", "0.001")]),
        "Gas Identification": ("gas", [("Test (Lime Water/Pop Sound)", "Lime Water")]),
        "Soap Preparation": ("soap", [("Oil (g)", "50"), ("NaOH (g)", "15")]),
    }

    @staticmethod
    def litmus(sol):
        sol = sol.lower()
        if "lemon" in sol or "vinegar" in sol or "acid" in sol: return {"Paper Color": "RED", "Nature": "Acidic"}
        if "soap" in sol or "lime" in sol or "base" in sol: return {"Paper Color": "BLUE", "Nature": "Basic"}
        return {"Paper Color": "NO CHANGE", "Nature": "Neutral"}

    @staticmethod
    def separation(mix):
        m = mix.lower()
        if "sand" in m and "salt" in m: return {"Methods": "Filtration followed by Evaporation"}
        if "oil" in m and "water" in m: return {"Methods": "Separating Funnel"}
        return {"Methods": "Refer NCERT Class 6 Ch-5"}

    @staticmethod
    def change(act):
        act = act.lower()
        if "burn" in act or "rust" in act or "cook" in act: return {"Type": "CHEMICAL CHANGE", "Note": "Irreversible, New substances formed"}
        return {"Type": "PHYSICAL CHANGE", "Note": "Reversible, No new substances"}

    @staticmethod
    def atom(z):
        z = int(z)
        elements = {1:"H", 2:"He", 6:"C", 7:"N", 8:"O", 11:"Na", 17:"Cl", 26:"Fe"}
        sym = elements.get(z, "?")
        return {"Symbol": sym, "Electrons": z, "Config": "Refer Bohr-Bury Rule"}

    @staticmethod
    def reactivity(m1, m2):
        series = ["Potassium", "Sodium", "Calcium", "Magnesium", "Aluminium", "Zinc", "Iron", "Lead", "Hydrogen", "Copper", "Mercury", "Silver", "Gold"]
        try:
            i1 = series.index(m1.capitalize())
            i2 = series.index(m2.capitalize())
            return {"More Reactive": m1 if i1 < i2 else m2, "Displacement": "Possible" if i1 < i2 else "Impossible"}
        except: return {"Error": "Metal not in series"}

    @staticmethod
    def ph(h):
        h = float(h)
        p = -math.log10(h)
        return {"pH": _r(p, 2), "Nature": "Acid" if p < 7 else "Base"}

    @staticmethod
    def gas(t):
        t = t.lower()
        if "lime" in t: return {"Inference": "CO2 present (Turns milky)"}
        if "pop" in t: return {"Inference": "H2 present (Burns with pop sound)"}
        return {"Inference": "Unknown gas test"}

    @staticmethod
    def soap(oil, naoh):
        oil, naoh = float(oil), float(naoh)
        # Saponification simplified
        soap_yield = oil * 0.9 + naoh * 0.5
        return {"Soap Yield (g)": _r(soap_yield, 2), "Process": "Saponification (Hydrolysis of Fats)"}

class Chemistry_Classes_11_12:
    TITLE = "Senior Chemistry: Precise Lab Work"
    EXP_DATA = {
        "Titration Calculator": ("titration", [("M1 (Acid)", "0.1"), ("V1 (Acid)", "20"), ("V2 (Base)", "22.5")]),
        "Redox (KMnO4)": ("redox", [("M_kmno4", "0.02"), ("V_fas", "20")]),
        "Salt Analysis (Anion)": ("anion", [("Observation (Effervescence/White PPT)", "Effervescence")]),
        "Electrolysis Yield": ("faraday", [("Current (A)", "2"), ("Time (s)", "965"), ("Eq Wt", "31.75")]),
        "Coordination (EAN)": ("ean", [("Z", "26"), ("Ox State", "2"), ("Coord Number", "6")]),
        "Nernst Cell Potential": ("nernst", [("E0 Cell", "1.1"), ("n", "2"), ("Q", "0.01")]),
        "Packing Efficiency": ("packing", [("Crystal (SCC/BCC/FCC)", "FCC")]),
        "Functional Group ID": ("functional", [("Test (Tollen's/FeCl3)", "Tollen's")]),
    }

    @staticmethod
    def titration(m1, v1, v2):
        m1, v1, v2 = float(m1), float(v1), float(v2)
        m2 = (m1 * v1) / v2
        return {"Concentration M2": _r(m2, 4)}

    @staticmethod
    def redox(m_k, v_f):
        m_k, v_f = float(m_k), float(v_f)
        # 5 moles FAS : 1 mole KMnO4
        m_f = (5 * m_k * 20) / v_f # simplified model
        return {"Molarity of FAS": _r(m_f, 4)}

    @staticmethod
    def anion(obs):
        obs = obs.lower()
        if "effer" in obs: return {"Inference": "Carbonate (CO3 2-) or Bicarbonate"}
        if "white ppt" in obs: return {"Inference": "Chloride (Cl-) or Sulphate (SO4 2-)"}
        return {"Inference": "Requires specific group reagent"}

    @staticmethod
    def faraday(i, t, w):
        i, t, w = float(i), float(t), float(w)
        mass = (i * t * w) / 96500
        return {"Mass Yield (g)": _r(mass, 5)}

    @staticmethod
    def ean(z, ox, cn):
        z, ox, cn = float(z), float(ox), float(cn)
        res = z - ox + 2*cn
        return {"EAN": int(res), "Stability": "Stable Noble Gas Config" if res in [36, 54, 86] else "Less Stable"}

    @staticmethod
    def nernst(e0, n, q):
        e0, n, q = float(e0), float(n), float(q)
        e = e0 - (0.0591 / n) * math.log10(q)
        return {"E Cell (V)": _r(e, 4)}

    @staticmethod
    def packing(style):
        d = {"scc": "52.4%", "bcc": "68%", "fcc": "74%"}
        return {"Efficiency": d.get(style.lower(), "Unknown")}

    @staticmethod
    def functional(t):
        t = t.lower()
        if "tollen" in t: return {"Result": "Silver Mirror", "Group": "Aldehyde (-CHO)"}
        if "fecl3" in t: return {"Result": "Violet Color", "Group": "Phenolic (-OH)"}
        return {"Result": "Specific observation needed"}

CHEMISTRY_REGISTRY = {
    "Classes 6-10": Chemistry_Classes_6_10,
    "Classes 11-12": Chemistry_Classes_11_12,
}
