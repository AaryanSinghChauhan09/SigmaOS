"""
SigmaOS NCERT Primary Mathematics Lab v7.0 — The Ultimate Lab Manual
Classes 1–5 | Foundational Arithmetic, Shapes & Patterns
100% stdlib, zero 3rd-party deps
"""
class Maths_Primary:
    TITLE = "Primary Math Foundations: Early Logic"
    EXP_DATA = {
        "Number Names (Class 1)": ("words", [("Number", "7")]),
        "Addition carry/sum": ("add", [("A", "45"), ("B", "56")]),
        "Multiplication (Class 3)": ("mul", [("A", "5"), ("B", "6")]),
        "Sharing Fractions": ("sharing", [("Pieces Shaded", "1"), ("Total Pieces", "4")]),
        "Pattern Hunter": ("pattern", [("Sequence (comma)", "2,4,6")]),
        "Measurement Weight": ("weight", [("A (kg)", "2"), ("B (g)", "500")]),
        "Clock Time": ("clock", [("Hours", "3"), ("Minutes", "30")]),
        "Shapes Corners": ("shapes", [("Shape", "Square")]),
    }

    @staticmethod
    def words(n):
        d = {0:"Zero", 1:"One", 2:"Two", 3:"Three", 4:"Four", 5:"Five", 6:"Six", 7:"Seven", 8:"Eight", 9:"Nine"}
        return {"Word": d.get(int(n), str(n))}

    @staticmethod
    def add(a, b):
        return {"Sum": int(a)+int(b), "Carry Logic": "Tens place shifted" if (int(a)%10 + int(b)%10) >= 10 else "No carry"}

    @staticmethod
    def mul(a, b):
        return {"Result": int(a)*int(b), "Multiplication type": "Repeated Addition"}

    @staticmethod
    def sharing(s, t):
        return {"Expression": f"{s}/{t}", "Percentage": f"{(int(s)/int(t))*100}%", "Note": "1/2=Half, 1/4=Quarter"}

    @staticmethod
    def pattern(s):
        nums = [int(x.strip()) for x in s.split(",")]
        diff = nums[1]-nums[0]
        return {"Next": nums[-1]+diff, "Rule": "Add "+str(diff)}

    @staticmethod
    def weight(kg, g):
        return {"Total grams": int(kg)*1000 + int(g), "Visual": f"{kg}kg and {g}g"}

    @staticmethod
    def clock(h, m):
        return {"Time Display": f"{int(h)}:{int(m):02d}", "Angle": f"{abs(30*int(h)-5.5*int(m))} deg"}

    @staticmethod
    def shapes(s):
        d = {"square": 4, "triangle": 3, "pentagon": 5}
        return {"Corners": d.get(s.lower(), "Refer Class 2"), "Sides": d.get(s.lower(), "Refer Class 2")}

PRIMARY_MATHS_REGISTRY = {
    "Primary Math (1-5)": Maths_Primary
}
