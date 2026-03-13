"""
SigmaOS NCERT Biology Lab v10.0 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Anatomy & Physiology"
    EXP_DATA = {
        "Starch Test": ("starch", [("Iodine Presence", "1")]),
        "Plant Growth": ("growth", [("Sun (1-10)", "8"), ("Water (1-10)", "7")]),
        "Heart Rate": ("heart", [("Age", "15"), ("Post-Ex (1/0)", "0")]),
        "Mendel Cross": ("mendel", [("P1 (TT/tt)", "Tt"), ("P2", "Tt")]),
        "Osmosis Logic": ("osmosis", [("In", "Sugar"), ("Out", "Water")]),
        "Sex Determination": ("gender", [("Inherited (X/Y)", "Y")]),
        "Digestion (Salivary)": ("amylase", [("pH", "7"), ("Temp (C)", "37")]),
        "Pulse Recovery": ("recovery", [("Max Pulse", "150"), ("Time_min", "5")]),
    }

    @staticmethod
    def starch(i):
        if int(i): return {"Result": "Blue-black", "Note": "Positive"}
        return {"Result": "Brown", "Note": "Negative"}

    @staticmethod
    def growth(s, w):
        score = (s + w) / 2
        return {"Health": "Optimal" if score > 7 else "Stress"}

    @staticmethod
    def heart(age, ex):
        base = 72 + (20-age)*0.5
        if int(ex): base += 40
        return {"Pulse": int(base)}

    @staticmethod
    def mendel(p1, p2):
        g = [a+b for a in p1 for b in p2]
        return {"Genotype": g, "Counts": dict(Counter(g))}

    @staticmethod
    def osmosis(ins, outs):
        if "sugar" in ins.lower() and "water" in outs.lower(): return {"Result": "Endosmosis (Swell)"}
        return {"Result": "Exosmosis (Shrink)"}

    @staticmethod
    def gender(chr):
        if "Y" in chr.upper(): return {"Offspring": "Male (XY)"}
        return {"Offspring": "Female (XX)"}

    @staticmethod
    def amylase(ph, t):
        if 6.8 <= ph <= 7.2 and 35 <= t <= 40: return {"Activity": "Optimal (Colorless after Iodine)"}
        return {"Activity": "Slow (Starch remains)"}

    @staticmethod
    def recovery(max_p, t):
        res = max_p - (t * 15)
        return {"Pulse": int(max(72, res))}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Exhaustive Lab Manual"
    EXP_DATA = {
        "Meiosis Stages": ("meiosis", [("Stage", "Anaphase I")]),
        "Transpiration Rate": ("transpire", [("Wind", "5"), ("Humidity", "20")]),
        "Respiratory Quotient": ("rq", [("CO2 Vol", "1"), ("O2 Vol", "1")]),
        "Hardy-Weinberg": ("hardy", [("p-allele", "0.6"), ("Pop Total", "1000")]),
        "DNA Sequence": ("dna_comp", [("Seq", "ATGCGTA")]),
        "Trophic 10% Law": ("trophic", [("Producer Energy", "10000")]),
        "Pedigree Probability": ("pedigree", [("Affected Dad (1/0)", "1"), ("Carrier Mom", "1")]),
        "Quadrat Density": ("quadrat", [("Area (m²)", "1"), ("Counts", "10,12,8,15")]),
        "Pollen Germination": ("pollen", [("Sugar %", "10"), ("Time (min)", "60")]),
        "Onion Mitosis": ("mitosis", [("Phase", "Metaphase")]),
    }

    @staticmethod
    def meiosis(s):
        s = s.lower()
        if "meta" in s: return {"Obs": "Align at equator"}
        if "ana" in s: return {"Obs": "Chromosomes separate"}
        return {"Obs": "Reduction division active"}

    @staticmethod
    def transpire(v, h):
        rate = (v * 10) / (h / 10) if h > 0 else v * 10
        return {"Rate": _r(rate, 2), "Inference": "High V, Low H = High Rate"}

    @staticmethod
    def rq(co2, o2):
        res = float(co2)/float(o2)
        return {"RQ": _r(res, 2), "Type": "Carb=1.0, Fats<1.0"}

    @staticmethod
    def hardy(p, n):
        q = 1 - p
        return {"AA (p²)": int(n*p**2), "Aa (2pq)": int(n*2*p*q), "aa (q²)": int(n*q**2)}

    @staticmethod
    def dna_comp(s):
        d = {"A":"T", "T":"A", "C":"G", "G":"C"}
        return {"Complementary": "".join(d.get(b.upper(), b) for b in s)}

    @staticmethod
    def trophic(e):
        return {"P_Consumer": _r(e*0.1), "S_Consumer": _r(e*0.01), "T_Consumer": _r(e*0.001)}

    @staticmethod
    def pedigree(ad, cm):
        if int(ad) and int(cm): return {"Risk": "50% Affected, 50% Carrier"}
        return {"Risk": "Refer Punnett Logic"}

    @staticmethod
    def quadrat(a, c_str):
        n = [float(x.strip()) for x in c_str.split(",")]
        avg = sum(n) / (a * len(n))
        return {"Density (org/m²)": _r(avg, 2)}

    @staticmethod
    def pollen(s, t):
        if 5 <= s <= 15: return {"Status": "Pollen Tube Growing"}
        return {"Status": "Stunted"}

    @staticmethod
    def mitosis(p):
        p = p.lower()
        if "meta" in p: return {"Obs": "Chromatids align at equator, Spindle fibers visible"}
        if "ana" in p: return {"Obs": "Sister chromatids pull apart"}
        return {"Obs": "Division of Nucleus"}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}
