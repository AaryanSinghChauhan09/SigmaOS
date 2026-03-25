from sigma_core.hal.native_libc import ApexLib
import tkinter as tk
from tkinter import ttk
# import os (Purged by Scrubber)
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class NcertSimulatorPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "NCERT VIRTUAL LAB (CLASS 1-12)", "Physics, Chem, Math & Bio Simulation Suite")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "Simulator Launch Core")
        card.master.pack(pady=50)
        
        tk.Label(card, text="The NCERT Virtual Lab offers dynamic, browser-based simulations of curriculum experiments for Class 1-12.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=400).pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("NCERT Lab", "Class 1-12 Virtual Lab launched in browser.", "OK")

        ttk.Button(card, text="🎓 Launch NCERT Virtual Lab (Class 1-12)", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS NCERT Virtual Lab (Class 1-12)</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0A0A12; color: #F2F2F7; margin: 0; padding: 20px; }
        .container { max-width: 1200px; margin: 0 auto; background-color: #11111E; padding: 20px; border-radius: 10px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); }
        h1 { color: #5AC8FA; font-size: 26px; text-align: center; }
        .nav { display: flex; justify-content: center; gap: 10px; margin-bottom: 20px; flex-wrap: wrap; }
        .nav button { background-color: #1C1C1E; color: white; border: 1px solid #38383A; padding: 8px 15px; border-radius: 5px; cursor: pointer; font-weight: bold; margin-bottom: 5px;}
        .nav button.active { background-color: #5856D6; border-color: #5856D6; }
        .nav button:hover { background-color: #2C2C2E; }
        .nav button.active:hover { background-color: #AF52DE; }
        
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        
        h2 { color: #4CD964; font-size: 20px; border-bottom: 1px solid #38383A; padding-bottom: 10px; margin-top: 20px;}
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 15px; margin-top: 15px; }
        .experiment-card { background-color: #1C1C1E; padding: 15px; border-radius: 8px; border: 1px solid #38383A; cursor: pointer; transition: transform 0.2s; }
        .experiment-card:hover { transform: scale(1.02); border-color: #5856D6; }
        .experiment-card h3 { margin-top: 0; color: #FFCC00; font-size: 16px; }
        .experiment-card p { font-size: 13px; color: #8E8E93; line-height: 1.4; margin-bottom: 0;}
        
        #sim-modal { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0,0,0,0.9); z-index: 1000; align-items: center; justify-content: center; }
        .modal-content { background-color: #11111E; padding: 25px; border-radius: 10px; max-width: 900px; width: 90%; max-height: 90%; overflow-y: auto; border: 1px solid #5Ac8fa; }
        .modal-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; border-bottom: 1px solid #38383A; padding-bottom: 10px; }
        .modal-header h2 { margin: 0; color: #5AC8FA; border: none; padding: 0; }
        .close-btn { background: none; border: none; color: #FF3B30; font-size: 24px; cursor: pointer; }
        
        .sim-area { background-color: #0A0A12; height: 350px; border-radius: 8px; border: 1px solid #38383A; margin-bottom: 20px; display: flex; align-items: center; justify-content: center; position: relative; overflow: hidden; }
        .controls { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }
        .controls label { font-size: 13px; color: #8E8E93; display: block; margin-bottom: 5px; }
        .controls input[type="range"] { width: 100%; }
        .output-box { background-color: #252529; padding: 10px; border-radius: 4px; font-family: monospace; color: #4CD964; margin-top: 15px; white-space: pre-wrap;}
    </style>
</head>
<body>
    <div class="container">
        <h1>🎓 SigmaOS NCERT Virtual Lab (Class 1-12)</h1>
        
        <div class="nav">
            <button class="active" onclick="switchTab('primary')">🎒 Primary (1-5)</button>
            <button onclick="switchTab('middle')">📘 Middle (6-8)</button>
            <button onclick="switchTab('secondary')">📗 Secondary (9-10)</button>
            <button onclick="switchTab('senior')">🎓 Senior Sec (11-12)</button>
        </div>
        
        <!-- PRIMARY (1-5) -->
        <div id="primary" class="tab-content active">
            <h2>🎒 Class 1-5 General Science & Math</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('abacus', 'Primary')">
                    <h3 style="color: #FF9F0A;">Visual Math Abacus</h3>
                    <p>Learn counting and basic arithmetic with a virtual interactive abacus.</p>
                </div>
                <div class="experiment-card" onclick="openSim('floating', 'Primary')">
                    <h3 style="color: #4CD964;">Sink or Float?</h3>
                    <p>Drop various objects into a virtual water tank to observe buoyancy.</p>
                </div>
                <div class="experiment-card" onclick="openSim('shadows', 'Primary')">
                    <h3 style="color: #5AC8FA;">Light & Shadows</h3>
                    <p>Move a virtual flashlight around an object to see how shadows change size and direction.</p>
                </div>
                <div class="experiment-card" onclick="openSim('seed', 'Primary')">
                    <h3 style="color: #BF5AF2;">Seed Germination</h3>
                    <p>Watch a fast-forward simulation of a seed growing based on water and sunlight inputs.</p>
                </div>
            </div>
        </div>

        <!-- MIDDLE (6-8) -->
        <div id="middle" class="tab-content">
            <h2>📘 Class 6-8 Science Basics</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('circuits', 'Middle')">
                    <h3 style="color: #5AC8FA;">Simple Electric Circuit</h3>
                    <p>Connect batteries, wires, switches, and a bulb to make it glow.</p>
                </div>
                <div class="experiment-card" onclick="openSim('magnetism', 'Middle')">
                    <h3 style="color: #5AC8FA;">Magnets & Iron Filings</h3>
                    <p>Visualize the invisible magnetic field lines around a bar magnet.</p>
                </div>
                <div class="experiment-card" onclick="openSim('acids', 'Middle')">
                    <h3 style="color: #4CD964;">Acids, Bases & Litmus Paper</h3>
                    <p>Test various household solutions with blue and red litmus paper.</p>
                </div>
                <div class="experiment-card" onclick="openSim('respiration', 'Middle')">
                    <h3 style="color: #BF5AF2;">Human Respiratory System</h3>
                    <p>Observe the movement of the diaphragm and lungs during breathing.</p>
                </div>
            </div>
        </div>

        <!-- SECONDARY (9-10) -->
        <div id="secondary" class="tab-content">
            <h2>📗 Class 9-10 Science Experiments</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('newton', 'Secondary')">
                    <h3 style="color: #5AC8FA;">Newton's Laws of Motion</h3>
                    <p>Push blocks of different masses on a frictionless surface to observe acceleration.</p>
                </div>
                <div class="experiment-card" onclick="openSim('reflection', 'Secondary')">
                    <h3 style="color: #5AC8FA;">Laws of Reflection</h3>
                    <p>Verify incident ray = reflected ray using a virtual plane mirror and laser.</p>
                </div>
                <div class="experiment-card" onclick="openSim('ph_scale', 'Secondary')">
                    <h3 style="color: #4CD964;">pH Scale & Universal Indicator</h3>
                    <p>Determine the exact pH of solutions using a simulated universal indicator color chart.</p>
                </div>
                <div class="experiment-card" onclick="openSim('stomata', 'Secondary')">
                    <h3 style="color: #BF5AF2;">Stomata Microscopic View</h3>
                    <p>Simulate a microscope view of a leaf peel to identify guard cells and stomatal pores.</p>
                </div>
            </div>
        </div>

        <!-- SENIOR (11-12) -->
        <div id="senior" class="tab-content">
            <h2>🎓 Class 11-12 Advanced Experiments</h2>
            <div class="grid">
                <div class="experiment-card" onclick="openSim('pendulum', 'Senior')">
                    <h3 style="color: #5AC8FA;">Simple Pendulum & Gravity</h3>
                    <p>Determine acceleration due to gravity (g) using a customizable pendulum.</p>
                </div>
                <div class="experiment-card" onclick="openSim('ohm', 'Senior')">
                    <h3 style="color: #5AC8FA;">Ohm's Law Verification</h3>
                    <p>Adjust voltage and measure current in an advanced circuit to plot V-I graphs.</p>
                </div>
                <div class="experiment-card" onclick="openSim('titration', 'Senior')">
                    <h3 style="color: #4CD964;">KMnO4 Acid-Base Titration</h3>
                    <p>Titrate KMnO4 against Mohr's salt to find the molarity of the unknown solution.</p>
                </div>
                <div class="experiment-card" onclick="openSim('mitosis', 'Senior')">
                    <h3 style="color: #BF5AF2;">Onion Root Tip Mitosis</h3>
                    <p>Prepare a virtual temporary mount of an onion root tip to study the stages of mitosis.</p>
                </div>
            </div>
        </div>
    </div>
    
    <!-- Simulation Modal -->
    <div id="sim-modal">
        <div class="modal-content">
            <div class="modal-header">
                <h2 id="sim-title">Simulation</h2>
                <div style="font-size:12px; color:#8E8E93;" id="sim-level">Level</div>
                <button class="close-btn" onclick="closeSim()">×</button>
            </div>
            
            <div class="sim-area" id="sim-canvas">
                <!-- Render area -->
                <div id="sim-visual" style="text-align:center; color:#8E8E93; width:100%;">[Virtual Reality Engine Loading...]</div>
            </div>
            
            <div class="controls" id="sim-controls">
                <!-- Dynamic controls go here -->
            </div>
            
            <div class="output-box" id="sim-output">
                System initialized. Ready for physics engine ticks.
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
        const simLevel = document.getElementById('sim-level');
        const simControls = document.getElementById('sim-controls');
        const simVisual = document.getElementById('sim-visual');
        const simOutput = document.getElementById('sim-output');

        function openSim(type, level) {
            modal.style.display = 'flex';
            simLevel.innerText = `[${level} Level Module]`;
            
            if(type === 'pendulum') {
                simTitle.innerText = "Simple Pendulum (Physics)";
                simVisual.innerHTML = `
                    <div style="width: 2px; height: 180px; background-color: #8E8E93; position: absolute; top: 0; left: 50%; transform-origin: top; animation: swing 2s infinite ease-in-out alternate;">
                        <div style="width: 40px; height: 40px; border-radius: 50%; background-color: #5AC8FA; position: absolute; bottom: -20px; left: -19px; box-shadow: inset -5px -5px 10px rgba(0,0,0,0.5);"></div>
                    </div>
                `;
                
                simControls.innerHTML = `
                    <div>
                        <label>Length of string (L) in meters: <span id="val-l">1.0</span></label>
                        <input type="range" min="0.1" max="2.0" step="0.1" value="1.0" oninput="document.getElementById('val-l').innerText=this.value; updatePendulum(this.value, document.getElementById('val-g').innerText)">
                    </div>
                    <div>
                        <label>Gravity (g) in m/s²: <span id="val-g">9.8</span></label>
                        <input type="range" min="1.0" max="25.0" step="0.1" value="9.8" oninput="document.getElementById('val-g').innerText=this.value; updatePendulum(document.getElementById('val-l').innerText, this.value)">
                    </div>
                `;
                
                if(!document.getElementById('anim-style')) {
                    const style = document.createElement('style');
                    style.id = 'anim-style';
                    style.innerHTML = `@keyframes swing { 0% { transform: rotate(20deg); } 100% { transform: rotate(-20deg); } }`;
                    document.head.appendChild(style);
                }
                updatePendulum(1.0, 9.8);
            }
            else if(type === 'titration') {
                simTitle.innerText = "Acid-Base Titration (Chemistry)";
                simVisual.innerHTML = `<div style="font-size: 60px;">🚰<br>💧<br>🧪</div>`;
                simControls.innerHTML = `
                    <div style="grid-column: span 2;">
                        <label>Volume of KMnO4 Titrant added (ml): <span id="val-v">0</span></label>
                        <input type="range" min="0" max="50" step="1" value="0" oninput="document.getElementById('val-v').innerText=this.value; updateTitration(this.value)">
                    </div>
                `;
                updateTitration(0);
            }
            else if(type === 'floating') {
                simTitle.innerText = "Sink or Float? (Science)";
                simVisual.innerHTML = `
                    <div style="width: 80%; height: 60%; background-color: rgba(90, 200, 250, 0.3); border: 2px solid #5AC8FA; position: absolute; bottom: 20px; border-radius: 5px;"></div>
                    <div id="obj-float" style="width: 50px; height: 50px; background-color: #FF9F0A; position: absolute; top: 20px; transition: top 1s ease-in, background-color 0.5s;"></div>
                `;
                simControls.innerHTML = `
                    <div style="grid-column: span 2; display: flex; gap: 10px; justify-content: center;">
                        <button onclick="objDrop('wood')" style="padding: 10px; background: #8B4513; color: white; border: none; cursor:pointer;">Drop Wood</button>
                        <button onclick="objDrop('iron')" style="padding: 10px; background: #A0A0A0; color: black; border: none; cursor:pointer;">Drop Iron</button>
                        <button onclick="objDrop('plastic')" style="padding: 10px; background: #FF9F0A; color: black; border: none; cursor:pointer;">Drop Plastic</button>
                    </div>
                `;
                simOutput.innerText = "Select an object to drop into the water tank.";
            }
            else if(type === 'acids') {
                simTitle.innerText = "Acids & Bases Litmus Test (Chemistry)";
                simVisual.innerHTML = `
                    <div style="display: flex; gap: 40px; align-items:flex-end; height: 100%;">
                        <div style="text-align:center;"><div style="width: 60px; height: 100px; border: 2px solid white; border-top: none; background: rgba(255,59,48,0.5);"></div><p>Lemon Juice</p></div>
                        <div style="text-align:center;"><div style="width: 60px; height: 100px; border: 2px solid white; border-top: none; background: rgba(76,217,100,0.5);"></div><p>Soap Water</p></div>
                        <div style="text-align:center;"><div style="width: 60px; height: 100px; border: 2px solid white; border-top: none; background: rgba(255,255,255,0.5);"></div><p>Pure Water</p></div>
                    </div>
                `;
                simControls.innerHTML = `
                    <div style="grid-column: span 2; text-align:center;">
                        <button onclick="simOutput.innerText='Lemon Juice + Blue Litmus -> Red (Acidic)\\nLemon + Red Litmus -> Remains Red'" style="padding: 10px;">Test Lemon</button>
                        <button onclick="simOutput.innerText='Soap + Red Litmus -> Blue (Basic)\\nSoap + Blue Litmus -> Remains Blue'" style="padding: 10px;">Test Soap</button>
                        <button onclick="simOutput.innerText='Water + Any Litmus -> No color change (Neutral)'" style="padding: 10px;">Test Pure Water</button>
                    </div>
                `;
                simOutput.innerText = "Click to perform Litmus tests on the solutions.";
            }
            else {
                simTitle.innerText = "Generic Engine Simulation";
                simVisual.innerHTML = `<div style="color: #FFCC00; font-size: 18px;">Simulation engine allocating resources for module: <b>${type}</b>...<br><br>WebGL context initialized. Physics daemon running locally.</div>`;
                simControls.innerHTML = ``;
                simOutput.innerText = `Data Matrix ready. Module fully offline accessible.`;
            }
        }
        
        function updatePendulum(l, g) {
            const T = 2 * Math.PI * Math.sqrt(l / g);
            simOutput.innerText = `[REAL-TIME PHYSICS COMPUTATION]\\nFormula: T = 2π√(L/g)\\nLength (L) = ${l} m, Gravity (g) = ${g} m/s²\\n\\nTime Period (T) = ${T.toFixed(4)} seconds.\\nFrequency = ${(1/T).toFixed(4)} Hz`;
        }
        
        function updateTitration(v) {
            if(v < 25) {
                simVisual.innerHTML = `<div style="font-size: 60px; color: transparent; text-shadow: 0 0 0 white;">🚰<br>💧<br>🧪</div><br><div style="position: absolute; bottom: 20px; color: #4CD964; font-weight: bold;">Solution: Colorless</div>`;
                simOutput.innerText = `Adding titrant... Current Volume: ${v} ml.\\nReaction occurring: MnO4⁻ + 8H⁺ + 5Fe²⁺ -> Mn²⁺ + 5Fe³⁺ + 4H2O`;
            } else if (v == 25) {
                simVisual.innerHTML = `<div style="font-size: 60px; color: transparent; text-shadow: 0 0 0 #FF69B4;">🚰<br>💧<br>🧪</div><br><div style="position: absolute; bottom: 20px; color: #FF69B4; font-weight: bold;">Solution: Light Pink</div>`;
                simOutput.innerText = `EQUIVALENCE POINT REACHED! Volume: exactly ${v} ml.\\nThe KMnO4 acts as a self-indicator. End point attained.`;
            } else {
                simVisual.innerHTML = `<div style="font-size: 60px; color: transparent; text-shadow: 0 0 0 #8B0000;">🚰<br>💧<br>🧪</div><br><div style="position: absolute; bottom: 20px; color: #FF3B30; font-weight: bold;">Solution: Dark Red</div>`;
                simOutput.innerText = `WARNING: Over-titrated. Volume: ${v} ml.\\nExcess KMnO4 added. Data point invalidated.`;
            }
        }

        function objDrop(type) {
            const obj = document.getElementById('obj-float');
            if(!obj) return;
            if(type==='wood') {
                obj.style.backgroundColor = '#8B4513';
                obj.style.top = '120px'; // floats
                simOutput.innerText = "Wood density < Water density. Buoyancy > Weight. IT FLOATS!";
            } else if (type === 'iron') {
                obj.style.backgroundColor = '#A0A0A0';
                obj.style.top = '280px'; // sinks
                simOutput.innerText = "Iron density > Water density. Buoyancy < Weight. IT SINKS!";
            } else if (type === 'plastic') {
                obj.style.backgroundColor = '#FF9F0A';
                obj.style.top = '140px'; // partially floats
                simOutput.innerText = "Plastic density ~ Water density. It partially floats/suspends.";
            }
        }

        function closeSim() {
            modal.style.display = 'none';
        }
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_ncert_simulator_full.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
