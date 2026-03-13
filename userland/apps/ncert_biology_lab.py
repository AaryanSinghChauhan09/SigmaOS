"""
SigmaOS NCERT Biology Lab v7.0 — The Ultimate Lab Manual
Classes 6–12 | Every Core NCERT Biological study & Ecology
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Secondary Biology: Life processes & Natural World"
    EXP_DATA = {
        "Starch Test": ("starch", [("Iodine Presence (1/0)", "1")]),
        "Food Component Tests": ("food_test", [("Substance", "Sugar"), ("Reagent", "Benedict's")]),
        "Cell Size Analysis": ("cell_size", [("Diameter (µm)", "20")]),
        "Heart Rate Sim": ("heart", [("Resting bpm", "72"), ("Exercise Level (1-10)", "5")]),
        "Mendel Monohybrid": ("mendel_m", [("P1 (TT/Tt/tt)", "Tt"), ("P2", "Tt")]),
        "Osmosis (Potato Osmometer)": ("osmosis_p", [("Inside Solution", "Sugar"), ("Outside", "Water")]),
        "Photosynthesis Rate": ("photo_rate", [("Light Distance (cm)", "20"), ("CO2 bubbles", "10")]),
        "Sex Determination": ("gender", [("Father's Gamete (X/Y)", "Y")]),
    }

    @staticmethod
    def starch(i):
        if int(i): return {"Result": "Blue-Black", "Note": "Starch Present"}
        return {"Result": "Brown", "Note": "Starch Absent"}

    @staticmethod
    def food_test(s, r):
        s, r = s.lower(), r.lower()
        if "benedict" in r and "sugar" in s: return {"Result": "Orange/Red precipitate", "Inference": "Reducing Sugar found"}
        if "biuret" in r and "protein" in s: return {"Result": "Violet/Purple", "Inference": "Protein found"}
        return {"Result": "No specific reaction"}

    @staticmethod
    def cell_size(d):
        d = float(d)
        v = (4/3) * math.pi * (d/2)**3
        return {"Volume (µm³)": _r(v, 2), "Inference": "Prokaryotic cells are usually < 10µm"}

    @staticmethod
    def heart(b, e):
        b, e = float(b), float(e)
        final = b + (e * 12)
        return {"Exercised Pulse": int(final), "Inference": "Heart rate increases with demand"}

    @staticmethod
    def mendel_m(p1, p2):
        g = [a+b for a in p1 for b in p2]
        g = ["".join(sorted(x)) for x in g]
        return {"Genotype List": g, "Counts": dict(Counter(g)), "Phenotype": "3 Dominant : 1 Recessive (Approx)"}

    @staticmethod
    def osmosis_p(ins, outs):
        ins, outs = ins.lower(), outs.lower()
        if "sugar" in ins and "water" in outs: return {"Note": "Endosmosis occurs", "Water Level": "Rises inside the potato"}
        return {"Note": "Requires concentration gradient"}

    @staticmethod
    def photo_rate(d, b):
        d, b = float(d), float(b)
        rate = b / d # simplified
        return {"Relative Rate": _r(rate, 2), "Observation": "Rate increases as Light source moves closer"}

    @staticmethod
    def gender(g):
        if "Y" in g.upper(): return {"Offspring": "Male (XY)", "Note": "Father determines the sex"}
        return {"Offspring": "Female (XX)"}

class Biology_Classes_11_12:
    TITLE = "Senior Biology: Physiology & Biotechnology"
    EXP_DATA = {
        "Pollen Germination": ("pollen", [("Sugar %", "10"), ("Time (mins)", "60")]),
        "Plasmolysis (Rhoeo)": ("plasmolysis", [("Solution (Sugar/Water)", "Sugar")]),
        "Transpiration (Potometer)": ("transpiration", [("Wind Speed (m/s)", "2"), ("Humidity (%)", "50")]),
        "Meiosis Stages": ("cell_div", [("Stage", "Metaphase I")]),
        "DNA Extraction logic": ("dna_iso", [("Sample", "Banana"), ("Reagent", "Cold Ethanol")]),
        "Respiratory Quotient": ("rq", [("CO2 Vol", "1"), ("O2 Vol", "1")]),
        "Hardy-Weinberg Freq": ("hardy", [("Allele p", "0.6")]),
        "Population Density": ("pop_dens", [("Total Sample Area", "100"), ("Organism Counts", "15,20,5")]),
    }

    @staticmethod
    def pollen(s, t):
        s, t = float(s), float(t)
        if 5 <= s <= 15: return {"Status": "POLLEN TUBE GROWS", "Observation": "Length increases over time"}
        return {"Status": "NO GROWTH", "Reason": "Inappropriate sugar concentration"}

    @staticmethod
    def plasmolysis(sol):
        if "sugar" in sol.lower(): return {"Process": "Exosmosis", "Cell Status": "Plasmolysed (Shrunken Protoplast)"}
        return {"Process": "Endosmosis/Equilibrium", "Cell Status": "Turgid/Flaccid"}

    @staticmethod
    def transpiration(v, h):
        v, h = float(v), float(h)
        rate = (v * 10) / (h / 10) # arbitrary model
        return {"Relative Rate": _r(rate, 2), "Note": "Rate ∝ Wind Speed, 1/Humidity"}

    @staticmethod
    def cell_div(s):
        s = s.lower()
        if "meta" in s: return {"Observation": "Chromosomes align at Equatorial Plate"}
        if "ana" in s: return {"Observation": "Homologous chromosomes / chromatids move to opposite poles"}
        return {"Note": "Refer NCERT Class 11 Biology Ch-10"}

    @staticmethod
    def dna_iso(s, r):
        if "ethanol" in r.lower(): return {"Result": "DNA precipitates as white fibers", "Observation": "Spooling of DNA can be performed"}
        return {"Result": "Incomplete separation"}

    @staticmethod
    def rq(co2, o2):
        co2, o2 = float(co2), float(o2)
        res = co2 / o2
        return {"RQ": _r(res, 2), "Inference": "Carbohydrate=1.0, Fats < 1.0, Proteins ~0.9"}

    @staticmethod
    def hardy(p):
        p = float(p); q = 1.0 - p
        return {"p² (AA)": _r(p**2, 4), "2pq (Aa)": _r(2*p*q, 4), "q² (aa)": _r(q**2, 4)}

    @staticmethod
    def pop_dens(a, counts):
        a = float(a)
        nums = [float(x.strip()) for x in counts.split(",")]
        avg = sum(nums) / (a * len(nums))
        return {"Density (org/sq.unit)": _r(avg, 3)}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}
