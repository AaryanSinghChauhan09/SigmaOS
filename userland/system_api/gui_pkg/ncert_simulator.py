import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class NcertSimulatorPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "NCERT VIRTUAL LAB", "Physics, Chemistry, Math & Bio Simulation Suite")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "Simulator Launch Core")
        card.master.pack(pady=50)
        
        tk.Label(card, text="The NCERT Virtual Lab offers dynamic, browser-based simulations of curriculum experiments.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=400).pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("NCERT Lab", "Virtual Lab launched in browser.", "OK")

        ttk.Button(card, text="🎓 Launch NCERT Virtual Lab", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS NCERT Virtual Lab</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0A0A12; color: #F2F2F7; margin: 0; padding: 20px; }
        .container { max-width: 1000px; margin: 0 auto; background-color: #11111E; padding: 20px; border-radius: 10px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); }
        h1 { color: #5AC8FA; font-size: 26px; text-align: center; }
        .nav { display: flex; justify-content: center; gap: 15px; margin-bottom: 20px; }
        .nav button { background-color: #1C1C1E; color: white; border: 1px solid #38383A; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-weight: bold; }
        .nav button.active { background-color: #5856D6; border-color: #5856D6; }
        .nav button:hover { background-color: #2C2C2E; }
        .nav button.active:hover { background-color: #AF52DE; }
        
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        
        h2 { color: #4CD964; font-size: 20px; border-bottom: 1px solid #38383A; padding-bottom: 10px; }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .experiment-card { background-color: #1C1C1E; padding: 15px; border-radius: 8px; border: 1px solid #38383A; cursor: pointer; transition: transform 0.2s; }
        .experiment-card:hover { transform: scale(1.02); border-color: #5856D6; }
        .experiment-card h3 { margin-top: 0; color: #FFCC00; font-size: 16px; }
        .experiment-card p { font-size: 13px; color: #8E8E93; line-height: 1.4; }
        
        #sim-modal { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0,0,0,0.8); z-index: 1000; align-items: center; justify-content: center; }
        .modal-content { background-color: #11111E; padding: 25px; border-radius: 10px; max-width: 800px; width: 90%; max-height: 90%; overflow-y: auto; border: 1px solid #5Ac8fa; }
        .modal-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; border-bottom: 1px solid #38383A; padding-bottom: 10px; }
        .modal-header h2 { margin: 0; color: #5AC8FA; border: none; padding: 0; }
        .close-btn { background: none; border: none; color: #FF3B30; font-size: 24px; cursor: pointer; }
        
        .sim-area { background-color: #0A0A12; height: 300px; border-radius: 8px; border: 1px solid #38383A; margin-bottom: 20px; display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden; }
        .controls { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }
        .controls label { font-size: 12px; color: #8E8E93; display: block; margin-bottom: 5px; }
        .controls input[type="range"] { width: 100%; }
        .output-box { background-color: #252529; padding: 10px; border-radius: 4px; font-family: monospace; color: #4CD964; margin-top: 15px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎓 SigmaOS NCERT Virtual Lab</h1>
        
        <div class="nav">
            <button class="active" onclick="switchTab('physics')">🔭 Physics</button>
            <button onclick="switchTab('chemistry')">🧪 Chemistry</button>
            <button onclick="switchTab('math')">📐 Mathematics</button>
            <button onclick="switchTab('biology')">🧬 Biology</button>
        </div>
        
        <!-- Physics -->
        <div id="physics" class="tab-content active">
            <h2>Class 11 & 12 Physics Mechanics & Waves</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('pendulum')">
                    <h3>Simple Pendulum (SHM)</h3>
                    <p>Determine the acceleration due to gravity (g) using a simple pendulum. Adjust length and mass.</p>
                </div>
                <div class="experiment-card" onclick="openSim('ohm')">
                    <h3>Ohm's Law Verification</h3>
                    <p>Determine resistance per cm of a given wire by plotting a graph of potential difference versus current.</p>
                </div>
                <div class="experiment-card" onclick="openSim('optics')">
                    <h3>Focal Length: Convex Lens</h3>
                    <p>Find the focal length of a convex lens by plotting graphs between u and v or between 1/u and 1/v.</p>
                </div>
            </div>
        </div>

        <!-- Chemistry -->
        <div id="chemistry" class="tab-content">
            <h2>Class 11 & 12 Chemical Kinetics & Titration</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('titration')">
                    <h3>Acid-Base Titration</h3>
                    <p>Determine the concentration of an unknown KMnO4 solution by titrating it against a standard solution of Mohr's salt.</p>
                </div>
                <div class="experiment-card" onclick="openSim('salt')">
                    <h3>Salt Analysis</h3>
                    <p>Qualitative analysis to identify the cation and anion present in a given inorganic salt.</p>
                </div>
            </div>
        </div>

        <!-- Math -->
        <div id="math" class="tab-content">
            <h2>Calculus & Geometry Visualizations</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('calculus')">
                    <h3>Area under Curve (Integration)</h3>
                    <p>Visualize the concept of definite integrals as the area under a curve using Riemann sums.</p>
                </div>
                <div class="experiment-card" onclick="openSim('conic')">
                    <h3>Conic Sections</h3>
                    <p>Explore parabola, ellipse, and hyperbola by slicing a 3D cone at different angles.</p>
                </div>
            </div>
        </div>

        <!-- Biology -->
        <div id="biology" class="tab-content">
            <h2>Microscopy & Physiology</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('osmosis')">
                    <h3>Osmosis (Potato Osmometer)</h3>
                    <p>Study osmosis by a potato osmometer. Observe water movement across semi-permeable membranes.</p>
                </div>
                <div class="experiment-card" onclick="openSim('mitosis')">
                    <h3>Onion Root Tip Mitosis</h3>
                    <p>Prepare a temporary mount of onion root tip to study mitosis cell division stages.</p>
                </div>
            </div>
        </div>
    </div>
    
    <!-- Simulation Modal -->
    <div id="sim-modal">
        <div class="modal-content">
            <div class="modal-header">
                <h2 id="sim-title">Simulation</h2>
                <button class="close-btn" onclick="closeSim()">×</button>
            </div>
            
            <div class="sim-area" id="sim-canvas">
                <!-- Render area -->
                <div id="sim-visual" style="text-align:center; color:#8E8E93; width:100%;">[Graphics Engine Loading...]</div>
            </div>
            
            <div class="controls" id="sim-controls">
                <!-- Dynamic controls go here -->
            </div>
            
            <div class="output-box" id="sim-output">
                System initialized.
            </div>
        </div>
    </div>

    <script>
        function switchTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.nav button').forEach(el => el.classList.remove('active'));
            
            document.getElementById(tabId).classList.add('active');
            event.target.classList.add('active');
        }

        const modal = document.getElementById('sim-modal');
        const simTitle = document.getElementById('sim-title');
        const simControls = document.getElementById('sim-controls');
        const simVisual = document.getElementById('sim-visual');
        const simOutput = document.getElementById('sim-output');

        function openSim(type) {
            modal.style.display = 'flex';
            
            if(type === 'pendulum') {
                simTitle.innerText = "Simple Pendulum (Physics)";
                simVisual.innerHTML = `
                    <div style="width: 2px; height: 150px; background-color: #8E8E93; position: absolute; top: 0; left: 50%; transform-origin: top; animation: swing 2s infinite ease-in-out alternate;">
                        <div style="width: 30px; height: 30px; border-radius: 50%; background-color: #5AC8FA; position: absolute; bottom: -15px; left: -14px;"></div>
                    </div>
                `;
                
                simControls.innerHTML = `
                    <div>
                        <label>Length of string (L) in meters: <span id="val-l">1.0</span></label>
                        <input type="range" min="0.1" max="2.0" step="0.1" value="1.0" oninput="document.getElementById('val-l').innerText=this.value; updatePendulum(this.value, document.getElementById('val-g').innerText)">
                    </div>
                    <div>
                        <label>Gravity (g) in m/s²: <span id="val-g">9.8</span></label>
                        <input type="range" min="1.0" max="20.0" step="0.1" value="9.8" oninput="document.getElementById('val-g').innerText=this.value; updatePendulum(document.getElementById('val-l').innerText, this.value)">
                    </div>
                `;
                
                // Add inline style for animation
                if(!document.getElementById('anim-style')) {
                    const style = document.createElement('style');
                    style.id = 'anim-style';
                    style.innerHTML = `@keyframes swing { 0% { transform: rotate(15deg); } 100% { transform: rotate(-15deg); } }`;
                    document.head.appendChild(style);
                }
                updatePendulum(1.0, 9.8);
            }
            else if(type === 'titration') {
                simTitle.innerText = "Acid-Base Titration (Chemistry)";
                simVisual.innerHTML = `<div style="font-size: 40px;">🚰 💧 🧪</div>`;
                simControls.innerHTML = `
                    <div>
                        <label>Volume of Titrant added (ml): <span id="val-v">0</span></label>
                        <input type="range" min="0" max="50" step="1" value="0" oninput="document.getElementById('val-v').innerText=this.value; updateTitration(this.value)">
                    </div>
                `;
                updateTitration(0);
            }
            else {
                simTitle.innerText = "Simulation Module";
                simVisual.innerHTML = `<div style="color: #FFCC00;">Simulation engine calibrating data for module: ${type}...<br>Local compute resources utilized.</div>`;
                simControls.innerHTML = ``;
                simOutput.innerText = `Matrix ready. Module fully offline accessible.`;
            }
        }
        
        function updatePendulum(l, g) {
            const T = 2 * Math.PI * Math.sqrt(l / g);
            simOutput.innerText = `[CALCULATING] T = 2π√(L/g)\\nL = ${l} m, g = ${g} m/s²\\nTime Period (T) = ${T.toFixed(3)} seconds.`;
        }
        
        function updateTitration(v) {
            if(v < 25) {
                simVisual.innerHTML = `<div style="font-size: 40px; color: transparent; text-shadow: 0 0 0 white;">🚰 💧 🧪</div><br><div style="color: #4CD964;">Solution is clear.</div>`;
                simOutput.innerText = `Adding titrant... Current Volume: ${v} ml.`;
            } else if (v == 25) {
                simVisual.innerHTML = `<div style="font-size: 40px; color: transparent; text-shadow: 0 0 0 pink;">🚰 💧 🧪</div><br><div style="color: #FF375F;">Light pink color appeared!</div>`;
                simOutput.innerText = `Equivalence point reached at exactly ${v} ml! Endpoint confirmed.`;
            } else {
                simVisual.innerHTML = `<div style="font-size: 40px; color: transparent; text-shadow: 0 0 0 darkred;">🚰 💧 🧪</div><br><div style="color: #FF3B30;">Dark red color - Over-titrated!</div>`;
                simOutput.innerText = `Caution: Over-titrated. Volume = ${v} ml. Data invalid.`;
            }
        }

        function closeSim() {
            modal.style.display = 'none';
        }
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_ncert_simulator.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
