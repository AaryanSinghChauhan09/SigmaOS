"""
SigmaOS NCERT Primary Science Lab v3.0
Classes 1–5 | EVS & Science Foundations
100% stdlib, zero 3rd-party deps
"""
class Science_Primary_Classes:
    TITLE = "Classes 1–5 – EVS, Science Foundations & Environment"
    EXP_DATA = {
        "Sense Organs": ("senses", [("Organ", "eyes")]),
        "Living vs Non-living": ("living", [("Does it grow? (1/0)", "1"), ("Does it breathe? (1/0)", "1")]),
        "Water Cycle": ("water_cycle", [("Step", "Evaporation")]),
        "Food Groups": ("food", [("Item", "Rice")]),
        "Simple Machines": ("machines", [("Tool", "Scissors")]),
        "Animal Lifecycles": ("lifecycle", [("Animal (Frog/Butterfly)", "Frog")]),
        "States of Matter": ("matter", [("Example", "Ice")]),
    }

    @staticmethod
    def senses(organ):
        d = {"eyes":"See", "ears":"Hear", "nose":"Smell", "tongue":"Taste", "skin":"Touch"}
        return {"Function": d.get(organ.lower(), "Refer Class 3 EVS")}

    @staticmethod
    def living(g, b):
        if int(g) and int(b): return {"Result": "Living Being"}
        return {"Result": "Non-living Object"}

    @staticmethod
    def water_cycle(step):
        d = {
            "evaporation": "Water turns to vapor due to Sun's heat",
            "condensation": "Vapor cools down to form clouds",
            "precipitation": "Water falls back as rain or snow",
            "collection": "Water gathers in rivers, lakes, and oceans"
        }
        return {"Description": d.get(step.lower(), "Refer Class 4 EVS")}

    @staticmethod
    def food(item):
        i = item.lower()
        if i in ["rice", "wheat", "potato"]: return {"Group": "Energy-giving (Carbohydrates)"}
        if i in ["dal", "egg", "meat", "milk"]: return {"Group": "Body-building (Proteins)"}
        if i in ["fruits", "vegetables"]: return {"Group": "Protective (Vitamins/Minerals)"}
        return {"Group": "Refer Class 3-5 Food sections"}

    @staticmethod
    def machines(tool):
        t = tool.lower()
        if "scissor" in t or "see-saw" in t: return {"Type": "Lever"}
        if "flagpole" in t: return {"Type": "Pulley"}
        if "slide" in t: return {"Type": "Inclined Plane"}
        return {"Type": "Simple Machine (Refer Class 5 Science)"}

    @staticmethod
    def lifecycle(animal):
        a = animal.lower()
        if "frog" in a: return {"Stages": "Egg -> Tadpole -> Froglet -> Adult Frog"}
        if "butterfly" in a: return {"Stages": "Egg -> Larva (Caterpillar) -> Pupa (Chrysalis) -> Adult Butterfly"}
        return {"Info": "Refer Class 4-5 EVS"}

    @staticmethod
    def matter(ex):
        ex = ex.lower()
        if "ice" in ex: return {"State": "Solid"}
        if "water" in ex: return {"State": "Liquid"}
        if "steam" in ex: return {"State": "Gas"}
        return {"State": "Refer Ch-3 Matter"}

# Registry
SCIENCE_PRIMARY_REGISTRY = {
    "Classes 1-5": Science_Primary_Classes
}
