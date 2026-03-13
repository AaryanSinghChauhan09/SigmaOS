"""
SigmaOS NCERT Biology Lab v9.0 — The Comprehensive series
Classes 6–12 | Exhaustive NCERT Biological Study & Ecology
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Advanced Anatomy & Physiology"
    EXP_DATA = {
        "Starch Test": ("starch", [("Iodine Presence (1/0)", "1")]),
        "Plant Growth (Variable)": ("growth", [("Sun (1-10)", "8"), ("Water (1-10)", "7"), ("Soil (1-10)", "9")]),
        "Cheek Cell Study": ("cheek", [("Stain", "Methylene Blue")]),
        "Human Heart Rate": ("heart", [("Age", "15"), ("Post-exercise (1/0)", "0")]),
        "Mendel Monohybrid": ("mendel_m", [("P1 (TT/Tt/tt)", "Tt"), ("P2 (TT/Tt/tt)", "Tt")]),
        "Osmosis (Potato/Raisins)": ("osmosis", [("Inside", "Sugar"), ("Outside", "Water")]),
        "Dicot vs Monocot": ("plant_type", [("Venation", "Reticulate")]),
        "Sex Determination": ("gender", [("Inherited (X/Y)", "Y")]),
        "Micro-organisms (Curd)": ("curd", [("Starter (ml)", "5"), ("Temp (C)", "37")]),
        "Pulse Rate Recovery": ("recovery", [("Max Pulse", "150"), ("Recovery (mins)", "5")]),
    }

    @staticmethod
    def starch(i):
        if int(i): return {"Result": "Blue-black", "Note": "Photosynthesis products detected"}
        return {"Result": "Brown", "Note": "No starch detected"}

    @staticmethod
    def growth(s, w, f):
        s, w, f = float(s), float(w), float(f)
        score = (s + w + f) / 3
        if score > 8: return {"Rate": "OPTIMAL", "Observation": "Vibrant growth, healthy biomass"}
        return {"Rate": "SUB-OPTIMAL", "Observation": "Slow growth or stress signs"}

    @staticmethod
    def cheek(stain):
        return {"Visual": "Blue nucleus visible, cell membrane distinct", "Type": "Eukaryotic"}

    @staticmethod
    def heart(age, ex):
        age, ex = float(age), int(ex)
        base = 72 + (20 - age) * 0.5
        if ex: base += 40
        return {"Estimated Pulse (bpm)": int(base)}

    @staticmethod
    def mendel_m(p1, p2):
        g = [a+b for a in p1 for b in p2]
        g = ["".join(sorted(x)) for x in g]
        pheno = "Tall" if "T" in g[0] else "Dwarf"
        return {"Offspring Genotypes": g, "Phenotype Counts": dict(Counter(g))}

    @staticmethod
    def osmosis(ins, outs):
        ins, outs = ins.lower(), outs.lower()
        if "sugar" in ins and "water" in outs: return {"Process": "ENDOSMOSIS", "Result": "Osmometer level rises"}
        return {"Process": "EXOSMOSIS/EQUILIBRIUM", "Result": "Osmometer level falls or stays"}

    @staticmethod
    def plant_type(v):
        if "retic" in v.lower(): return {"Type": "DICOT", "Character": "Tap Root, 2 Cotyledons"}
        return {"Type": "MONOCOT", "Character": "Fibrous Root, 1 Cotyledon"}

    @staticmethod
    def gender(chr):
        if "Y" in chr.upper(): return {"Offspring": "MALE (XY)", "Determiner": "Father"}
        return {"Offspring": "FEMALE (XX)", "Determiner": "Mother"}

    @staticmethod
    def curd(s, t):
        s, t = float(s), float(t)
        if 35 <= t <= 45: return {"Status": "SUCCESS", "Note": "Lactobacillus fermentation rapid"}
        return {"Status": "FAILED", "Note": "Temperature unsuitable for bacteria"}

    @staticmethod
    def recovery(max_p, t):
        max_p, t = float(max_p), float(t)
        final = max_p - (t * 15)
        return {"Restored Pulse": int(max(72, final)), "Fitness level": "Good" if final < 90 else "Fair"}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Exhaustive Lab Manual"
    EXP_DATA = {
        "Meiosis Simulation": ("meiosis", [("Stage", "Anaphase I")]),
        "Transpiration Rate": ("transpire", [("Wind Speed", "5"), ("Humidity", "20")]),
        "Respiratory Quotient": ("rq", [("Substrate (Carb/Fat)", "Carb"), ("CO2", "1"), ("O2", "1")]),
        "Species Area relationship": ("species", [("Area (km²)", "100"), ("z slope", "0.3")]),
        "Hardy-Weinberg": ("hardy", [("p-allele", "0.6"), ("Total Pop", "1000")]),
        "Biomass Pyramid": ("biomass", [("Trophics", "1000,100,10,1")]),
        "DNA Complement": ("dna_comp", [("Sequence", "ATGCGTA")]),
        "Trophic Efficiency": ("trophic", [("Producer Energy (J)", "10000")]),
        "Pedigree Logic": ("pedigree", [("Mode (AD/AR/XD/XR)", "AR"), ("Affected Father (1/0)", "1"), ("Carrier Mother (1/0)", "1")]),
    }

    @staticmethod
    def meiosis(s):
        s = s.lower()
        if "meta" in s: return {"Observation": "Homologous pairs at equator"}
        if "ana" in s: return {"Observation": "Homologous chromosomes separate"}
        return {"Note": "Reductional division stage"}

    @staticmethod
    def transpire(v, h):
        v, h = float(v), float(h)
        rate = (v * 10) / (h / 10) if h > 0 else v * 10
        return {"Relative Rate": _r(rate, 2), "Inference": "High wind & low humidity increase rate"}

    @staticmethod
    def rq(sub, co2, o2):
        co2, o2 = float(co2), float(o2)
        res = co2 / o2
        return {"RQ": _r(res, 2), "Inference": "Carbohydrate = 1, Fats < 1"}

    @staticmethod
    def species(a, z):
        a, z = float(a), float(z)
        # S = C * A^z (simplified C=1)
        s = a**z
        return {"Species Richness (S)": _r(s, 2)}

    @staticmethod
    def hardy(p, total):
        p, total = float(p), float(total)
        q = 1 - p
        return {"AA (p²)": int(total * p**2), "Aa (2pq)": int(total * 2*p*q), "aa (q²)": int(total * q**2)}

    @staticmethod
    def biomass(t_str):
        nums = [float(x.strip()) for x in t_str.split(",")]
        return {"Pyramid Status": "Upright" if all(nums[i] > nums[i+1] for i in range(len(nums)-1)) else "Inverted"}

    @staticmethod
    def dna_comp(s):
        comp = {"A":"T", "T":"A", "C":"G", "G":"C"}
        res = "".join(comp.get(b.upper(), b) for b in s)
        return {"Complementary": res}

    @staticmethod
    def trophic(e):
        e = float(e)
        return {"Primary Consumer": _r(e * 0.1), "Secondary": _r(e * 0.01), "Tertiary": _r(e * 0.001), "Rule": "10% Energy Law"}

    @staticmethod
    def pedigree(m, af, cm):
        m = m.upper()
        if m == "AR" and int(af) and int(cm): return {"Risk of Affected Child": "50%", "Observation": "Autosomal Recessive carrier x affected"}
        return {"Note": "Refer Punnett square for specific inheritance probabilities"}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}
