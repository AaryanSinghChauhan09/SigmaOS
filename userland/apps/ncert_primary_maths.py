"""
SigmaOS NCERT Primary Mathematics Lab v10.0 — The Ultimate Series
Classes 1–5 | Foundational Arithmetic, Shapes & Patterns
100% stdlib, zero 3rd-party deps
"""
class Maths_Primary:
    TITLE = "Primary Math Foundations: Exhaustive Logic"
    EXP_DATA = {
        "Number Express": ("words", [("Number", "7")]),
        "Addition carry": ("add", [("A", "45"), ("B", "56")]),
        "Multiplication (Repeated)": ("mul", [("A", "5"), ("B", "6")]),
        "Fraction Sharing": ("sharing", [("Shaded", "1"), ("Total", "4")]),
        "Pattern Sequence": ("pattern", [("Seq", "2,4,6")]),
        "Weight Logic": ("weight", [("kg", "2"), ("g", "500")]),
        "Handy Clock": ("clock", [("H", "3"), ("M", "30")]),
        "Pointy Shapes": ("shapes", [("Shape", "Square")]),
        "Greater/Lesser": ("compare", [("X", "25"), ("Y", "18")]),
        "Money Calc": ("money", [("Rs", "10"), ("Paise", "50"), ("Count", "3")]),
        "Sorting Order": ("sort", [("Data", "5,2,9,1,7")]),
        "Equal Sharing": ("divide", [("Total", "20"), ("Friends", "4")]),
        "Skip Count": ("skip", [("Start", "2"), ("Step", "3"), ("Count", "5")]),
        "Roman Digit": ("roman", [("N", "9")]),
        "Perimeter": ("perimeter", [("Shape", "Square"), ("Side", "5")]),
    }

    @staticmethod
    def words(n):
        d = {0:"Zero", 1:"One", 2:"Two", 3:"Three", 4:"Four", 5:"Five", 6:"Six", 7:"Seven", 8:"Eight", 9:"Nine"}
        return {"Word": d.get(int(n), "Big Number")}

    @staticmethod
    def add(a, b):
        return {"Sum": int(a)+int(b), "Carry": (int(a)%10 + int(b)%10) >= 10}

    @staticmethod
    def mul(a, b):
        return {"Product": int(a)*int(b)}

    @staticmethod
    def sharing(s, t):
        return {"Fraction": f"{s}/{t}", "Note": "Quarter" if s/t==0.25 else "Half" if s/t==0.5 else "Part"}

    @staticmethod
    def pattern(s):
        n = [int(x.strip()) for x in s.split(",")]
        return {"Next": n[-1] + (n[1]-n[0])}

    @staticmethod
    def weight(kg, g):
        return {"Total (g)": int(kg)*1000 + int(g)}

    @staticmethod
    def clock(h, m):
        return {"Display": f"{int(h)}:{int(m):02d}"}

    @staticmethod
    def shapes(s):
        d = {"square":4, "triangle":3, "circle":0}
        return {"Sides": d.get(s.lower(), "?")}

    @staticmethod
    def compare(x, y):
        x, y = int(x), int(y)
        if x > y: return {"Sign": ">", "Result": f"{x} is GREATER than {y}"}
        if x < y: return {"Sign": "<", "Result": f"{x} is LESS than {y}"}
        return {"Sign": "=", "Result": "EQUAL"}

    @staticmethod
    def money(rs, ps, c):
        total = (int(rs)*100 + int(ps)) * int(c)
        return {"Total Rs": total/100, "Notes/Coins": f"Total {c} items cost Rs {total/100}"}

    @staticmethod
    def sort(s):
        n = [int(x.strip()) for x in s.split(",")]
        return {"Ascending": sorted(n), "Descending": sorted(n, reverse=True)}

    @staticmethod
    def divide(t, f):
        t, f = int(t), int(f)
        return {"Each child gets": t//f, "Leftover": t%f}

    @staticmethod
    def skip(s, step, c):
        res = [int(s) + i*int(step) for i in range(int(c))]
        return {"Sequence": ", ".join(map(str, res))}

    @staticmethod
    def roman(n):
        d = {1:"I", 2:"II", 3:"III", 4:"IV", 5:"V", 6:"VI", 7:"VII", 8:"VIII", 9:"IX", 10:"X"}
        return {"Roman": d.get(int(n), "Out of Range")}

    @staticmethod
    def perimeter(sh, s):
        sh = sh.lower(); s = int(s)
        if "square" in sh: return {"Perimeter": 4*s}
        if "triangle" in sh: return {"Perimeter": 3*s}
        return {"Perimeter": "Unknown Shape"}

PRIMARY_MATHS_REGISTRY = {
    "Primary Math (1-5)": Maths_Primary
}
