"""
SigmaOS NCERT Virtual Lab v1.0
=================================
Comprehensive NCERT simulator: Physics, Chemistry, Biology, Mathematics
Classes 1 – 12 | 100% Offline | Zero 3rd-party dependencies
Inspired by: DIKSHA Virtual Labs, VLab (IIT), OLabs (Amrita), KATBOOK
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import math, random, time, json, threading

# ─── Palette ──────────────────────────────────────────────────────────────────
PAL = {
    "bg": "#0D0F18", "sidebar": "#13162A", "card": "#1A1E30",
    "accent": "#6C63FF", "success": "#00D26A", "danger": "#FF4D4D",
    "warning": "#FFA500", "text": "#E8E8F0", "dim": "#9090A0",
    "border": "#2A2D45", "ph": "#3B82F6", "ch": "#22C55E",
    "bi": "#EC4899", "ma": "#F59E0B"
}

# ══════════════════════════════════════════════════════════════════════════════
# PHYSICS ENGINE  (Classes 6–12)
# ══════════════════════════════════════════════════════════════════════════════
class PhysicsLab:
    EXPERIMENTS = {
        "Ohm's Law": "ph_ohm",
        "Projectile Motion": "ph_proj",
        "Simple Pendulum": "ph_pend",
        "Lens Formula": "ph_lens",
        "Snell's Law": "ph_snell",
        "Newton's 2nd Law": "ph_newton2",
        "Boyle's Law": "ph_boyle",
        "Kirchhoff's Laws": "ph_kirch",
        "Doppler Effect": "ph_doppler",
        "Wave Interference": "ph_wave",
        "Photoelectric Effect": "ph_photo",
        "Radioactive Decay": "ph_radio",
        "Coulomb's Law": "ph_coulomb",
        "Magnetic Force": "ph_magnet",
    }

    @staticmethod
    def ohms_law(V=None, I=None, R=None):
        if V is None: return {"V": round(I*R, 4), "unit": "Volts", "law": "V = IR"}
        if I is None: return {"I": round(V/R, 4), "unit": "Amperes", "law": "I = V/R"}
        if R is None: return {"R": round(V/I, 4), "unit": "Ohms", "law": "R = V/I"}

    @staticmethod
    def projectile_motion(u, angle_deg, h0=0):
        g = 9.8; a = math.radians(angle_deg)
        ux, uy = u*math.cos(a), u*math.sin(a)
        t_flight = (uy + math.sqrt(uy**2 + 2*g*h0))/g
        R = round(ux*t_flight, 3); H = round(uy**2/(2*g)+h0, 3)
        return {"Range_m": R, "Max_Height_m": H, "Time_s": round(t_flight,3),
                "Vx": round(ux,3), "Vy_initial": round(uy,3)}

    @staticmethod
    def simple_pendulum(L):
        g=9.8; T=round(2*math.pi*math.sqrt(L/g),4)
        f=round(1/T,4)
        return {"T_period_s": T, "Frequency_Hz": f, "L_m": L, "g": g}

    @staticmethod
    def lens_formula(f=None, u=None, v=None):
        try:
            if v is None: v=round((f*u)/(u-f),4)
            elif u is None: u=round((f*v)/(v-f),4)
            elif f is None: f=round((u*v)/(u+v),4)
            m = round(-v/u,4) if u else 0
            return {"f_cm": f,"u_cm": u,"v_cm": v,"magnification": m,"law":"1/f=1/v-1/u"}
        except ZeroDivisionError: return {"error": "Division by zero"}

    @staticmethod
    def snells_law(n1, theta1_deg, n2=None, theta2_deg=None):
        t1=math.radians(theta1_deg)
        if n2 is None:
            t2=math.radians(theta2_deg)
            n2=round(n1*math.sin(t1)/math.sin(t2),4)
            return {"n2": n2}
        sin_t2=n1*math.sin(t1)/n2
        if abs(sin_t2)>1: return {"error":"Total Internal Reflection"}
        t2=round(math.degrees(math.asin(sin_t2)),4)
        return {"theta2_deg": t2, "n1": n1, "n2": n2}

    @staticmethod
    def newtons_second_law(m, a): return {"F_N": round(m*a,4), "law": "F = ma"}

    @staticmethod
    def boyles_law(P1, V1, P2=None, V2=None):
        if P2 is None: P2=round(P1*V1/V2,4); return {"P2": P2,"law":"P1V1=P2V2"}
        V2=round(P1*V1/P2,4); return {"V2": V2,"law":"P1V1=P2V2"}

    @staticmethod
    def coulombs_law(q1, q2, r): k=9e9; F=round(k*q1*q2/r**2,4); return {"F_N":F,"law":"F=kq1q2/r²"}

    @staticmethod
    def photoelectric_effect(freq_Hz):
        h=6.626e-34; phi={'Na':2.27,'K':2.29,'Ca':2.87,'Zn':4.31}
        E=h*freq_Hz; res={}
        for metal,wf in phi.items():
            wf_J=wf*1.6e-19
            KE=E-wf_J
            res[metal]={"KE_J":round(KE,25),"emitted":KE>0}
        return {"photon_E_J":round(E,25),"metals":res}

    @staticmethod
    def radioactive_decay(N0, half_life, t):
        decay_const=math.log(2)/half_life
        N=round(N0*math.exp(-decay_const*t),4)
        activity=round(decay_const*N,4)
        return {"N_remaining":N,"N_decayed":round(N0-N,4),"Activity_Bq":activity}

# ══════════════════════════════════════════════════════════════════════════════
# CHEMISTRY ENGINE  (Classes 6–12)
# ══════════════════════════════════════════════════════════════════════════════
class ChemistryLab:
    EXPERIMENTS = {
        "Molar Mass Calculator": "ch_molar",
        "pH Calculator": "ch_ph",
        "Ideal Gas Law": "ch_gas",
        "Stoichiometry": "ch_stoich",
        "Molarity": "ch_mol",
        "Titration (Acid-Base)": "ch_titration",
        "Electrolysis": "ch_electro",
        "Periodic Table Query": "ch_periodic",
        "Bond Energy": "ch_bond",
        "Enthalpy Change": "ch_enthalpy",
        "Rate of Reaction": "ch_rate",
        "Equilibrium Constant": "ch_keq",
    }

    ELEMENTS = {
        "H":{"mass":1.008,"group":1,"period":1,"symbol":"H"},
        "He":{"mass":4.003,"group":18,"period":1},
        "Li":{"mass":6.941,"group":1,"period":2},
        "C":{"mass":12.011,"group":14,"period":2},
        "N":{"mass":14.007,"group":15,"period":2},
        "O":{"mass":15.999,"group":16,"period":2},
        "Na":{"mass":22.990,"group":1,"period":3},
        "Mg":{"mass":24.305,"group":2,"period":3},
        "Al":{"mass":26.982,"group":13,"period":3},
        "S":{"mass":32.06,"group":16,"period":3},
        "Cl":{"mass":35.453,"group":17,"period":3},
        "K":{"mass":39.098,"group":1,"period":4},
        "Ca":{"mass":40.078,"group":2,"period":4},
        "Fe":{"mass":55.845,"group":8,"period":4},
        "Cu":{"mass":63.546,"group":11,"period":4},
        "Zn":{"mass":65.38,"group":12,"period":4},
        "Ag":{"mass":107.868,"group":11,"period":5},
        "Au":{"mass":196.967,"group":11,"period":6},
    }

    @staticmethod
    def molar_mass(formula):
        import re
        elements = re.findall(r'([A-Z][a-z]?)(\d*)', formula)
        total=0
        breakdown={}
        for sym,cnt in elements:
            if not sym: continue
            data=ChemistryLab.ELEMENTS.get(sym)
            if not data: continue
            n=int(cnt) if cnt else 1
            m=round(data['mass']*n,4)
            breakdown[sym]={"count":n,"mass_each":data['mass'],"total":m}
            total+=m
        return {"formula":formula,"molar_mass_g_mol":round(total,4),"breakdown":breakdown}

    @staticmethod
    def ph_calculator(concentration, is_acid=True):
        pH=round(-math.log10(concentration),4) if is_acid else round(14+math.log10(concentration),4)
        return {"pH":pH,"pOH":round(14-pH,4),"H+":concentration if is_acid else round(10**-pH,4),
                "nature":"Acidic" if pH<7 else ("Basic" if pH>7 else "Neutral")}

    @staticmethod
    def ideal_gas_law(P=None,V=None,n=None,T=None):
        R=0.0821  # L·atm/mol·K
        vals={"P":P,"V":V,"n":n,"T":T}
        missing=[k for k,v in vals.items() if v is None]
        if len(missing)!=1: return {"error":"Provide exactly 3 of P,V,n,T"}
        m=missing[0]
        known={k:v for k,v in vals.items() if v is not None}
        if m=="P": res=round(known["n"]*R*known["T"]/known["V"],4)
        elif m=="V": res=round(known["n"]*R*known["T"]/known["P"],4)
        elif m=="n": res=round(known["P"]*known["V"]/(R*known["T"]),4)
        else: res=round(known["P"]*known["V"]/(known["n"]*R),4)
        return {m:res,"R":"0.0821 L·atm/mol·K","law":"PV=nRT"}

    @staticmethod
    def molarity(moles, volume_L): return {"M":round(moles/volume_L,4),"unit":"mol/L"}

    @staticmethod
    def enthalpy_change(products_kJ, reactants_kJ):
        dH=round(products_kJ-reactants_kJ,4)
        return {"delta_H_kJ":dH,"type":"Exothermic" if dH<0 else "Endothermic"}

    @staticmethod
    def rate_of_reaction(delta_conc, delta_time):
        return {"rate":round(abs(delta_conc/delta_time),6),"unit":"mol/L/s"}

# ══════════════════════════════════════════════════════════════════════════════
# BIOLOGY ENGINE  (Classes 6–12)
# ══════════════════════════════════════════════════════════════════════════════
class BiologyLab:
    EXPERIMENTS = {
        "Genetics (Mendel)": "bi_mendel",
        "Photosynthesis Rate": "bi_photo",
        "Osmosis / Diffusion": "bi_osmosis",
        "Human Heartbeat Calc": "bi_heart",
        "Cell Division (Mitosis)": "bi_mitosis",
        "Digestive System": "bi_digest",
        "BMI Calculator": "bi_bmi",
        "Microscope Magnification": "bi_micro",
        "Blood Group Genetics": "bi_blood",
        "Ecosystem Energy Flow": "bi_eco",
    }

    @staticmethod
    def mendel_cross(parent1, parent2):
        combos=[a+b for a in parent1 for b in parent2]
        from collections import Counter
        c=Counter(combos)
        total=len(combos)
        return {"offspring":{k:f"{v}/{total}"for k,v in c.items()},
                "dominant_ratio":f"{sum(v for k,v in c.items() if k[0].isupper() or k[1].isupper())}/{total}"}

    @staticmethod
    def photosynthesis_rate(light_intensity, CO2_ppm, temp_C):
        base=0.5; rate=base*(light_intensity/1000)*(CO2_ppm/400)*(1+0.02*(temp_C-25))
        return {"rate_mg_O2_per_hr":round(rate,4),"limiting_factor":
                "Light" if light_intensity<500 else ("CO2" if CO2_ppm<200 else "Temperature")}

    @staticmethod
    def osmosis(cell_conc, solution_conc):
        if solution_conc>cell_conc: result="Plasmolysis (water out)"
        elif solution_conc<cell_conc: result="Turgidity (water in)"
        else: result="Isotonic – No net movement"
        return {"result":result,"gradient":round(solution_conc-cell_conc,4)}

    @staticmethod
    def bmi(weight_kg, height_m):
        bmi=round(weight_kg/height_m**2,2)
        cat="Underweight" if bmi<18.5 else("Normal" if bmi<25 else("Overweight" if bmi<30 else "Obese"))
        return {"BMI":bmi,"category":cat}

    @staticmethod
    def microscope_magnification(objective, eyepiece): return {"total": objective*eyepiece}

    @staticmethod
    def heart_rate_zones(age):
        max_hr=220-age
        return {"Max_HR":max_hr,"Fat_Burn":f"{int(max_hr*0.6)}-{int(max_hr*0.7)} bpm",
                "Cardio":f"{int(max_hr*0.7)}-{int(max_hr*0.85)} bpm",
                "Peak":f"{int(max_hr*0.85)}-{max_hr} bpm"}

# ══════════════════════════════════════════════════════════════════════════════
# MATHEMATICS ENGINE  (Classes 1–12)
# ══════════════════════════════════════════════════════════════════════════════
class MathLab:
    EXPERIMENTS = {
        "Quadratic Solver": "ma_quad",
        "Matrix Operations": "ma_matrix",
        "Statistics (Mean/SD)": "ma_stats",
        "Trigonometry": "ma_trig",
        "Binomial Theorem": "ma_binom",
        "Integration": "ma_integ",
        "Differentiation": "ma_diff",
        "Probability": "ma_prob",
        "Geometry Calculator": "ma_geo",
        "Arithmetic Progression": "ma_ap",
        "Geometric Progression": "ma_gp",
        "Number Theory": "ma_nt",
        "Permutations & Combos": "ma_pc",
        "Complex Numbers": "ma_complex",
    }

    @staticmethod
    def quadratic(a, b, c):
        disc=b**2-4*a*c
        if disc>0: r1=round((-b+math.sqrt(disc))/(2*a),6); r2=round((-b-math.sqrt(disc))/(2*a),6); nature="Two real"
        elif disc==0: r1=r2=round(-b/(2*a),6); nature="Equal real"
        else:
            real=round(-b/(2*a),6); imag=round(math.sqrt(-disc)/(2*a),6)
            r1=f"{real}+{imag}i"; r2=f"{real}-{imag}i"; nature="Complex"
        return {"x1":r1,"x2":r2,"discriminant":disc,"nature":nature}

    @staticmethod
    def statistics(data):
        n=len(data); mean=round(sum(data)/n,6)
        var=round(sum((x-mean)**2 for x in data)/n,6)
        sd=round(math.sqrt(var),6)
        s=sorted(data); med=(s[n//2] if n%2 else (s[n//2-1]+s[n//2])/2)
        return {"n":n,"mean":mean,"median":med,"std_dev":sd,"variance":var,"min":min(data),"max":max(data)}

    @staticmethod
    def trig_values(angle_deg):
        a=math.radians(angle_deg)
        def r(x): return round(x,8)
        return {"angle_deg":angle_deg,"sin":r(math.sin(a)),"cos":r(math.cos(a)),
                "tan":r(math.tan(a)) if abs(math.cos(a))>1e-10 else "∞",
                "sec":r(1/math.cos(a)) if abs(math.cos(a))>1e-10 else "∞",
                "cosec":r(1/math.sin(a)) if abs(math.sin(a))>1e-10 else "∞",
                "cot":r(math.cos(a)/math.sin(a)) if abs(math.sin(a))>1e-10 else "∞"}

    @staticmethod
    def factorial(n): return math.factorial(int(n))

    @staticmethod
    def permutation(n, r): return math.factorial(n)//math.factorial(n-r)

    @staticmethod
    def combination(n, r): return math.factorial(n)//(math.factorial(r)*math.factorial(n-r))

    @staticmethod
    def binomial(n, x, p):
        C=MathLab.combination(n,x); q=1-p
        prob=C*(p**x)*(q**(n-x))
        return {"P(X=x)":round(prob,8),"n":n,"x":x,"p":p}

    @staticmethod
    def ap_terms(a, d, n): return {"nth_term":a+(n-1)*d,"sum_n":n*(2*a+(n-1)*d)//2}

    @staticmethod
    def gp_terms(a, r, n):
        nth=a*r**(n-1)
        total=a*(r**n-1)/(r-1) if r!=1 else a*n
        return {"nth_term":round(nth,6),"sum_n":round(total,6)}

    @staticmethod
    def complex_ops(a1, b1, a2, b2):
        add=(round(a1+a2,6),round(b1+b2,6))
        sub=(round(a1-a2,6),round(b1-b2,6))
        mul=(round(a1*a2-b1*b2,6),round(a1*b2+b1*a2,6))
        mod1=round(math.sqrt(a1**2+b1**2),6)
        mod2=round(math.sqrt(a2**2+b2**2),6)
        return {"add":f"{add[0]}+{add[1]}i","sub":f"{sub[0]}+{sub[1]}i",
                "mul":f"{mul[0]}+{mul[1]}i","|z1|":mod1,"|z2|":mod2}

    @staticmethod
    def geometry(shape, **kw):
        if shape=="circle":
            r=kw["r"]; return {"area":round(math.pi*r**2,4),"perimeter":round(2*math.pi*r,4)}
        if shape=="rectangle":
            l,w=kw["l"],kw["w"]; return {"area":l*w,"perimeter":2*(l+w)}
        if shape=="triangle":
            a,b,c=kw["a"],kw["b"],kw["c"]; s=(a+b+c)/2
            return {"area":round(math.sqrt(s*(s-a)*(s-b)*(s-c)),4),"perimeter":a+b+c}
        if shape=="sphere":
            r=kw["r"]; return {"volume":round(4/3*math.pi*r**3,4),"surface_area":round(4*math.pi*r**2,4)}
        if shape=="cylinder":
            r,h=kw["r"],kw["h"]
            return {"volume":round(math.pi*r**2*h,4),"surface_area":round(2*math.pi*r*(r+h),4)}

# ══════════════════════════════════════════════════════════════════════════════
# TKINTER GUI
# ══════════════════════════════════════════════════════════════════════════════
class NCERTVirtualLab(tk.Tk):
    SUBJECTS = {
        "⚛ Physics": (PAL["ph"], PhysicsLab),
        "🧪 Chemistry": (PAL["ch"], ChemistryLab),
        "🧬 Biology": (PAL["bi"], BiologyLab),
        "📐 Mathematics": (PAL["ma"], MathLab),
    }

    def __init__(self, kernel=None):
        super().__init__()
        self.kernel = kernel
        self.title("SigmaOS • NCERT Virtual Lab — Classes 1–12")
        self.geometry("1280x820")
        self.configure(bg=PAL["bg"])
        self.current_subject = None
        self.current_color = PAL["accent"]
        self._setup_styles()
        self._build_ui()

    def _setup_styles(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure("TNotebook", background=PAL["bg"], borderwidth=0)
        s.configure("TNotebook.Tab", background=PAL["sidebar"], foreground=PAL["dim"],
                    padding=[14, 7], font=("Segoe UI", 9))
        s.map("TNotebook.Tab", background=[("selected", PAL["card"])],
              foreground=[("selected", "white")])
        s.configure("Treeview", background=PAL["card"], foreground=PAL["text"],
                    fieldbackground=PAL["card"], borderwidth=0)
        s.configure("TScrollbar", background=PAL["card"], borderwidth=0)

    def _build_ui(self):
        # Header
        hdr = tk.Frame(self, bg="#0A0C18", height=65)
        hdr.pack(fill="x"); hdr.pack_propagate(False)
        tk.Label(hdr, text="🔬 NCERT VIRTUAL LAB", font=("Segoe UI Bold", 17),
                 fg=PAL["accent"], bg="#0A0C18").pack(side="left", padx=25, pady=14)
        self.class_var = tk.StringVar(value="Class 10")
        classes = [f"Class {i}" for i in range(1, 13)]
        cb = ttk.Combobox(hdr, textvariable=self.class_var, values=classes, width=10,
                          state="readonly", font=("Segoe UI", 10))
        cb.pack(side="right", padx=25, pady=18)
        tk.Label(hdr, text="Select Class:", fg=PAL["dim"], bg="#0A0C18",
                 font=("Segoe UI", 9)).pack(side="right", pady=18)

        # Body
        body = tk.Frame(self, bg=PAL["bg"])
        body.pack(fill="both", expand=True)

        # Subject sidebar
        side = tk.Frame(body, bg=PAL["sidebar"], width=170)
        side.pack(side="left", fill="y"); side.pack_propagate(False)
        tk.Label(side, text="SUBJECTS", fg=PAL["dim"], bg=PAL["sidebar"],
                 font=("Segoe UI", 8, "bold"), pady=12).pack()

        self.exp_buttons = []
        for subj, (col, _) in self.SUBJECTS.items():
            btn = tk.Button(side, text=subj, fg="white", bg=PAL["sidebar"],
                            font=("Segoe UI", 10), relief="flat", anchor="w",
                            padx=14, pady=10, activebackground=col,
                            command=lambda s=subj: self._load_subject(s))
            btn.pack(fill="x")
            btn.bind("<Enter>", lambda e, b=btn, c=col: b.config(bg=c))
            btn.bind("<Leave>", lambda e, b=btn: b.config(bg=PAL["sidebar"]))
            self.exp_buttons.append((btn, col))

        # Right panel (experiments + output)
        right = tk.Frame(body, bg=PAL["bg"])
        right.pack(side="right", fill="both", expand=True, padx=10, pady=10)

        self.exp_frame = tk.Frame(right, bg=PAL["bg"], height=120)
        self.exp_frame.pack(fill="x", pady=(0, 8))

        splitter = tk.Frame(right, bg=PAL["bg"])
        splitter.pack(fill="both", expand=True)

        self.input_frame = tk.Frame(splitter, bg=PAL["card"], width=380,
                                    highlightthickness=1, highlightbackground=PAL["border"])
        self.input_frame.pack(side="left", fill="y", padx=(0, 8))
        self.input_frame.pack_propagate(False)

        self.output_txt = scrolledtext.ScrolledText(splitter, bg="#080A15", fg=PAL["success"],
                                                    font=("Cascadia Code", 10), borderwidth=0,
                                                    padx=15, pady=15, insertbackground="green")
        self.output_txt.pack(side="right", fill="both", expand=True)
        self.output_txt.tag_config("title", foreground=PAL["accent"], font=("Segoe UI Bold", 12))
        self.output_txt.tag_config("ok", foreground=PAL["success"])
        self.output_txt.tag_config("warn", foreground=PAL["warning"])
        self.output_txt.tag_config("dim", foreground=PAL["dim"])

        self._welcome()

    def _welcome(self):
        self.output_txt.delete("1.0", "end")
        self.output_txt.insert("end", "NCERT VIRTUAL LAB — SIGMAOS\n", "title")
        self.output_txt.insert("end", "═"*42 + "\n\n", "dim")
        self.output_txt.insert("end",
            "• Select a subject from the left panel\n"
            "• Pick an experiment\n"
            "• Enter values and click RUN\n\n"
            "Covers NCERT Physics, Chemistry, Biology\n"
            "& Mathematics — Classes 1 to 12.\n\n"
            "100% Offline  |  Zero 3rd-party deps\n", "ok")

    def _load_subject(self, subj):
        self.current_subject = subj
        col, cls = self.SUBJECTS[subj]
        self.current_color = col
        for w in self.exp_frame.winfo_children(): w.destroy()
        for w in self.input_frame.winfo_children(): w.destroy()

        tk.Label(self.exp_frame, text=f"{subj}  Experiments",
                 font=("Segoe UI Bold", 13), fg=col, bg=PAL["bg"]).pack(anchor="w", pady=(0, 6))

        wrap = tk.Frame(self.exp_frame, bg=PAL["bg"])
        wrap.pack(fill="x")
        for i, (name, _) in enumerate(cls.EXPERIMENTS.items()):
            btn = tk.Button(wrap, text=name, fg="white", bg=PAL["card"],
                            font=("Segoe UI", 8), relief="flat", padx=10, pady=6,
                            command=lambda n=name, c=cls: self._load_experiment(n, c))
            btn.grid(row=i//5, column=i%5, padx=3, pady=3, sticky="w")
            btn.bind("<Enter>", lambda e, b=btn, c=col: b.config(bg=c))
            btn.bind("<Leave>", lambda e, b=btn: b.config(bg=PAL["card"]))

        self.output_txt.delete("1.0", "end")
        self.output_txt.insert("end", f"  {subj} Lab Loaded\n", "title")
        self.output_txt.insert("end", f"  {len(cls.EXPERIMENTS)} experiments available.\n", "ok")

    def _load_experiment(self, name, cls):
        for w in self.input_frame.winfo_children(): w.destroy()
        col = self.current_color

        tk.Label(self.input_frame, text=name, fg=col, bg=PAL["card"],
                 font=("Segoe UI Bold", 11), pady=10).pack(anchor="w", padx=12)
        tk.Frame(self.input_frame, bg=PAL["border"], height=1).pack(fill="x", padx=8)

        entries = {}

        def entry_row(label, default=""):
            fr = tk.Frame(self.input_frame, bg=PAL["card"])
            fr.pack(fill="x", padx=12, pady=4)
            tk.Label(fr, text=label, fg=PAL["dim"], bg=PAL["card"],
                     font=("Segoe UI", 9), width=18, anchor="w").pack(side="left")
            e = tk.Entry(fr, bg="#0F1225", fg="white", insertbackground="white",
                         font=("Cascadia Code", 10), relief="flat", highlightthickness=1,
                         highlightbackground=PAL["border"])
            e.insert(0, str(default))
            e.pack(side="right", fill="x", expand=True)
            entries[label] = e
            return e

        # ── Physics ──────────────────────────────────────────────────
        if name == "Ohm's Law":
            entry_row("Voltage V (V)", "")
            entry_row("Current I (A)", "2")
            entry_row("Resistance R (Ω)", "5")
            def run():
                V=entries["Voltage V (V)"].get(); I=entries["Current I (A)"].get(); R=entries["Resistance R (Ω)"].get()
                V=float(V) if V else None; I=float(I) if I else None; R=float(R) if R else None
                self._show(name, PhysicsLab.ohms_law(V=V,I=I,R=R))

        elif name == "Projectile Motion":
            entry_row("Initial Speed (m/s)", "20"); entry_row("Angle (°)", "45"); entry_row("Height h0 (m)", "0")
            def run():
                self._show(name, PhysicsLab.projectile_motion(
                    float(entries["Initial Speed (m/s)"].get()),
                    float(entries["Angle (°)"].get()),
                    float(entries["Height h0 (m)"].get())))

        elif name == "Simple Pendulum":
            entry_row("Length L (m)", "1.0")
            def run(): self._show(name, PhysicsLab.simple_pendulum(float(entries["Length L (m)"].get())))

        elif name == "Lens Formula":
            entry_row("Focal Length f (cm)", "10"); entry_row("Object Dist u (cm)", "-30"); entry_row("Image Dist v (cm)", "")
            def run():
                f=entries["Focal Length f (cm)"].get(); u=entries["Object Dist u (cm)"].get(); v=entries["Image Dist v (cm)"].get()
                self._show(name, PhysicsLab.lens_formula(float(f) if f else None,float(u) if u else None,float(v) if v else None))

        elif name == "Newton's 2nd Law":
            entry_row("Mass m (kg)", "5"); entry_row("Acceleration a (m/s²)", "3")
            def run(): self._show(name, PhysicsLab.newtons_second_law(float(entries["Mass m (kg)"].get()),float(entries["Acceleration a (m/s²)"].get())))

        elif name == "Radioactive Decay":
            entry_row("Initial Atoms N₀", "1000"); entry_row("Half-life (years)", "10"); entry_row("Time t (years)", "25")
            def run(): self._show(name, PhysicsLab.radioactive_decay(float(entries["Initial Atoms N₀"].get()),float(entries["Half-life (years)"].get()),float(entries["Time t (years)"].get())))

        elif name == "Coulomb's Law":
            entry_row("Charge q1 (C)", "1e-6"); entry_row("Charge q2 (C)", "2e-6"); entry_row("Distance r (m)", "0.1")
            def run(): self._show(name, PhysicsLab.coulombs_law(float(entries["Charge q1 (C)"].get()),float(entries["Charge q2 (C)"].get()),float(entries["Distance r (m)"].get())))

        elif name == "Snell's Law":
            entry_row("n1 (medium 1)", "1.0"); entry_row("Angle θ1 (°)", "30"); entry_row("n2 (medium 2)", "1.5")
            def run(): self._show(name, PhysicsLab.snells_law(float(entries["n1 (medium 1)"].get()),float(entries["Angle θ1 (°)"].get()),float(entries["n2 (medium 2)"].get())))

        elif name == "Boyle's Law":
            entry_row("P1 (atm)", "2"); entry_row("V1 (L)", "5"); entry_row("P2 (atm)", "4")
            def run(): self._show(name, PhysicsLab.boyles_law(float(entries["P1 (atm)"].get()),float(entries["V1 (L)"].get()),float(entries["P2 (atm)"].get())))

        elif name == "Photoelectric Effect":
            entry_row("Frequency (Hz)", "6e14")
            def run(): self._show(name, PhysicsLab.photoelectric_effect(float(entries["Frequency (Hz)"].get())))

        # ── Chemistry ─────────────────────────────────────────────────
        elif name == "Molar Mass Calculator":
            entry_row("Formula (e.g. H2O)", "H2O")
            def run(): self._show(name, ChemistryLab.molar_mass(entries["Formula (e.g. H2O)"].get().strip()))

        elif name == "pH Calculator":
            entry_row("Concentration (mol/L)", "0.01"); entry_row("Is Acid? (1/0)", "1")
            def run(): self._show(name, ChemistryLab.ph_calculator(float(entries["Concentration (mol/L)"].get()),bool(int(entries["Is Acid? (1/0)"].get()))))

        elif name == "Ideal Gas Law":
            entry_row("P (atm, blank=?)", ""); entry_row("V (L, blank=?)", "10"); entry_row("n (mol)", "1"); entry_row("T (K)", "300")
            def run():
                P=entries["P (atm, blank=?)"].get(); V=entries["V (L, blank=?)"].get()
                n=entries["n (mol)"].get(); T=entries["T (K)"].get()
                self._show(name,ChemistryLab.ideal_gas_law(float(P) if P else None,float(V) if V else None,float(n) if n else None,float(T) if T else None))

        elif name == "Molarity":
            entry_row("Moles (mol)", "0.5"); entry_row("Volume (L)", "0.25")
            def run(): self._show(name, ChemistryLab.molarity(float(entries["Moles (mol)"].get()),float(entries["Volume (L)"].get())))

        elif name == "Enthalpy Change":
            entry_row("Products ΔH (kJ)", "-600"); entry_row("Reactants ΔH (kJ)", "-200")
            def run(): self._show(name, ChemistryLab.enthalpy_change(float(entries["Products ΔH (kJ)"].get()),float(entries["Reactants ΔH (kJ)"].get())))

        elif name == "Rate of Reaction":
            entry_row("ΔConcentration (mol/L)", "0.05"); entry_row("ΔTime (s)", "10")
            def run(): self._show(name, ChemistryLab.rate_of_reaction(float(entries["ΔConcentration (mol/L)"].get()),float(entries["ΔTime (s)"].get())))

        elif name == "Periodic Table Query":
            entry_row("Element Symbol (e.g. Fe)", "Fe")
            def run():
                sym=entries["Element Symbol (e.g. Fe)"].get().capitalize()
                d=ChemistryLab.ELEMENTS.get(sym,{"error":"Not found"})
                self._show(name,{"symbol":sym,**d})

        # ── Biology ──────────────────────────────────────────────────
        elif name == "Genetics (Mendel)":
            entry_row("Parent 1 genotype (e.g. Tt)", "Tt"); entry_row("Parent 2 genotype", "Tt")
            def run(): self._show(name, BiologyLab.mendel_cross(entries["Parent 1 genotype (e.g. Tt)"].get(),entries["Parent 2 genotype"].get()))

        elif name == "Photosynthesis Rate":
            entry_row("Light Intensity (lux)", "800"); entry_row("CO2 (ppm)", "400"); entry_row("Temperature (°C)", "25")
            def run(): self._show(name, BiologyLab.photosynthesis_rate(float(entries["Light Intensity (lux)"].get()),float(entries["CO2 (ppm)"].get()),float(entries["Temperature (°C)"].get())))

        elif name == "Osmosis / Diffusion":
            entry_row("Cell conc (mol/L)", "0.5"); entry_row("Solution conc (mol/L)", "0.8")
            def run(): self._show(name, BiologyLab.osmosis(float(entries["Cell conc (mol/L)"].get()),float(entries["Solution conc (mol/L)"].get())))

        elif name == "BMI Calculator":
            entry_row("Weight (kg)", "65"); entry_row("Height (m)", "1.70")
            def run(): self._show(name, BiologyLab.bmi(float(entries["Weight (kg)"].get()),float(entries["Height (m)"].get())))

        elif name == "Microscope Magnification":
            entry_row("Objective lens (×)", "40"); entry_row("Eyepiece (×)", "10")
            def run(): self._show(name, BiologyLab.microscope_magnification(int(entries["Objective lens (×)"].get()),int(entries["Eyepiece (×)"].get())))

        elif name == "Human Heartbeat Calc":
            entry_row("Age (years)", "16")
            def run(): self._show(name, BiologyLab.heart_rate_zones(int(entries["Age (years)"].get())))

        # ── Mathematics ───────────────────────────────────────────────
        elif name == "Quadratic Solver":
            entry_row("a", "1"); entry_row("b", "-5"); entry_row("c", "6")
            def run(): self._show(name, MathLab.quadratic(float(entries["a"].get()),float(entries["b"].get()),float(entries["c"].get())))

        elif name == "Statistics (Mean/SD)":
            entry_row("Data (comma-sep)", "23,45,12,67,34,56,9,100")
            def run():
                data=[float(x.strip()) for x in entries["Data (comma-sep)"].get().split(",")]
                self._show(name, MathLab.statistics(data))

        elif name == "Trigonometry":
            entry_row("Angle (°)", "30")
            def run(): self._show(name, MathLab.trig_values(float(entries["Angle (°)"].get())))

        elif name == "Arithmetic Progression":
            entry_row("First term a", "2"); entry_row("Common diff d", "3"); entry_row("Term number n", "10")
            def run(): self._show(name, MathLab.ap_terms(float(entries["First term a"].get()),float(entries["Common diff d"].get()),int(entries["Term number n"].get())))

        elif name == "Geometric Progression":
            entry_row("First term a", "3"); entry_row("Common ratio r", "2"); entry_row("Term number n", "8")
            def run(): self._show(name, MathLab.gp_terms(float(entries["First term a"].get()),float(entries["Common ratio r"].get()),int(entries["Term number n"].get())))

        elif name == "Permutations & Combos":
            entry_row("n (total)", "10"); entry_row("r (choose)", "3")
            def run():
                n,r=int(entries["n (total)"].get()),int(entries["r (choose)"].get())
                self._show(name,{"P(n,r)":MathLab.permutation(n,r),"C(n,r)":MathLab.combination(n,r)})

        elif name == "Geometry Calculator":
            entry_row("Shape (circle/rect/tri/sphere/cyl)", "circle")
            entry_row("Param 1 (r or l or a)", "5")
            entry_row("Param 2 (h or w or b)", "")
            entry_row("Param 3 (c)", "")
            def run():
                sh=entries["Shape (circle/rect/tri/sphere/cyl)"].get().strip().lower()
                p1=entries["Param 1 (r or l or a)"].get(); p2=entries["Param 2 (h or w or b)"].get(); p3=entries["Param 3 (c)"].get()
                kw={}
                if sh=="circle": kw={"r":float(p1)}
                elif sh in("sphere","cyl"): kw={"r":float(p1),"h":float(p2) if p2 else 1}
                elif sh=="rect": kw={"l":float(p1),"w":float(p2)}
                elif sh=="tri": kw={"a":float(p1),"b":float(p2),"c":float(p3)}
                self._show(name, MathLab.geometry(sh,**kw))

        elif name == "Binomial Theorem":
            entry_row("n (trials)", "10"); entry_row("x (successes)", "3"); entry_row("p (probability)", "0.4")
            def run(): self._show(name, MathLab.binomial(int(entries["n (trials)"].get()),int(entries["x (successes)"].get()),float(entries["p (probability)"].get())))

        elif name == "Complex Numbers":
            entry_row("a1 (Re z1)", "3"); entry_row("b1 (Im z1)", "4")
            entry_row("a2 (Re z2)", "1"); entry_row("b2 (Im z2)", "-2")
            def run(): self._show(name, MathLab.complex_ops(float(entries["a1 (Re z1)"].get()),float(entries["b1 (Im z1)"].get()),float(entries["a2 (Re z2)"].get()),float(entries["b2 (Im z2)"].get())))

        else:
            def run(): self._show(name, {"status": "Experiment coming soon!"})

        tk.Frame(self.input_frame, bg=PAL["border"], height=1).pack(fill="x", padx=8, pady=8)
        run_btn = tk.Button(self.input_frame, text="▶  RUN EXPERIMENT",
                            bg=col, fg="white", font=("Segoe UI Bold", 10),
                            relief="flat", padx=20, pady=10, command=run)
        run_btn.pack(pady=12, padx=12, fill="x")
        run_btn.bind("<Enter>", lambda e: run_btn.config(bg=PAL["accent"]))
        run_btn.bind("<Leave>", lambda e: run_btn.config(bg=col))

    def _show(self, title, result):
        self.output_txt.delete("1.0", "end")
        self.output_txt.insert("end", f"  EXPERIMENT: {title}\n", "title")
        self.output_txt.insert("end", "  " + "─"*44 + "\n\n", "dim")
        self._pretty_print(result, indent=2)

    def _pretty_print(self, obj, indent=0):
        pad = " " * indent
        if isinstance(obj, dict):
            for k, v in obj.items():
                if isinstance(v, dict):
                    self.output_txt.insert("end", f"{pad}{k}:\n", "warn")
                    self._pretty_print(v, indent+4)
                else:
                    self.output_txt.insert("end", f"{pad}{k}: ", "warn")
                    self.output_txt.insert("end", f"{v}\n", "ok")
        else:
            self.output_txt.insert("end", f"{pad}{obj}\n", "ok")

def launch(kernel=None):
    app = NCERTVirtualLab(kernel)
    app.mainloop()

if __name__ == "__main__":
    launch()
