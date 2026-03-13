"""
SigmaOS NCERT Primary Science (EVS) v9.0 — The Comprehensive series
Classes 1–5 | Foundational Environmental Studies & Science
100% stdlib, zero 3rd-party deps
"""
class Science_Primary_Classes:
    TITLE = "Primary EVS: Exhaustive Foundation Library"
    EXP_DATA = {
        "Sense Organs": ("senses", [("Organ", "eyes")]),
        "Living vs Non-living": ("living", [("Grows?", "1"), ("Breathes?", "1")]),
        "Food We Eat": ("food", [("Item", "Rice")]),
        "Neighborhood Help": ("neighborhood", [("Place", "Hospital")]),
        "Shelter (Homes)": ("shelter", [("Region", "Mountain")]),
        "Water Cycle": ("water_cycle", [("Step", "Rain")]),
        "Plant Parts": ("plant", [("Part", "Leaf")]),
        "Cleanliness": ("clean", [("Task", "Hands")]),
        "States of Matter": ("matter", [("Example", "Ice")]),
        "Seed Germination": ("germination", [("Water?", "1"), ("Air?", "1"), ("Sun?", "1")]),
        "Animal Habitats": ("habitats", [("Animal", "Fish")]),
        "Transport Modes": ("transport", [("Vehicle", "Aeroplane")]),
        "Seasons & Clothes": ("seasons", [("Season", "Summer")]),
        "Day & Night": ("daynight", [("Is Sun out?", "1")]),
    }

    @staticmethod
    def senses(o):
        d = {"eyes":"Vision", "ears":"Hearing", "nose":"Smell", "tongue":"Taste", "skin":"Touch"}
        return {"Used for": d.get(o.lower(), "Refer Class 3")}

    @staticmethod
    def living(g, b):
        if int(g) and int(b): return {"Result": "LIVING"}
        return {"Result": "NON-LIVING"}

    @staticmethod
    def food(i):
        i = i.lower()
        if i in ["rice", "wheat"]: return {"Group": "Energy"}
        if i in ["milk", "egg"]: return {"Group": "Body-building"}
        return {"Group": "Protective"}

    @staticmethod
    def neighborhood(p):
        d = {"hospital": "Treats Sick", "bank": "Saves Money", "post": "Mails letters"}
        return {"Info": d.get(p.lower(), "Neighborhood help")}

    @staticmethod
    def shelter(a):
        d = {"mountain": "Sloping Roof", "desert": "Thick Mud", "river": "Houseboat"}
        return {"Style": d.get(a.lower(), "Shelter")}

    @staticmethod
    def water_cycle(p):
        return {"Step": p}

    @staticmethod
    def plant(p):
        d = {"root": "Water absorb", "leaf": "Food factory", "stem": "Support"}
        return {"Role": d.get(p.lower(), "Plant part")}

    @staticmethod
    def clean(t):
        return {"Guideline": "Cleanliness is next to godliness"}

    @staticmethod
    def matter(e):
        e = e.lower()
        if "ice" in e: return {"State": "Solid"}
        if "water" in e: return {"State": "Liquid"}
        return {"State": "Gas"}

    @staticmethod
    def germination(w, a, s):
        if int(w) and int(a) and int(s): return {"Result": "SUCCESS", "Note": "Seed grows into a sapling"}
        return {"Result": "FAILURE", "Note": "Seeds need Water, Air, and Sunlight"}

    @staticmethod
    def habitats(a):
        d = {"fish": "Aquatic", "monkey": "Arboreal", "lion": "Terrestrial", "camel": "Desert"}
        return {"Habitat": d.get(a.lower(), "Natural home")}

    @staticmethod
    def transport(v):
        d = {"car": "Land", "boat": "Water", "aeroplane": "Air", "train": "Land"}
        return {"Mode": d.get(v.lower(), "Transportation")}

    @staticmethod
    def seasons(s):
        d = {"summer": "Cotton clothes", "winter": "Woolen clothes", "monsoon": "Raincoats"}
        return {"Clothing": d.get(s.lower(), "Climate clothing")}

    @staticmethod
    def daynight(s):
        if int(s): return {"Status": "DAY", "Activity": "School & Work"}
        return {"Status": "NIGHT", "Activity": "Sleep & Rest"}

SCIENCE_PRIMARY_REGISTRY = {
    "Primary EVS (1-5)": Science_Primary_Classes
}
