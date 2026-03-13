"""
SigmaOS NCERT Primary Science (EVS) v6.0 — The Ultimate Series
Classes 1–5 | Foundational Environmental Studies & Science
100% stdlib, zero 3rd-party deps
"""
class Science_Primary_Classes:
    TITLE = "Primary EVS: Nature, Body, Food & Shelter"
    EXP_DATA = {
        "Sense Organs": ("senses", [("Organ (eyes/ears/nose/tongue/skin)", "eyes")]),
        "Living vs Non-living": ("living", [("Does it grow? (1/0)", "1"), ("Does it breathe? (1/0)", "1")]),
        "Water Cycle Basics": ("water_cycle", [("Process (Evaporation/Rain)", "Evaporation")]),
        "Food Groups": ("food", [("Item", "Rice")]),
        "Shelter Types": ("shelter", [("Area (Cold/Hot/Water)", "Cold")]),
        "Animal Habitats": ("habitats", [("Animal", "Fish")]),
        "Plant Parts": ("plant", [("Part (Root/Steam/Leaf)", "Leaf")]),
        "Types of Families": ("family", [("Type (Joint/Nuclear)", "Nuclear")]),
        "States of Matter": ("matter", [("Example (Ice/Water/Steam)", "Ice")]),
    }

    @staticmethod
    def senses(o):
        d = {"eyes":"Vision/Colors", "ears":"Hearing", "nose":"Smell", "tongue":"Taste", "skin":"Touch/Heat"}
        return {"Used for": d.get(o.lower(), "Refer Class 3 EVS")}

    @staticmethod
    def living(g, b):
        if int(g) and int(b): return {"Result": "Living Being"}
        return {"Result": "Non-living Object"}

    @staticmethod
    def water_cycle(p):
        d = {"evaporation": "Sun turns water to vapor", "rain": "Clouds release water drops"}
        return {"Step": d.get(p.lower(), "Refer Class 4 EVS")}

    @staticmethod
    def food(i):
        i = i.lower()
        if i in ["rice", "wheat", "potato"]: return {"Category": "Energy-giving (Carbs)"}
        if i in ["milk", "egg", "dal"]: return {"Category": "Body-building (Proteins)"}
        return {"Category": "Protective (Vitamins)"}

    @staticmethod
    def shelter(a):
        d = {"cold": "Igloos / Sloping roofs", "hot": "Thick Mud walls", "water": "Houseboats"}
        return {"Shelter Style": d.get(a.lower(), "Refer Class 5 Shelter section")}

    @staticmethod
    def habitats(a):
        d = {"fish": "Water (Aquatic)", "lion": "Forest (Terrestrial)", "monkey": "Trees (Arboreal)"}
        return {"Habitat": d.get(a.lower(), "Refer Class 4 Habitats")}

    @staticmethod
    def plant(p):
        d = {"root": "Absorbs water", "leaf": "Makes food", "stem": "Support"}
        return {"Function": d.get(p.lower(), "Refer Class 3-5 Plant Fairy")}

    @staticmethod
    def family(t):
        if "joint" in t.lower(): return {"Details": "Grandparents, parents, children live together"}
        return {"Details": "Parents and children only"}

    @staticmethod
    def matter(e):
        e = e.lower()
        if "ice" in e: return {"State": "Solid"}
        if "water" in e: return {"State": "Liquid"}
        return {"State": "Gas"}

SCIENCE_PRIMARY_REGISTRY = {
    "EVS (Classes 1-5)": Science_Primary_Classes
}
