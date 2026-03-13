"""
SigmaOS NCERT Biology Lab v4.0
Classes 6–12 | Every NCERT Biology concept & simulation
100% stdlib, zero 3rd-party deps
"""
import math, random
from collections import Counter

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Biology_Classes_6_10:
    TITLE = "Classes 6–10 – Cells, Life Processes, Control & Coordination"
    EXP_DATA = {
        "Starch Test": ("starch", [("Iodine added (1/0)", "1")]),
        "Cell Organelles": ("cell", [("Organelle", "Mitochondria")]),
        "Mendel Monohybrid": ("mendel", [("P1 (TT/Tt/tt)", "Tt"), ("P2 (TT/Tt/tt)", "Tt")]),
        "Heart Rate": ("heart", [("Age", "15"), ("Post-exercise (1/0)", "0")]),
        "Blood Group Match": ("blood", [("Donor", "O-"), ("Recipient", "A+")]),
        "Plant Hormones": ("hormones", [("Effect", "Growth")]),
    }

    @staticmethod
    def starch(iodine):
        if int(iodine): return {"Result": "Blue-black", "Inference": "Starch Present"}
        return {"Result": "Brown", "Inference": "No Starch"}

    @staticmethod
    def cell(name):
        n = name.lower()
        data = {
            "nucleus": "Genetics, Control center",
            "mitochondria": "ATP, Powerhouse",
            "chloroplast": "Photosynthesis",
            "ribosome": "Proteins",
            "lysosome": "Suicide bags"
        }
        return {"Function": data.get(n, "Refer Class 9 Biology")}

    @staticmethod
    def mendel(p1, p2):
        genotypes = [a+b for a in p1 for b in p2]
        genotypes = ["".join(sorted(g)) for g in genotypes]
        return {"Offspring": genotypes, "Stats": dict(Counter(genotypes))}

    @staticmethod
    def heart(age, ex):
        age, ex = int(age), int(ex)
        base = 72
        if ex: base += 30
        return {"Estimated Pulse": base, "Note": "Normal range: 60-100 bpm"}

    @staticmethod
    def blood(d, r):
        d, r = d.upper(), r.upper()
        if "O-" in d: return {"Status": "SUCCESS", "Note": "O- is universal donor"}
        if d == r: return {"Status": "SUCCESS"}
        if "AB+" in r: return {"Status": "SUCCESS", "Note": "AB+ is universal recipient"}
        return {"Status": "RISKY", "Note": "Cross-match required"}

    @staticmethod
    def hormones(eff):
        data = {
            "growth": "Auxins / Gibberellins",
            "cell division": "Cytokinins",
            "wilting": "Abscisic Acid (ABA)",
            "ripening": "Ethylene"
        }
        return {"Phytohormone": data.get(eff.lower(), "Unknown")}

class Biology_Classes_11_12:
    TITLE = "Classes 11–12 – Physiology, Ecology, Evolution & Biotechnology"
    EXP_DATA = {
        "Species Area Rel": ("species_area", [("Area (sq km)", "1000")]),
        "PCR Copies": ("pcr", [("Initial", "1"), ("Cycles", "30")]),
        "Hardy-Weinberg": ("hardy", [("Allele freq p", "0.6")]),
        "Respiratory Quotient": ("rq", [("CO2", "1"), ("O2", "1")]),
        "Ecological Pyramid": ("pyramid", [("Producer Energy (J)", "10000")]),
        "DNA Transcribe": ("transcribe", [("Sequence (DNA)", "ATGC")]),
        "Cranial Nerves": ("cranial", [("Number (1-12)", "10")]),
    }

    @staticmethod
    def species_area(a):
        # S = C * A^Z (C=0.5, Z=0.3)
        s = 0.5 * (float(a) ** 0.3)
        return {"Richness S": _r(s, 2)}

    @staticmethod
    def pcr(n, cycles):
        return {"Yield": int(n) * (2 ** int(cycles))}

    @staticmethod
    def hardy(p):
        p = float(p)
        q = 1.0 - p
        return {"p² (AA)": _r(p**2, 4), "2pq (Aa)": _r(2*p*q, 4), "q² (aa)": _r(q**2, 4)}

    @staticmethod
    def rq(co2, o2):
        res = float(co2) / float(o2)
        return {"RQ": _r(res, 2)}

    @staticmethod
    def pyramid(e):
        e = float(e)
        # 10% law
        return {"Herbivores": e*0.1, "Carnivores": e*0.01, "Top Carnivores": e*0.001}

    @staticmethod
    def transcribe(dna):
        dna = dna.upper()
        m = {"A":"U", "T":"A", "G":"C", "C":"G"}
        return {"mRNA": "".join(m.get(x, "?") for x in dna)}

    @staticmethod
    def cranial(num):
        data = {
            1: "Olfactory (Smell)", 2: "Optic (Vision)", 3: "Oculomotor", 
            10: "Vagus (Heart/Lungs)", 12: "Hypoglossal"
        }
        return {"Nerve": data.get(int(num), "Refer NCERT Class 11 Biology")}

BIOLOGY_REGISTRY = {
    "Classes 6-10": Biology_Classes_6_10,
    "Classes 11-12": Biology_Classes_11_12,
}
