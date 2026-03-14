"""
SigmaOS NCERT Physics Lab v10.1 — The Ultimate Series
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
        "Magnet Interaction": ("magnet", [("P1 (N/S)", "N"), ("P2 (N/S)", "S")]),
        "Light & Shadow": ("shadow", [("Dist (cm)", "20"), ("Height (cm)", "50")]),
        "Ohm's Law (V=IR)": ("ohms", [("V (V)", "10"), ("R (Ω)", "5")]),
        "Mirror/Lens Calc": ("optics", [("Type (M/L)", "Mirror"), ("u (cm)", "-20"), ("f (cm)", "10")]),
        "Archimedes/Density": ("buoyancy", [("Vol (m³)", "0.001"), ("Rho (kg/m³)", "1000")]),
        "Joule's Heating": ("joules", [("I (A)", "2"), ("R (Ω)", "10"), ("Time (s)", "60")]),
        "Snell's Law": ("snell", [("n1", "1"), ("Angle1 (deg)", "30"), ("n2", "1.5")]),
        "Gravity Logic": ("gravity", [("Mass (kg)", "60"), ("Body", "Earth")]),
        "Pendulum Clock": ("pendulum", [("Length (m)", "1")]),
        "Sound Speed (fλ)": ("sound", [("Freq (Hz)", "440"), ("Lambda (m)", "0.78")]),
        "Friction Lab": ("friction", [("Mass (kg)", "5"), ("Surface (Ice/Wood/Rubber)", "Wood")]),
        "Work Done": ("work", [("Force (N)", "50"), ("Disp (m)", "10"), ("Angle (deg)", "0")]),
        "Newton's 2nd (F=ma)": ("f_ma", [("Mass (kg)", "10"), ("Acc (m/s²)", "5")]),
        "Sound Echo": ("echo", [("Time (s)", "2"), ("Temp (C)", "20")]),
        "Eye Lens Power": ("eye", [("Object Dist (m)", "2")]),
    }

    @staticmethod
    def magnet(p1, p2):
        if p1.upper() == p2.upper(): return {"Result": "REPEL"}
        return {"Result": "ATTRACT"}

    @staticmethod
    def shadow(d, h):
        return {"Scale": _r(h / d, 2)}

    @staticmethod
    def ohms(v, r):
        return {"I (A)": _r(v/r), "P (W)": _r(v**2/r)}

    @staticmethod
    def optics(t, u, f):
        if "mirror" in t.lower(): v = 1/(1/f - 1/u)
        else: v = 1/(1/f + 1/u)
        return {"v (cm)": _r(v, 2), "Mag": _r(-v/u if "mirror" in t.lower() else v/u, 2)}

    @staticmethod
    def buoyancy(v, d):
        return {"Upthrust (N)": _r(v*d*9.81, 2)}

    @staticmethod
    def joules(i, r, t):
        h = i**2 * r * t
        return {"Heat (J)": _r(h, 1)}

    @staticmethod
    def snell(n1, th, n2):
        r1 = math.radians(th)
        s2 = (n1 * math.sin(r1)) / n2
        if s2 > 1: return {"Result": "TIR"}
        return {"r Angle": _r(math.degrees(math.asin(s2)), 2)}

    @staticmethod
    def gravity(m, b):
        g = {"earth":9.81, "moon":1.62, "mars":3.71, "jupiter":24.79}.get(b.lower(), 9.81)
        return {"Weight (N)": _r(m*g, 2)}

    @staticmethod
    def pendulum(l):
        return {"Period T (s)": _r(2 * math.pi * math.sqrt(l/9.81), 3)}

    @staticmethod
    def sound(f, l):
        return {"v (m/s)": _r(f*l, 1)}

    @staticmethod
    def friction(m, s):
        u = {"ice":0.05, "wood":0.3, "rubber":0.7}.get(s.lower(), 0.3)
        return {"Friction (N)": _r(m * 9.81 * u, 2), "Mu": u}

    @staticmethod
    def work(f, d, a):
        w = f * d * math.cos(math.radians(a))
        return {"Work (J)": _r(w, 2)}

    @staticmethod
    def f_ma(m, a):
        return {"Force (N)": _r(m * a, 2)}

    @staticmethod
    def echo(t, temp):
        v = 331 + 0.6 * temp
        d = (v * t) / 2
        return {"Distance to Obstacle (m)": _r(d, 2), "Min Distance for Echo": "17.2m (at 20C)"}

    @staticmethod
    def eye(d):
        if d < 0.25: return {"Status": "Blurry", "Reason": "Near point limit (25cm)"}
        p = 1/d
        return {"Lens Power (D)": _r(p, 2), "Acommodation": "Active"}

class Physics_Classes_11_12:
    TITLE = "Senior Physics: Exhaustive Lab Manual"
    EXP_DATA = {
        "Coulomb Force": ("coulomb", [("q1 (uC)", "10"), ("q2 (uC)", "-5"), ("r (cm)", "3")]),
        "Doppler Shift": ("doppler", [("Freq (Hz)", "500"), ("Vs (m/s)", "20"), ("Vo (m/s)", "10")]),
        "Kirchhoff's Node": ("kirch_node", [("I1 (A)", "5"), ("I2 (A)", "-2"), ("I3 (A)", "-1")]),
        "Boyle's Law": ("boyle", [("P1 (atm)", "1"), ("V1 (L)", "22.4"), ("V2 (L)", "11.2")]),
        "YDSE (Fringe)": ("ydse", [("Lambda (nm)", "589"), ("d (mm)", "0.1"), ("D (m)", "1")]),
        "Photoelectric": ("photo", [("Wavelength (nm)", "400"), ("WorkFn (eV)", "2.3")]),
        "Viscosity (Stokes)": ("visco", [("Radius (mm)", "1"), ("Density_s", "7800"), ("Density_f", "1260"), ("vTerm", "0.5")]),
        "Biot-Savart (Loop)": ("biot_loop", [("Current (A)", "5"), ("Radius (cm)", "10"), ("Dist z (cm)", "0")]),
        "Torque on Loop": ("torque", [("B (T)", "0.5"), ("Area (m²)", "0.02"), ("Current (A)", "2"), ("Angle", "30")]),
        "Esc Velocity": ("escape", [("Planet Mass (kg)", "5.97e24"), ("Radius (km)", "6371")]),
        "Carnot Eff %": ("carnot", [("Th (K)", "600"), ("Tc (K)", "300")]),
        "Vernier Callipers": ("vernier", [("MS Reading (cm)", "1.2"), ("VS Division", "4"), ("LC (cm)", "0.01")]),
        "Screw Gauge": ("screw_gauge", [("PSR (mm)", "5"), ("HSD", "32"), ("LC (mm)", "0.01")]),
        "Meter Bridge": ("meter_bridge", [("Known R (Ω)", "2"), ("Balancing l (cm)", "40")]),
        "Potentiometer": ("potentiometer", [("E1 (V)", "2.0"), ("l1 (cm)", "150"), ("l2 (cm)", "220")]),
        "Hooke's Law (Y)": ("hookes", [("Load (kg)", "1"), ("Area (m²)", "1e-6"), ("L (m)", "2"), ("dL (mm)", "0.5")]),
        "Surface Tension": ("capillary", [("r (mm)", "0.5"), ("h (cm)", "3")]),
        "Resonance Tube": ("resonance", [("Freq (Hz)", "512"), ("l1 (cm)", "16")]),
        "Potentiometer (r)": ("pot_internal", [("l1 (cm)", "200"), ("l2 (cm)", "130"), ("R (Ω)", "10")]),
        "Newton's Cooling": ("cooling", [("T_env (C)", "25"), ("T_obj (C)", "80"), ("k", "0.05")]),
        "Specific Heat": ("specific_heat", [("Mass (kg)", "0.5"), ("Q (J)", "4200"), ("dT (K)", "2")]),
        "Zener Diode": ("zener", [("V_in (V)", "15"), ("V_z (V)", "10")]),
    }

    @staticmethod
    def coulomb(q1, q2, r):
        k = 9e9; q1 *= 1e-6; q2 *= 1e-6; r /= 100
        return {"Force (N)": _r(k * (q1 * q2) / r**2, 3)}

    @staticmethod
    def doppler(f, vs, vo):
        v = 343
        return {"Observed F (Hz)": _r(f * (v+vo)/(v-vs), 2)}

    @staticmethod
    def kirch_node(i1, i2, i3):
        return {"Sum outgoing": i1+i2+i3}

    @staticmethod
    def boyle(p1, v1, v2):
        return {"P2 (atm)": _r(p1*v1/v2, 3)}

    @staticmethod
    def ydse(lam, d, big_d):
        w = (lam*1e-9 * big_d) / (d*1e-3)
        return {"Width (mm)": _r(w*1000, 3)}

    @staticmethod
    def photo(lam, phi):
        e = 1240/lam
        return {"E_ph (eV)": _r(e, 2), "Emission": e > phi}

    @staticmethod
    def visco(r, rs, rf, v):
        r /= 1000
        eta = (2 * r**2 * (rs-rf) * 9.81) / (9 * v)
        return {"Viscosity (Pa.s)": _r(eta, 4)}

    @staticmethod
    def biot_loop(i, r, z):
        mu0 = 4*math.pi*1e-7; r/=100; z/=100
        bz = (mu0*i*r**2)/(2*(r**2+z**2)**1.5)
        return {"B (Tesla)": f"{bz:.4e}"}

    @staticmethod
    def torque(b, a, i, th):
        t = i * a * b * math.sin(math.radians(th))
        return {"Torque (Nm)": _r(t, 4)}

    @staticmethod
    def escape(m, r_km):
        g = 6.67e-11; r = r_km * 1000
        v = math.sqrt(2 * g * m / r)
        return {"v_esc (m/s)": _r(v, 1)}

    @staticmethod
    def carnot(th, tc):
        eff = 1 - (tc/th)
        return {"Efficiency %": _r(eff*100, 2)}

    @staticmethod
    def vernier(msr, vsd, lc):
        total = msr + (vsd * lc)
        return {"Thickness (cm)": _r(total, 3)}

    @staticmethod
    def screw_gauge(psr, hsd, lc):
        total = psr + (hsd * lc)
        return {"Diameter (mm)": _r(total, 3)}

    @staticmethod
    def meter_bridge(r, l):
        x = (r * (100 - l)) / l
        return {"Unknown X (Ω)": _r(x, 2)}

    @staticmethod
    def potentiometer(e1, l1, l2):
        # Comparison of EMF: E2 = E1 * (l2/l1)
        e2 = e1 * (l2 / l1)
        return {"EMF E2 (V)": _r(e2, 3)}

    @staticmethod
    def hookes(m, a, l, dl):
        stress = (m * 9.81) / a
        strain = (dl * 1e-3) / l
        y = stress / strain
        return {"Young's Modulus (Pa)": f"{y:.4e}", "Result": "Elastic" if strain < 0.01 else "Plastic"}

    @staticmethod
    def capillary(r_mm, h_cm):
        r = r_mm / 1000; h = h_cm / 100; rho = 1000; g = 9.81
        t = (rho * g * r * h) / 2
        return {"Surface Tension (N/m)": _r(t, 4)}

    @staticmethod
    def resonance(f, l1_cm):
        l1 = l1_cm / 100
        v = 4 * f * (l1 + 0.3 * (2 * 0.02)) # Approx with end correction
        return {"Speed of Sound (m/s)": _r(v, 2)}

    @staticmethod
    def pot_internal(l1, l2, r):
        # Internal resistance r = R * (l1 - l2) / l2
        int_r = r * (l1 - l2) / l2
        return {"Internal r (Ω)": _r(int_r, 2)}

    @staticmethod
    def cooling(te, to, k):
        t_seq = [0, 5, 10, 20, 30]
        res = {f"T at {t}m": _r(te + (to-te)*math.exp(-k*t), 1) for t in t_seq}
        return res

    @staticmethod
    def specific_heat(m, q, dt):
        return {"c (J/kg.K)": _r(q / (m * dt), 1)}

    @staticmethod
    def zener(vin, vz):
        if vin < vz: return {"V_out (V)": vin, "Status": "Normal Forward/Reverse"}
        return {"V_out (V)": vz, "Status": "Breakdown/Regulated"}

PHYSICS_REGISTRY = {
    "Classes 6-10": Physics_Classes_6_10,
    "Classes 11-12": Physics_Classes_11_12,
}
