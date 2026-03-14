"""
SigmaOS NCERT Primary Science (EVS) v10.0 — The Ultimate Series
Classes 1–5 | Foundational Environmental Studies & Science
100% stdlib, zero 3rd-party deps
"""
class Science_Primary_Classes:
    TITLE = "Primary EVS: Exhaustive Foundation Library"
    EXP_DATA = {
        "Sense Organs": ("senses", [("Organ", "eyes")]),
        "Living/Non-living": ("living", [("Grows?", "1"), ("Breathes?", "1")]),
        "Food Groups": ("food", [("Item", "Rice")]),
        "Neighborhood Help": ("neighborhood", [("Place", "Hospital")]),
        "Home Styles": ("shelter", [("Region", "Mountain")]),
        "Water Cycle": ("water_cycle", [("Step", "Rain")]),
        "Plant Anatomy": ("plant", [("Part", "Leaf")]),
        "Cleanliness Skills": ("clean", [("Task", "Hands")]),
        "Matter States": ("matter", [("Example", "Ice")]),
        "Germination": ("germination", [("Water?", "1"), ("Air?", "1"), ("Sun?", "1")]),
        "Animal Habitat": ("habitats", [("Animal", "Fish")]),
        "Transport Modes": ("transport", [("Vehicle", "Aeroplane")]),
        "Season Clothing": ("seasons", [("Season", "Summer")]),
        "Cycle of Day": ("daynight", [("Sun visible?", "1")]),
        "Family Roles": ("family", [("Role", "Father")]),
        "Animal Sounds": ("voices", [("Animal", "Dog")]),
        "Habit Checker": ("habits", [("Action", "Brushing teeth")]),
        "Body Parts": ("organs", [("Part", "Heart")]),
        "Pollution": ("pollution", [("Source", "Smoke")]),
        "Safety Rule": ("safety", [("Status", "Red Light")]),
    }

    @staticmethod
    def senses(o):
        d = {"eyes":"Vision", "ears":"Hearing", "nose":"Smell", "tongue":"Taste", "skin":"Touch"}
        return {"Role": d.get(o.lower(), "Sensing")}

    @staticmethod
    def living(g, b):
        return {"Result": "LIVING" if int(g) and int(b) else "NON-LIVING"}

    @staticmethod
    def food(i):
        i = i.lower()
        if i in ["rice", "wheat"]: return {"Group": "Energy"}
        if i in ["milk", "egg"]: return {"Group": "Body"}
        return {"Group": "Protective"}

    @staticmethod
    def neighborhood(p):
        d = {"hospital": "Treat", "bank": "Safe", "post": "Letters"}
        return {"Duty": d.get(p.lower(), "Help")}

    @staticmethod
    def shelter(a):
        d = {"mountain": "Sloping", "desert": "Mud", "river": "Boat"}
        return {"Roof": d.get(a.lower(), "Flat")}

    @staticmethod
    def water_cycle(p):
        return {"Process": p}

    @staticmethod
    def plant(p):
        d = {"root": "Water", "leaf": "Food", "stem": "Support"}
        return {"Duty": d.get(p.lower(), "Growth")}

    @staticmethod
    def clean(t):
        return {"Guideline": "Cleanliness leads to health"}

    @staticmethod
    def matter(e):
        e = e.lower()
        if "ice" in e: return {"State": "Solid"}
        if "water" in e: return {"State": "Liquid"}
        return {"State": "Gas"}

    @staticmethod
    def germination(w, a, s):
        if int(w) and int(a) and int(s): return {"Result": "Grows!"}
        return {"Result": "Fails"}

    @staticmethod
    def habitats(a):
        d = {"fish": "Water", "monkey": "Tree", "lion": "Land", "camel": "Desert"}
        return {"Home": d.get(a.lower(), "Forest")}

    @staticmethod
    def transport(v):
        d = {"car": "Land", "boat": "Water", "plane": "Air"}
        return {"Path": d.get(v.lower(), "Land")}

    @staticmethod
    def seasons(s):
        d = {"summer": "Cotton", "winter": "Woolen", "monsoon": "Rubber"}
        return {"Cloth": d.get(s.lower(), "Cloth")}

    @staticmethod
    def daynight(s):
        return {"Active": "Day" if int(s) else "Night"}

    @staticmethod
    def family(r):
        d = {"father":"Parent", "mother":"Parent", "brother":"Sibling", "sister":"Sibling"}
        return {"Relationship": d.get(r.lower(), "Relative")}

    @staticmethod
    def voices(a):
        d = {"dog":"Bark", "cat":"Meow", "lion":"Roar", "cow":"Moo"}
        return {"Sound": d.get(a.lower(), "Noise")}

    @staticmethod
    def habits(a):
        good = ["brushing", "bathing", "washing", "studying"]
        return {"Quality": "GOOD" if any(x in a.lower() for x in good) else "NEEDS IMPROVEMENT"}

    @staticmethod
    def organs(p):
        d = {"heart": "Pumps Blood", "lungs": "Breathe", "stomach": "Digests"}
        return {"Function": d.get(p.lower(), "Supports Life")}

    @staticmethod
    def pollution(s):
        s = s.lower()
        if "smoke" in s: return {"Type": "Air Pollution"}
        if "garbage" in s: return {"Type": "Land Pollution"}
        if "noise" in s: return {"Type": "Noise Pollution"}
        return {"Type": "Environmental Harm"}

    @staticmethod
    def safety(s):
        s = s.lower()
        if "red" in s: return {"Action": "STOP"}
        if "green" in s: return {"Action": "GO"}
        if "yellow" in s: return {"Action": "WAIT"}
        return {"Action": "Be Careful"}

SCIENCE_PRIMARY_REGISTRY = {
    "Primary EVS (1-5)": Science_Primary_Classes
}
