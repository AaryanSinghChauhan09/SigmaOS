"""
SigmaOS NCERT Primary Science (EVS) v7.0 — The Ultimate Lab Manual
Classes 1–5 | Foundational Environmental Studies & Science
100% stdlib, zero 3rd-party deps
"""
class Science_Primary_Classes:
    TITLE = "Primary EVS: Nature, Body, Food & Community"
    EXP_DATA = {
        "Sense Organs": ("senses", [("Organ (eyes/ears/nose/tongue/skin)", "eyes")]),
        "Living vs Non-living": ("living", [("Does it grow? (1/0)", "1"), ("Does it breathe? (1/0)", "1")]),
        "Food We Eat": ("food", [("Item (Rice/Milk/Apple)", "Rice")]),
        "Our Neighborhood": ("neighborhood", [("Place (Bank/Hospital/Post)", "Hospital")]),
        "Shelter Types": ("shelter", [("Area (Desert/Mountain/River)", "Mountain")]),
        "Water Cycle Basics": ("water_cycle", [("Process (Rain/Clouds)", "Rain")]),
        "Parts of a Plant": ("plant", [("Part (Root/Stem/Leaf/Flower)", "Leaf")]),
        "Cleanliness Skills": ("clean", [("Task (Hands/Brushing)", "Hands")]),
        "States of Matter": ("matter", [("Example (Ice/Water/Steam)", "Ice")]),
    }

    @staticmethod
    def senses(o):
        d = {"eyes":"Vision/Colors", "ears":"Hearing", "nose":"Smell", "tongue":"Taste", "skin":"Touch/Heat"}
        return {"Used for": d.get(o.lower(), "Refer Class 3 EVS")}

    @staticmethod
    def living(g, b):
        if int(g) and int(b): return {"Result": "LIVING BEING", "Inference": "Grows and Breathes"}
        return {"Result": "NON-LIVING", "Inference": "Stationary and inanimate"}

    @staticmethod
    def food(i):
        i = i.lower()
        if i in ["rice", "wheat", "potato"]: return {"Group": "Energy-giving (Carbohydrates)"}
        if i in ["milk", "egg", "dal"]: return {"Group": "Body-building (Proteins)"}
        return {"Group": "Protective (Vitamins)"}

    @staticmethod
    def neighborhood(p):
        d = {"hospital": "Doctor & Nurses treat sick people", "bank": "Safe place for money", "post": "Send letters and parcels"}
        return {"Info": d.get(p.lower(), "Refer Class 3 Neighborhood")}

    @staticmethod
    def shelter(a):
        d = {"mountain": "Sloping roofs for snow", "desert": "Thick walls to stay cool", "river": "Houseboats"}
        return {"Style": d.get(a.lower(), "Refer Class 5 Shelter")}

    @staticmethod
    def water_cycle(p):
        d = {"rain": "Precipitation from clouds", "clouds": "Vapor condensing in air"}
        return {"Step": d.get(p.lower(), "Refer Class 4 EVS")}

    @staticmethod
    def plant(p):
        d = {"root": "Holds plant, absorbs water", "stem": "Supports, carries nutrients", "leaf": "Kitchen (makes food)"}
        return {"Role": d.get(p.lower(), "Refer Class 3 Plant Fairy")}

    @staticmethod
    def clean(t):
        d = {"hands": "Wash before eating to kill germs", "brushing": "Prevents cavities"}
        return {"Guideline": d.get(t.lower(), "Refer Class 1 Community")}

    @staticmethod
    def matter(e):
        e = e.lower()
        if "ice" in e: return {"State": "Solid", "Atom Spacing": "Very Close"}
        if "water" in e: return {"State": "Liquid", "Atom Spacing": "Moderate"}
        return {"State": "Gas", "Atom Spacing": "Far Apart"}

SCIENCE_PRIMARY_REGISTRY = {
    "Primary EVS (1-5)": Science_Primary_Classes
}
