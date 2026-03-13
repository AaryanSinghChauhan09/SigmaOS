"""
SigmaOS NCERT Primary Mathematics Lab v6.0 — The Ultimate Series
Classes 1–5 | Foundational Arithmetic, Shapes & Patterns
100% stdlib, zero 3rd-party deps
"""
class Maths_Primary:
    TITLE = "Primary Math Foundations: Counting, Patterns & Geometry"
    EXP_DATA = {
        "Number Names (Class 1)": ("words", [("Number", "7")]),
        "Multiplication (Class 3)": ("mul", [("A", "5"), ("B", "6")]),
        "Area (Grid Count)": ("area", [("Length", "4"), ("Width", "3")]),
        "Money Math": ("money", [("Item A Cost", "15.50"), ("Item B Cost", "20.25")]),
        "Pattern Completion": ("pattern", [("Sequence (comma)", "2,4,6")]),
        "Fraction Shading": ("fraction", [("Shaded", "1"), ("Total", "4")]),
        "Measurement (Weight)": ("weight", [("A (kg)", "2"), ("B (g)", "500")]),
    }

    @staticmethod
    def words(n):
        d = {0:"Zero", 1:"One", 2:"Two", 3:"Three", 4:"Four", 5:"Five", 6:"Six", 7:"Seven", 8:"Eight", 9:"Nine"}
        return {"Word": d.get(int(n), str(n))}

    @staticmethod
    def mul(a, b):
        return {"Result": int(a)*int(b), "Repeated Addition": "+".join([str(a)]*int(b))}

    @staticmethod
    def area(l, w):
        return {"Area (Total Squares)": int(l)*int(w)}

    @staticmethod
    def money(a, b):
        return {"Total": f"₹ {float(a)+float(b)}"}

    @staticmethod
    def pattern(s):
        nums = [int(x.strip()) for x in s.split(",")]
        diff = nums[1]-nums[0]
        return {"Next": nums[-1]+diff, "Rule": "+"+str(diff)}

    @staticmethod
    def fraction(s, t):
        return {"Text": f"{int(s)} out of {int(t)}", "Percent": f"{(int(s)/int(t))*100}%"}

    @staticmethod
    def weight(kg, g):
        return {"Total (grams)": int(kg)*1000 + int(g)}

PRIMARY_MATHS_REGISTRY = {
    "Foundation (Classes 1-5)": Maths_Primary
}
