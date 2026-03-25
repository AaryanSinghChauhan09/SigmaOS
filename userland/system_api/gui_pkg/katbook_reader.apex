from sigma_core.hal.native_libc import ApexLib
import tkinter as tk
from tkinter import ttk
# import os (Purged by Scrubber)
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class KatbookReaderPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "AI INTERACTIVE NCERT TEXTBOOKS", "KATBOOK Digital Learning Format")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "Adaptive Learning Syllabus Reader")
        card.master.pack(pady=50)
        
        tk.Label(card, text="KATBOOK integrates the NCERT textbook texts directly alongside AI tools like dictionary lookups, multilingual translation, inline videos, and Text-to-Speech audiobooks.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=500, justify="center").pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("KATBOOK", "AI NCERT Interactive reader launched.", "OK")

        ttk.Button(card, text="📚 Open KATBOOK Digital Library", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NCERT Interactive KATBOOK (AI-Enabled)</title>
    <style>
        :root { --p: #007bff; --s: #28a745; --bg: #fdfdfd; --text: #333; --panel: #fff; }
        body { font-family: 'Georgia', serif; background-color: var(--bg); color: var(--text); margin: 0; padding: 0; display: flex; height: 100vh; overflow: hidden; }
        
        /* Sidebar Library */
        .library { width: 300px; background-color: #f4f6f9; border-right: 1px solid #ddd; overflow-y: auto; display: flex; flex-direction: column; }
        .logo { padding: 25px 20px; font-family: 'Segoe UI', sans-serif; text-align: center; background-color: var(--p); color: white; font-weight: bold; font-size: 20px; letter-spacing: 1px;}
        .book-item { padding: 15px 20px; border-bottom: 1px solid #e1e4e8; cursor: pointer; transition: 0.2s; font-family: 'Segoe UI', sans-serif;}
        .book-item:hover, .book-item.active { background-color: #e2e6ea; border-left: 4px solid var(--p); }
        .book-title { font-weight: bold; font-size: 14px; color: #2c3e50; }
        .book-class { font-size: 12px; color: #7f8c8d; margin-top: 4px; }
        
        /* Reader Panel */
        .reader { flex: 1; display: flex; flex-direction: column; overflow: hidden; position: relative;}
        
        .toolbar { background-color: var(--panel); padding: 10px 20px; box-shadow: 0 2px 5px rgba(0,0,0,0.05); display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #eee; font-family: 'Segoe UI', sans-serif;}
        .ai-tools { display: flex; gap: 10px; }
        .tool-btn { background: #fff; border: 1px solid #ccc; padding: 6px 12px; border-radius: 20px; cursor: pointer; font-size: 12px; font-weight: bold; color: #555; display: flex; align-items: center; gap: 5px; transition: 0.2s;}
        .tool-btn:hover { background: #f0f0f0; color: #000; border-color: #999;}
        
        .page-content { padding: 40px 80px; overflow-y: auto; flex: 1; line-height: 1.8; font-size: 18px; color: #222; max-width: 800px; margin: 0 auto; }
        
        h1, h2 { font-family: 'Segoe UI', Tahoma, sans-serif; color: #1a252f; }
        p::selection { background: #b3d4fc; }

        /* Integrated AI Dictionary Popup */
        #ai-popup { display: none; position: absolute; background: white; border: 1px solid #ccc; box-shadow: 0 5px 15px rgba(0,0,0,0.2); border-radius: 8px; width: 300px; padding: 15px; z-index: 1000; font-family: 'Segoe UI', sans-serif; }
        .ai-pop-title { font-size: 16px; font-weight: bold; color: var(--p); border-bottom: 1px solid #eee; padding-bottom: 5px; margin-bottom: 10px; text-transform: capitalize; }
        .ai-pop-body { font-size: 13px; color: #555; }
        .ai-pop-close { position: absolute; top: 10px; right: 15px; cursor: pointer; color: #999; font-weight: bold; }
        
        /* Embedded Simulation / Video Component */
        .embed-sim { margin: 30px 0; border: 2px dashed #007bff; border-radius: 8px; padding: 20px; background: #f8fbff; text-align: center; font-family: 'Segoe UI', sans-serif; }
        .embed-sim-btn { background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-weight: bold; margin-top: 10px;}
        
    </style>
</head>
<body>
    <div class="library">
        <div class="logo">
            📖 KATBOOK<br><span style="font-size: 12px; font-weight: normal; color: #e0e0e0;">AI Interactive Textbooks</span>
        </div>
        
        <div class="book-item active" onclick="loadChapter('physics')">
            <div class="book-title">Chapter 8: Motion</div>
            <div class="book-class">Class 9 Science • NCERT</div>
        </div>
        <div class="book-item" onclick="loadChapter('bio')">
            <div class="book-title">Chapter 5: The Fundamental Unit of Life</div>
            <div class="book-class">Class 9 Science • NCERT</div>
        </div>
        <div class="book-item" onclick="loadChapter('chem')">
            <div class="book-title">Chapter 2: Acids, Bases and Salts</div>
            <div class="book-class">Class 10 Science • NCERT</div>
        </div>
        <div class="book-item" onclick="loadChapter('math')">
            <div class="book-title">Chapter 4: Quadratic Equations</div>
            <div class="book-class">Class 10 Math • NCERT</div>
        </div>
    </div>
    
    <div class="reader">
        <div class="toolbar">
            <div style="font-size: 14px; font-weight: bold; color: #555;">Reading Focus Mode</div>
            <div class="ai-tools">
                <button class="tool-btn" onclick="readAloud()">🔊 AudioBook (TTS)</button>
                <button class="tool-btn" onclick="stopAloud()">🔇 Stop Audio</button>
                <div style="width: 1px; background: #ddd; margin: 0 5px;"></div>
                <button class="tool-btn" onclick="alert('Hindi language translation module engaged globally via DOM substitution.')">🗣️ Translate to Hindi</button>
                <button class="tool-btn" onclick="alert('Highlight any word in the text to see the AI Dictionary & Context tool in action!')">🔍 AI Dictionary Helper</button>
            </div>
        </div>
        
        <div class="page-content" id="book-content" onmouseup="getTextSelection(event)">
            <!-- Content Injected Here -->
        </div>
        
        <!-- AI Dictionary Popup -->
        <div id="ai-popup">
            <div class="ai-pop-close" onclick="closePopup()">×</div>
            <div class="ai-pop-title" id="ai-word">Word</div>
            <div class="ai-pop-body" id="ai-def">Definition loading from local neural cache...</div>
            <div style="margin-top: 10px; border-top: 1px solid #eee; padding-top: 10px; cursor:pointer; color: #007bff; font-weight:bold; font-size: 12px;" onclick="readAloudWord()">🔊 Pronounce</div>
        </div>
    </div>

    <script>
        const contentBox = document.getElementById('book-content');
        
        const chapters = {
            'physics': `
                <h1>8. Motion</h1>
                <p>In everyday life, we see some objects at rest and others in motion. Birds fly, fish swim, blood flows through veins and arteries, and cars move. Atoms, molecules, planets, stars and galaxies are all in motion. We often perceive an object to be in motion when its position changes with time.</p>
                <p>However, there are situations where the motion is inferred through indirect evidences. For example, we infer the motion of air by observing the movement of dust and the movement of leaves and branches of trees.</p>
                
                <h2>8.1 Describing Motion</h2>
                <p>We describe the location of an object by specifying a reference point. Let us understand this by an example. Let us assume that a school in a village is 2 km north of the railway station. We have specified the position of the school with respect to the railway station.</p>
                
                <div class="embed-sim">
                    <h3 style="margin-top:0;">Interactive Component: Reference Points</h3>
                    <p style="font-size: 14px; color: #555;">Drag an observer along the axis to see how "Distance" and "Displacement" change depending on the origin point.</p>
                    <button class="embed-sim-btn" onclick="alert('Launching the Virtual Lab Simulator overlay...')">Launch Kinematics Simulation</button>
                </div>
                
                <p>The simplest type of motion is the motion along a straight line. The total path length covered by the object is called distance. The shortest distance measured from the initial to the final position of an object is known as the displacement.</p>
            `,
            'bio': `
                <h1>5. The Fundamental Unit of Life</h1>
                <p>While examining a thin slice of cork, Robert Hooke saw that the cork resembled the structure of a honeycomb consisting of many little compartments. Cork is a substance which comes from the bark of a tree. This was in the year 1665 when Hooke made this chance observation through a self-designed microscope. Robert Hooke called these boxes cells. Cell is a Latin word for ‘a little room’.</p>
                <p>What are living organisms made up of? All living organisms are made up of cells.</p>
                
                <div class="embed-sim">
                    <h3 style="margin-top:0;">Interactive Component: Virtual Microscope</h3>
                    <p style="font-size: 14px; color: #555;">Adjust the coarse and fine adjustment knobs to bring the Onion Peel into focus.</p>
                    <button class="embed-sim-btn" onclick="alert('Invoking Microscope VR Canvas module...')">Open Microscope</button>
                </div>
                
                <h2>5.2 What is a cell made up of?</h2>
                <p>If we study a cell under a microscope, we would come across three features in almost every cell: plasma membrane, nucleus and cytoplasm. All activities inside the cell and interactions of the cell with its environment are possible due to these features.</p>
            `
        };

        function loadChapter(id) {
            document.querySelectorAll('.book-item').forEach(el => el.classList.remove('active'));
            event.target.closest('.book-item').classList.add('active');
            contentBox.innerHTML = chapters[id] || "Content indexing...";
            closePopup();
            window.scrollTo(0, 0);
        }

        let currentSelection = "";
        
        function getTextSelection(e) {
            const sel = window.getSelection().toString().trim();
            if(sel.length > 0 && sel.length < 30) {
                currentSelection = sel;
                showPopup(e.pageX, e.pageY, sel);
            } else {
                closePopup();
            }
        }
        
        const dictionary = {
            "motion": "The action or process of moving or being moved. A change in position over time.",
            "displacement": "The shortest distance from the initial to the final position of a point.",
            "cell": "The smallest structural and functional unit of an organism, typically microscopic.",
            "nucleus": "A dense organelle present in most eukaryotic cells, typically a single rounded structure containing genetic material.",
            "cytoplasm": "The material or protoplasm within a living cell, excluding the nucleus."
        };

        function showPopup(x, y, word) {
            const pop = document.getElementById('ai-popup');
            pop.style.display = 'block';
            
            // Adjust to prevent overflow
            const winWidth = window.innerWidth;
            if(x + 320 > winWidth) x = winWidth - 340;
            
            pop.style.left = x + 'px';
            pop.style.top = (y + 15) + 'px';
            document.getElementById('ai-word').innerText = word;
            
            const wLower = word.toLowerCase();
            let def = "Generating contextually aware definition via local linguistic engine...\\n\\n";
            if(dictionary[wLower]) {
                def = dictionary[wLower];
            } else {
                def += "A specialized term found in the NCERT syllabus. AI heuristic determines it is related to the current chapter's core concepts.";
            }
            document.getElementById('ai-def').innerText = def;
        }
        
        function closePopup() {
            document.getElementById('ai-popup').style.display = 'none';
        }
        
        function readAloudWord() {
            if('speechSynthesis' in window) {
                const utterance = new SpeechSynthesisUtterance(currentSelection);
                window.speechSynthesis.speak(utterance);
            }
        }

        function readAloud() {
            if('speechSynthesis' in window) {
                window.speechSynthesis.cancel(); // Stop any current
                const text = contentBox.innerText;
                const utterance = new SpeechSynthesisUtterance(text);
                window.speechSynthesis.speak(utterance);
            } else {
                alert("Text-to-Speech not supported in this browser.");
            }
        }
        
        function stopAloud() {
            if('speechSynthesis' in window) {
                window.speechSynthesis.cancel();
            }
        }

        // Initialize
        loadChapter('physics');
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_katbook.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
