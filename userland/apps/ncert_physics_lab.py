"""
SigmaOS NCERT Physics Lab v10.0 — The Ultimate Series
Classes 6–12 | Exhaustive NCERT Experiment & Simulation Hub
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
        "Mirror/Lens Formula": ("optics", [("Type (Mirror/Lens)", "Mirror"), ("u (cm)", "-20"), ("f (cm)", "10")]),
        "Gravity & Weight": ("gravity_weight", [("Mass (kg)", "60"), ("Planet", "Earth")]),
        "Archimedes Principle": ("buoyancy", [("Object Vol (m³)", "0.001"), ("Fluid Density (kg/m³)", "1000")]),
        "Work & Energy": ("energy", [("Mass (kg)", "2"), ("Velocity (m/s)", "10"), ("Height (m)", "5")]),
        "Sound Velocity": ("sound_vel", [("Freq (Hz)", "440"), ("Lambda (m)", "0.78")]),
        "Heat Capacity": ("heat_cap", [("Mass (kg)", "0.5"), ("dT (K)", "10")]),
        "Hooke's Law": ("hookes", [("Mass (g)", "100"), ("Extension (cm)", "2")]),
        "Simple Pendulum": ("pendulum", [("Length (m)", "0.5"), ("Angle (deg)", "10")]),
    }

    @staticmethod
    def magnet(p1, p2):
        if p1.upper() == p2.upper(): return {"Result": "REPEL"}
        return {"Result": "ATTRACT"}

    @staticmethod
    def shadow(d, h):
        return {"Scale Factor": _r(h / d, 2)}

    @staticmethod
    def ohms_law(v, r):
        return {"Current (A)": _r(v/r), "Power (W)": _r(v**2/r)}

    @staticmethod
    def optics(t, u, f):
        if "mirror" in t.lower(): v = 1/(1/f - 1/u)
        else: v = 1/(1/f + 1/u)
        return {"Image dist (cm)": _r(v, 2), "Mag": _r(-v/u if "mirror" in t.lower() else v/u, 2)}

    @staticmethod
    def gravity_weight(m, p):
        g = {"earth":9.81, "moon":1.62, "mars":3.71, "jupiter":24.79}.get(p.lower(), 9.81)
        return {"Weight (N)": _r(m*g, 2)}

    @staticmethod
    def buoyancy(v, d):
        return {"Upthrust (N)": _r(v*d*9.81, 2)}

    @staticmethod
    def energy(m, v, h):
        return {"KE (J)": _r(0.5*m*v**2, 2), "PE (J)": _r(m*9.81*h, 2)}

    @staticmethod
    def sound_vel(f, l):
        return {"Velocity (m/s)": _r(f*l, 1)}

    @staticmethod
    def heat_cap(m, dt):
        return {"Heat (J)": _r(m*4184*dt, 1)}

    @staticmethod
    def hookes(m_g, x_cm):
        f = (m_g/1000)*9.81
        k = f / (x_cm/100)
        return {"Spring Constant k (N/m)": _r(k, 2)}

    @staticmethod
    def pendulum(l, a):
        t = 2 * math.pi * math.sqrt(l/9.81)
        return {"Time Period (s)": _r(t, 2)}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Exhaustive Lab Manual"
    EXP_DATA = {
        "Vector Resultant": ("vector", [("P (N)", "3"), ("Q (N)", "4"), ("Angle", "90")]),
        "Viscosity (Stokes)": ("viscosity", [("r (mm)", "1"), ("rho_s (kg/m³)", "7800"), ("rho_f", "1260"), ("v (m/s)", "0.5")]),
        "Prism (Ref Index)": ("prism", [("A", "60"), ("Dm", "30")]),
        "Zener Regulator": ("zener", [("Vin", "12"), ("Vz", "10"), ("Rs", "100")]),
        "Photoelectric": ("photo", [("Wavelength (nm)", "400"), ("Work Fn (eV)", "2.1")]),
        "Young's Modulus": ("youngs", [("Load (kg)", "5"), ("Length (m)", "2"), ("Radius (mm)", "0.25"), ("dl (mm)", "0.5")]),
        "Capillary Rise": ("capillary", [("Radius (mm)", "0.5"), ("Surface T (N/m)", "0.072")]),
        "Transistor (CE)": ("transistor", [("Ib (uA)", "20"), ("Ic (mA)", "2"), ("Vce (V)", "5")]),
        "Resonance Tube": ("resonance", [("Freq (Hz)", "512"), ("L1 (cm)", "16")]),
        "Newton's Cooling": ("cooling", [("T0 (C)", "80"), ("Ta (C)", "25"), ("k", "0.05"), ("Time (t)", "10")]),
    }

    @staticmethod
    def vector(p, q, th):
        r = math.sqrt(p**2 + q**2 + 2*p*q*math.cos(math.radians(th)))
        return {"Resultant (N)": _r(r, 2)}

    @staticmethod
    def viscosity(r_mm, rs, rf, v):
        r = r_mm/1000
        eta = (2 * r**2 * (rs-rf) * 9.81) / (9 * v)
        return {"Viscosity (Pa.s)": _r(eta, 4)}

    @staticmethod
    def prism(a, dm):
        ar, dmr = math.radians(a), math.radians(dm)
        n = math.sin((ar+dmr)/2) / math.sin(ar/2)
        return {"Ref Index n": _r(n, 3)}

    @staticmethod
    def zener(vin, vz, rs):
        if vin < vz: return {"Status": "Off", "Vout": vin}
        iz = (vin - vz) / rs
        return {"Status": "Regulating", "Vout": vz, "Iz (A)": _r(iz, 4)}

    @staticmethod
    def photo(lam, phi):
        e_ev = (1240/lam)
        if e_ev < phi: return {"Status": "No Emission"}
        return {"KE_max (eV)": _r(e_ev - phi, 2), "Stopping V": _r(e_ev - phi, 2)}

    @staticmethod
    def youngs(m, l, r_mm, dl_mm):
        f = m * 9.81
        a = math.pi * (r_mm/1000)**2
        y = (f * l) / (a * (dl_mm/1000))
        return {"Young's Modulus (Pa)": f"{y:.4e}"}

    @staticmethod
    def capillary(r_mm, t):
        r = r_mm/1000
        h = (2*t)/(r*1000*9.81)
        return {"Rise (cm)": _r(h*100, 2)}

    @staticmethod
    def transistor(ib, ic, vce):
        beta = (ic*1e-3) / (ib*1e-6)
        return {"Current Gain Beta": _r(beta, 1)}

    @staticmethod
    def resonance(f, l1):
        v = 4 * f * (l1/100)
        return {"Sound Vel (m/s)": _r(v, 1)}

    @staticmethod
    def cooling(t0, ta, k, t):
        tt = ta + (t0 - ta)*math.exp(-k*t)
        return {"Temp at t (C)": _r(tt, 2)}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
