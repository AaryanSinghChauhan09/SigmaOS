"""
SigmaOS NCERT Physics Lab v9.0 — The Comprehensive series
Classes 6–12 | Exhaustive NCERT Experiment & Calculator Suite
100% stdlib, zero 3rd-party deps
"""
import math, random

def _r(x, d=4):
    try: return float(("{:." + str(int(d)) + "f}").format(float(x)))
    except: return x

class Physics_Classes_6_10:
    TITLE = "Secondary Physics: Advanced Simulations"
    EXP_DATA = {
        "Magnetism (Interaction)": ("magnet", [("Pole 1 (N/S)", "N"), ("Pole 2 (N/S)", "S")]),
        "Light & Shadows": ("shadow", [("Object Distance (cm)", "20"), ("Lamp Height (cm)", "50")]),
        "Electric Circuit (V=IR)": ("ohms_law", [("Voltage (V)", "10"), ("Resistance (Ω)", "5")]),
        "Mirror/Lens Formula": ("optics", [("Type (Mirror/Lens)", "Mirror"), ("Object dist u (cm)", "-20"), ("Focal length f (cm)", "10")]),
        "Gravity & Weight": ("gravity_weight", [("Mass (kg)", "60"), ("Planet", "Earth")]),
        "Archimedes Principle": ("buoyancy", [("Object Vol (m³)", "0.001"), ("Fluid Density (kg/m³)", "1000")]),
        "Work & Energy": ("energy", [("Mass (kg)", "2"), ("Velocity (m/s)", "10"), ("Height (m)", "5")]),
        "Sound Velocity": ("sound_vel", [("Frequency (Hz)", "440"), ("Wavelength (m)", "0.78")]),
        "Heat Capacity (Water)": ("heat_cap", [("Mass (kg)", "0.5"), ("Temp Change (K)", "10")]),
        "Atmospheric Pressure": ("atm_p", [("Altitude (m)", "1000")]),
    }

    @staticmethod
    def magnet(p1, p2):
        p1, p2 = p1.upper(), p2.upper()
        if p1 == p2: return {"Result": "REPEL", "Force": "Like poles repel each other"}
        return {"Result": "ATTRACT", "Force": "Unlike poles attract each other"}

    @staticmethod
    def shadow(d, h):
        d, h = float(d), float(h)
        ratio = h / d
        return {"Shadow Scale Factor": _r(ratio, 2), "Note": "Closer to light = Larger shadow"}

    @staticmethod
    def ohms_law(v, r):
        v, r = float(v), float(r)
        return {"Current I (A)": _r(v/r), "Power P (W)": _r(v**2/r)}

    @staticmethod
    def optics(t, u, f):
        u, f = float(u), float(f)
        if "mirror" in t.lower():
            v = 1 / (1/f - 1/u)
            m = -v/u
        else:
            v = 1 / (1/f + 1/u)
            m = v/u
        return {"Image dist v (cm)": _r(v, 2), "Magnification m": _r(m, 2)}

    @staticmethod
    def gravity_weight(m, p):
        gs = {"earth": 9.81, "moon": 1.62, "mars": 3.71, "jupiter": 24.79, "sun": 274}
        g = gs.get(p.lower(), 9.81)
        return {"Weight (N)": _r(float(m)*g, 1), "g (m/s²)": g}

    @staticmethod
    def buoyancy(v, d):
        v, d = float(v), float(d)
        f = v * d * 9.81
        return {"Upthrust (N)": _r(f, 2), "Observation": "Object floats if Upthrust >= Weight"}

    @staticmethod
    def energy(m, v, h):
        m, v, h = float(m), float(v), float(h)
        ke = 0.5 * m * v**2
        pe = m * 9.81 * h
        return {"Kinetic (J)": _r(ke, 2), "Potential (J)": _r(pe, 2), "Total (J)": _r(ke+pe, 2)}

    @staticmethod
    def sound_vel(f, l):
        f, l = float(f), float(l)
        return {"Velocity (m/s)": _r(f*l, 1), "Note": "Standard at 20°C is ~343 m/s"}

    @staticmethod
    def heat_cap(m, dt):
        m, dt = float(m), float(dt)
        c = 4184 # Specific heat of water
        q = m * c * dt
        return {"Heat Required (J)": _r(q, 1), "In kcal": _r(q/4184, 2)}

    @staticmethod
    def atm_p(h):
        h = float(h)
        # P = P0 * exp(-mgh/kT) approx
        p = 101325 * math.exp(-0.00012 * h)
        return {"Pressure (Pa)": _r(p, 0), "In atm": _r(p/101325, 3)}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Comprehensive Lab Manual"
    EXP_DATA = {
        "Parallelogram Law": ("vector_sum", [("Force P (N)", "3"), ("Force Q (N)", "4"), ("Angle (deg)", "90")]),
        "Centripetal Force": ("centripetal", [("Mass (kg)", "0.5"), ("Velocity (m/s)", "10"), ("Radius (m)", "2")]),
        "Moment of Inertia": ("moi", [("Shape", "Ring"), ("Mass (kg)", "1"), ("Radius (m)", "0.5")]),
        "Surface Tension (Rise)": ("capillary", [("Radius (mm)", "0.5"), ("Surface Tension", "0.072")]),
        "Viscosity (Stoke's)": ("viscosity", [("Ball Radius (mm)", "1"), ("Density (kg/m³)", "7800"), ("Fluid Density", "1260"), ("Viscosity (Pa·s)", "0.8")]),
        "Spherometer": ("spherometer", [("Reading (mm)", "1.25"), ("Pitch (mm)", "1"), ("LC (mm)", "0.01")]),
        "Potentiometer (EMF)": ("pot_emf", [("L1 (cm)", "100"), ("L2 (cm)", "150"), ("E1 (V)", "1.5")]),
        "Young's Double Slit": ("ydse", [("Wave (nm)", "589"), ("Slit d (mm)", "0.1"), ("Dist D (m)", "1")]),
        "Semi-conductor Diode": ("diode", [("Voltage (V)", "0.7"), ("Temp (K)", "300")]),
        "Logic Gates (Advanced)": ("logic_adv", [("Gate (NAND/NOR/XOR)", "NAND"), ("A", "1"), ("B", "0")]),
    }

    @staticmethod
    def vector_sum(p, q, theta):
        p, q, theta = float(p), float(q), math.radians(float(theta))
        r = math.sqrt(p**2 + q**2 + 2*p*q*math.cos(theta))
        alpha = math.degrees(math.atan2(q * math.sin(theta), p + q * math.cos(theta)))
        return {"Resultant (N)": _r(r, 2), "Direction (deg)": _r(alpha, 1)}

    @staticmethod
    def centripetal(m, v, r):
        m, v, r = float(m), float(v), float(r)
        f = (m * v**2) / r
        return {"Force (N)": _r(f, 2)}

    @staticmethod
    def moi(shape, m, r):
        m, r = float(m), float(r)
        s = shape.lower()
        if "ring" in s: i = m * r**2
        elif "disc" in s: i = 0.5 * m * r**2
        elif "sphere" in s: i = 0.4 * m * r**2 # Solid
        elif "rod" in s: i = (1/12) * m * r**2 # Center axis, r is length
        else: return {"Error": "Shape unknown"}
        return {"MOI (kg·m²)": _r(i, 4)}

    @staticmethod
    def capillary(r_mm, t):
        r = float(r_mm) / 1000
        t = float(t)
        h = (2 * t) / (r * 1000 * 9.81)
        return {"Rise Height (m)": _r(h, 4), "In cm": _r(h*100, 2)}

    @staticmethod
    def viscosity(r_mm, ds, df, eta):
        r = float(r_mm) / 1000
        ds, df, eta = float(ds), float(df), float(eta)
        v = (2/9) * (r**2 * (ds - df) * 9.81) / eta
        return {"Terminal Velocity (m/s)": _r(v, 4)}

    @staticmethod
    def spherometer(r, p, lc):
        r, p, lc = float(r), float(p), float(lc)
        # Spherometer reading logic
        total = r * p / 100 # simplified
        return {"Thickness/Curvature": _r(total, 4)}

    @staticmethod
    def pot_emf(l1, l2, e1):
        l1, l2, e1 = float(l1), float(l2), float(e1)
        e2 = e1 * (l2 / l1)
        return {"EMF of Second Cell (V)": _r(e2, 3)}

    @staticmethod
    def ydse(lam_nm, d_mm, big_d):
        lam = float(lam_nm) * 1e-9
        d = float(d_mm) * 1e-3
        big_d = float(big_d)
        w = (lam * big_d) / d
        return {"Fringe Width (m)": f"{w:.4e}", "In mm": _r(w*1000, 3)}

    @staticmethod
    def diode(v, t):
        v, t = float(v), float(t)
        # I = Is * (exp(qV/kT) - 1)
        k = 1.38e-23; q = 1.6e-19; is_sat = 1e-12
        i = is_sat * (math.exp((q * v) / (k * t)) - 1)
        return {"Forward Current (A)": f"{i:.4e}"}

    @staticmethod
    def logic_adv(g, a, b):
        a, b = int(a), int(b)
        mode = g.upper()
        if mode == "NAND": res = not (a and b)
        elif mode == "NOR": res = not (a or b)
        elif mode == "XOR": res = a != b
        else: res = 0
        return {"Output": int(res)}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
