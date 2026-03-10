"""
SigmaOS Sovereign v3.0 Apex — Universal User Manual (Sovereign-Ready)
====================================================================
A comprehensive, searchable, and interactive manual program for SigmaOS.
Supports both Apex (Professional) and Novice (Simplified) modes.
"""

from typing import Dict, List, Any

class SigmaManual:
    def __init__(self):
        self.MANUAL_DATA = {
            "Getting Started": {
                "Welcome": (
                    "Welcome to SigmaOS Sovereign V4.0. "
                    "The world's first independent, AI-native meta-OS. "
                    "\n\n  Σ I G M A   O S   |   S O V E R E I G N  \n"
                    "  [ OmniBoot | MeshIntel | CoreBrain ]\n\n"
                    "You are now operating within a secure, sharded ecosystem. "
                    "Go to the 'Sovereign Dashboard' (Win+D) for a status overview."
                ),
                "Introduction": "SigmaOS is a sovereign, agentic operating environment designed for privacy, performance, and automation.",
                "Novice vs Apex Mode": "Toggle 'Novice Mode' in the top right to switch between simplified everyday language and professional technical jargon.",
                "Dashboard": "Your central hub for system stats (Boot Time, RAM, Security) and quick actions."
            },
            "Neural Fabric (Performance)": {
                "What is it?": "The system's brain. It predicts your next move and warms up apps before you even click them.",
                "Performance Profiles": "Includes Efficiency Max (Battery), Mesh Pooling (Maximum Power), and Local Hardened (Privacy/Local Computing).",
                "Neural Prefetch": "Allows the OS to intelligently preload work environments based on 'Work', 'Creative', or 'Gaming' intents."
            },
            "Omni Automator (Easy Routines)": {
                "Agentic Missions": "Complex tasks like 'Forensic Audit' or 'Web Mission' that the OS plans and executes autonomously.",
                "Apex Modes": "12+ pre-defined routines (Sovereign Work, Night Owl, Digital Detox) that orchestrate the entire OS with one click.",
                "Scratch Blocks": "A visual logic builder based on MIT Scratch. Drag and drop triggers, actions, and logic to create custom automations."
            },
            "Content Forge (My Files)": {
                "Sovereign Ingest": "Securely process PDFs, images, and text files. Includes OCR and PII (Personal Information) redaction by default.",
                "Universal Conversion": "Convert any file type to any other type (PDF -> Markdown, Excel -> JSON) offline and securely.",
                "Audit & Ledger": "Tracks every transformation to ensure file integrity and security."
            },
            "Aura Mesh (Network)": {
                "P2P Lattice": "Connect directly to other SigmaOS devices without a central server. Share CPU, RAM, and storage securely.",
                "Mesh Broadcast": "Instantly send status updates or data packets across your local device cluster.",
                "Zero-Trust": "Every connection is verified via PQC (Post-Quantum Cryptography) for maximum security."
            },
            "Future-Ready OS Features": {
                "Adaptive Intelligence": "SigmaOS predicts user needs and pre-loads relevant environments/apps automatically.",
                "Mood-Based Customization": "Wallpapers, themes, and soundscapes morph according to your current emotional state.",
                "Sovereign Security Sandbox": "Launch any untrusted program in a totally isolated, RAM-only container.",
                "Sustainability (Eco-Mode)": "Carbon-aware task scheduling to minimize energy footprints during high-intensity compute.",
                "Universal Automation Dashboard": "Synthesized view of Scratch, Samsung Routines, and Automa workflows in one place."
            },
            "Sovereign ML Algorithm Matrix": {
                "Regression (Linear/Logistic)": "The workhorses for predictive modeling and binary classification.",
                "Ensemble (Tree/Forest)": "High-robustness models using decision paths and random bagging for reliable results.",
                "Geometric (SVM/KNN)": "Using max-margin hyperplanes and neighborhood voting for non-linear data clusters.",
                "Probabilistic (Naive Bayes)": "Fast, Bayesian inference models ideal for high-dimensional text and feature sets.",
                "Deep Learning (Neural Nets)": "Sovereign implementation of Multi-Layer Perceptrons with backprop and Adam optimization."
            },
            "AI & ML Thinking Paradigms": {
                "Rule-Based vs. Neural": "Blends symbolic logic for structured decisions with deep learning for pattern recognition.",
                "Reinforcement Learning": "Our kernel learns optimal resource allocation through trial-and-error over time.",
                "Supervised & Probabilistic": "Predict customer churn or system faults based on historical labeled datasets.",
                "Hybrid AI (The Omni-Brain)": "Blending expert systems with generative intelligence for creative yet logical OS tasks."
            },
            "Factors of Decision-Making": {
                "Information & Data": "Timeliness and completeness of telemetry directly impact our automation success rates.",
                "Risk & Resource Management": "Calculating probability of outcomes vs. your tolerance for ambiguity and budget.",
                "Values & Ethics": "Strategic OS decisions must align with fairness, transparency, and integrity.",
                "Group Dynamics": "Integrating consensus and majority rule in Mesh-wide decisions (Social Logic)."
            },
            "Cognitive Decision Models": {
                "Rational Model": "Logical, step-by-step analysis assuming complete information. Benchmarked against strategic planning.",
                "Bounded Rationality": "Satisficing with limited info—identifying 'good enough' solutions under constraints.",
                "Intuitive Engine": "Speed and experience-driven 'gut' feelings for urgent, high-stakes contexts.",
                "Prospect Theory": "Loss-aversion logic and risk-framing filters applied to financial/security blocks.",
                "Garbage Can & Political": "Understanding the chaotic, bargained mix of problems and solutions in mesh groups."
            },
            "Jurisprudence (Legal Logic)": {
                "Natural Law": "Systems aligned with universal moral order and inherent justice.",
                "Legal Positivism": "Strict adherence to Authority and the Rule of Law (Registry/Ledger enforce).",
                "Sociological & Realist": "Pragmatic, welfare-centric logic that adapts to social needs and actual outcomes.",
                "Critical & Historical": "Examining power structures and local traditions to reform and evolve the OS over time."
            },
            "Strategic Thinking Lattice": {
                "Critical & Analytical": "Evidence-backed, precise audits. Zero-fallback logic for system integrity.",
                "Creative & Lateral": "Divergent, innovative brainstorming blocks to find unconventional solutions.",
                "Systems Thinking": "The 'Holistic' lens. Understanding feedback loops and the ripple effects of every action.",
                "Design & Reflective": "Human-centered iteration and learning from every mission's outcome to self-heal and grow."
            },
            "Social Decision Framework": {
                "Individual Authority": "Direct, fast execution by a single person. Ideal for routine or urgent system overrides.",
                "Group Consensus": "Collective agreement required. Every node in the Mesh must verify the action.",
                "Majority Rule (Voter)": "Logic executes once a 51% (or custom) threshold is met across your device cluster.",
                "Committee & Stakeholders": "Formal processes requiring review and participative input from multiple system roles.",
                "Mesh Synchronization": "Bridging individual authority with collective group wisdom for strategic OS scaling."
            },
            "Decision Sciences (The Routine Strategy)": {
                "Programmed Decisions": "Standard rules for predictable tasks (e.g., auto-approving leave or reordering inventory).",
                "Operational Logic": "Day-to-day decisions aimed at process fluidity, often delegated to the Aura-Mesh pool.",
                "Rule-Based (Threshold)": "Algorithms strictly guided by predefined values (e.g., credit checks or automated QA).",
                "SOP Protocol": "Standard Operating Procedures embedded in our logic blocks for documented, repeatable safety checks.",
                "Automated (System-Driven)": "Fully repetitive choices handled by Scratch, Automa, or Samsung-style routines without human input."
            },
            "Sovereign UI/UX Principles": {
                "Visual & Interaction": "Minimalism, Hierarchy, Affordance, and Progressive Disclosure ensure a premium, predictable experience.",
                "Context Awareness": "The UI adapts intelligently based on time, location, and emotional state ('Mood-Based Themes').",
                "Advanced Interactivity": "Direct manipulation via drag-and-drop, gesture support, and immediate visual/haptic feedback.",
                "Accessibility & Privacy": "Support for screen readers, high-contrast modes, and transparent permission-ledger tracking."
            },
            "Adaptive Intelligence (AI Layer)": {
                "Routine Prediction": "The OS analyzes your telemetry to suggest 'Work' or 'Relaxation' modes before you even click them.",
                "Health & Wellness": "Integrated posture detection (via Webcam), hydration streaks, and healthy sleep transitions.",
                "Resource Management": "AI-driven CPU/GPU allocation (zRAM + 4.0 scheduler) for zero-jitter performance.",
                "Gamification": "Points, badges, and streaks integrated across system tasks to turn productivity into progress."
            },
            "Security & Sovereignty": {
                "Zero-Trust Shield": "SigmaOS operates on a default-deny policy. All external traffic is blocked unless explicitly bridged.",
                "Universal Bridge (UAL)": "The ONLY way to run non-sovereign apps. Encapsulates external apps (like VSCode or Photoshop) in a secure wrapper.",
                "Self-Healing": "The OS automatically detects and repairs corrupted system files using the Registry-UAL-Lattice consensus."
            },
            "SigmaLegalPro (The Universal Legal OS)": {
                "Sovereign Research (SCC/Manupatra)": "Unified database for case laws, statutes, and judicial trends with AI-driven CaseIQ relevance.",
                "Practice & Case Mgmt (Clio)": "Full lifecycle tracking: Client database, Billing ledgers, and eCourts CNR status tracking.",
                "Compliance & Risk (VIDUR/MCA21)": "Automated auditing for SEBI, MCA filings, and GST/Tax regulations.",
                "Drafting & DocGen (HotDocs)": "Universal drafting workbench with automated templates for Bail, FIR Writs, and Consumer Notices.",
                "AI Discovery (Kira/Luminance)": "Automated risk review of legal documents to detect missing clauses or risky indemnity terms.",
                "Statutory Auditing (Calculators)": "Real-time computation for FY25 Tax, GST, Gratuity (15/26), and Statutory Bonus."
            },
            "Science & Research (The Scholar Engine)": {
                "Statistical Analysis": "Hypothesis testing (p-value calculation), ANOVA, and Bayesian priors for research integrity.",
                "Computer Science Core": "Distributed consensus, Big-O complexity audits, and graph-based problem solving.",
                "Data Science (EDA)": "Automated profiling, feature engineering, and high-dimensional data encoding (Sovereign Forge).",
                "LLM Orchestration": "RAG (Retrieval Augmented Generation), Prompt-Chaining, and PEFT Fine-tuning (Aura Brain)."
            },
            "Sovereign ERP: The Odoo Alternative": {
                "CRM & Sales": "Track leads, manage customers, and convert opportunities through a private pipeline.",
                "Accounting & Ledger": "Generate invoices and manage your industrial-grade financial records without external cloud audits.",
                "Inventory & MRP": "Automated stock tracking and manufacturing resource planning (MRP) for your production lines.",
                "Project & Task Management": "Kanban-style boards and milestones integrated directly into your OS workspace.",
                "HR & Payroll": "Manage employee profiles, roles, and automated salary disbursements via sovereign protocols."
            },
            "Aura Assistant: Guided Audio Intelligence": {
                "Step-by-Step Approval": "The ONLY audio assistant that requests your PERMISSION before taking any action. You are always the pilot.",
                "Mission Decomposition": "Converts high-level goals into a sequence of verifiable steps for your review.",
                "Auditory Guidance": "Interact with Google Home or Alexa-style ease, but with the power of sovereign system control.",
                "Real-Time Refinement": "Tell the assistant to 'Refine step 2' to dynamically adjust the mission plan on the fly.",
                "HITL (Human-in-the-Loop)": "Zero autonomous actions without deliberate voice or touch approval."
            },
            "AuraVoice: The JARVIS Interface": {
"Ambient Listening": "Wake-up on demand with custom wake-words (default: 'Aura', 'Friday', 'Jarvis').",
                "Intent Recognition": "Advanced NLP that understands complex requests like 'Deploy my work cluster and dim the lights'.",
                "Emotional Sync": "Auditory feedback that adapts its tone based on your detected mood and cortisol levels.",
                "Biometric Voice-ID": "Multi-factor authentication using your unique vocal print. Secure and hands-free.",
                "Sovereign Synthesis": "High-fidelity TTS (Text-to-Speech) profiles for real-time mission reports."
            },
            "Universal Application Layer (The Compatibility Vault)": {
                "Proton-Sigma (Win32/x64)": "Run .exe and .msi files at 98%+ native speed. Full support for DirectX 12 to Vulkan translation.",
                "Retina-Bridge (macOS)": "Seamlessly execute .app and .dmg binaries. Translates AppKit and Metal commands in real-time.",
                "AOSP-Shadow (Android)": "A lightweight Android runtime that sideloads .apk files directly into your sovereign desktop workspace.",
                "Native-POSIX (Linux)": "Zero-latency execution for .deb, .rpm, and raw ELF binaries. Outperforms WSL2 in resource efficiency.",
                "Omni-Shim (Input Mapping)": "Dynamically converts MOUSE input to TOUCH-Taps for Android apps, and STYLUS input for creative Mac software."
            },
            "Logic & Control Flows": {
                "Core Conditionals": (
                    "• If/Else: Standard binary decision.\n"
                    "• Else-If Ladder: Sequence of multiple checks.\n"
                    "• Switch/Case: Selection based on specific values.\n"
                    "• Nested If: Complexity layers for precision control."
                ),
                "Advanced Logic Blocks": (
                    "• Guard Clauses: Swift exits for invalid system states.\n"
                    "• Pattern Match: Structural AI-driven data matching.\n"
                    "• Ternary: Ultra-compact conditional expressions.\n"
                    "• Short-Circuiting: Efficient logical evaluation (AND/OR)."
                ),
                "Domain-Specific Flows": (
                    "• Event-Driven: Triggered by sensors or mesh alerts.\n"
                    "• State Machine: Transitions based on current OS state.\n"
                    "• Try-Catch Flow: Error-resilient conditional paths.\n"
                    "• Rule Engine: Config-driven system policy logic."
                ),
                "Comparative & Range": (
                    "• Relational: Comparisons (<, >, ==, !=).\n"
                    "• Range Check: Verifying if values fall within bounds.\n"
                    "• Membership: Checking if a peer exists in the Mesh lattice."
                ),
                "Loops & Iterators": (
                    "• Repeat-X / While: Classic frequency-based execution.\n"
                    "• Do-While: Ensure action runs at least once.\n"
                    "• Foreach-Peer: Orchestrate across every node in the Aura Mesh."
                )
            },
            "Automation Studio (The Ecosystem)": {
                "1. Logic Builder": "Drag-and-drop Scratch blocks to design complex conditions (If/Else, Switch, Pattern Match).",
                "2. Routine Library": "Samsung-style device modes (Focus, Sleep, Travel) that orchestrate system settings instantly.",
                "3. Workflow Engine (Automa)": "Web and Desktop automation: Auto-fill forms, scrape research data, and sort local assets.",
                "4. Analytics Overlay": "Gamified productivity with XP, badges, and 'Time Saved' counters visualized via live charts.",
                "5. Sharing Panel": "Export and Import routines to share automation recipes with other SigmaOS users on the mesh."
            },
            "Aura Remote Hub (Universal Control)": {
                "IR Universal Remote": "Control TVs, ACs, and Projectors directly from SigmaOS. Mirrors Mi Remote functionality.",
                "Wi-Fi & IoT Control": "Seamlessly bridge SmartThings, Home Assistant, and Google Home into one sovereign interface.",
                "PC Remote Control": "Control other PCs (Mouse/Key/Media) with TeamViewer-grade security and PQC encryption.",
                "Macro Commander": "Run complex multi-device routines (e.g., 'Cinema Mode' dims lights and starts the projector).",
                "Gaming Link": "Optimized low-latency streaming for Steam, PlayStation, and Xbox consoles."
            },
            "Universal OS Bridge (Cross-Platform Parity)": {
                "Windows Parity": "Mirrors Power Automate (OmniAutomator), WSL (UAL Wasm), and Registry (Sovereign Ledger).",
                "macOS Parity": "Mirrors Spotlight (OmniSearch), Automator (Logic Builder), and Time Machine (Aura Snapshots).",
                "Linux Parity": "Complete Cron & Shell scripting via Terminal REPL and P2P Package Management (Marketplace).",
                "Mobile Parity": "Samsung-style Modes & Routines integrated with iOS-style Shortcuts for device-level orchestration.",
                "Enterprise Parity": "Industrial-grade Virtualization (UAL) and Distributed Task Scheduling across the mesh."
            },
            "SigmaBuyHatke (Price Intelligence)": {
                "Price History Trend": "View sovereign-tracked price fluctuations to identify the absolute lowest entry point.",
                "Auto-Coupon Discovery": "Automated scanning and verification of active promo codes across major e-commerce platforms.",
                "Multi-Store Comparison": "Real-time cost analysis across Amazon, Flipkart, Croma, and Reliance Digital.",
                "Smart Buy Verdict": "AI-driven analysis (EXCELLENT/WAIT/DECENT) based on historical average and lowest recorded price."
            },
            "SigmaWriteSense (Writing Intelligence)": {
                "Sovereign Grammar & Tone": "Grammarly-style real-time corrections and tone adjustments across all OS apps.",
                "Hemingway Readability Audit": "Analyze sentence complexity, passive voice, and adverb usage to simplify your prose.",
                "Semantic Paraphrasing": "QuillBot-style rephrasing (Formal/Simple/Creative) while maintaining original meaning.",
                "Deep Editorial Reports": "ProWritingAid-style deep dives into sentence variety, glue words, and repetitive patterns.",
                "Enterprise Brand Voice": "Ensure all communications align with your professional brand identity."
            },
            "SigmaFlowAI (Procedural Logic)": {
                "AI Flowchart Generation": "Convert raw text procedures into computer-logic-based flowcharts using Mermaid technology.",
                "Procedural Efficiency Audit": "Identify bottlenecks and optimization points in any legal or commercial workflow.",
                "Logic Mapping": "Transform complex hierarchical rules into flat, executable logic maps for systems analysis."
            },
            "Sigma AI Nexus (Intelligence Gateway)": {
                "Multi-Model Swapping": "One-click switching between exhaustive suite: ChatGPT, Gemini, Claude, Copilot, Grok, Perplexity, Krutrim, and local Sovereign.",
                "Global Auth Bridge": "USA & India optimized. Securely link your credentials to access free tiers of global (USD) and Indic (INR) models.",
                "Continental Consensus": "Simultaneously query American (USA) and Indic (India) models to synthesize a globally balanced 'Master Response'.",
                "Sovereign Core Hub": "Automatically reverts to Indian-optimized local offline intelligence if cloud connectivity is lost."
            },
            "Sigma Customization Studio (The Living Canvas)": {
                "AI Theme Engine": "Generative UI that creates color palettes and layouts based on your mood or context.",
                "Atomic Layout Mutation": "Instantly switch sidebar positions (Left/Right/Floating) and dashboard density without restart.",
                "Icon Pack Studio": "Swap OS-wide icon sets (3D, Material, Fluent, Retro) in real-time.",
                "OS Physicality": "Control system acoustics (Soundscapes) and animation physics (Bouncy, Elastic, Quartic curves)."
            },
            "Sigma Automation Workshop": {
                "Macro Recorder": "Record complex sequence of system actions and replay them as single-click macros.",
                "Global Task Scheduler": "Schedule any OS function, script, or AI prompt to run at specific intervals or delays.",
                "Zero-Trust Efficiency": "Automate workflows across Law, Writing, and Commerce hubs with full sovereign audit logs."
            },
            "Sovereign Apex (Multi-OS Fusion Hub)": {
                "Sigma Spotlight": "macOS/Alfred USP. Universal entry for files, AI prompts, and system commands.",
                "SnapGrid Layouts": "Windows 11 USP. Intelligent, grid-based window tiling and mosaic orchestration.",
                "TimeVault Snapshots": "macOS USP. Atomic, zero-copy system versioning and one-click rollback.",
                "Sigma Subsystem for Linux (SSL)": "Windows WSL USP. Run Linux, Wasm, and Docker binaries in native sandboxes.",
                "Universal Control Center": "iOS/Android/macOS USP. One-tap system toggles for security, AI, and energy states.",
                "Ecosystem Continuity": "Apple Handoff USP. Universal clipboard and cross-device session migration for phones and tablets.",
                "Identity Cloaking": "Proton/Privacy Linux USP. Burner identities, automated metadata purging, and total stealth network mode.",
                "Adaptive Context Engine": "Apple Intelligence USP. The OS anticipates your intent and auto-reconfigures tools based on the active mission.",
                "CoreBoost Performance": "Windows Game Mode/DirectStorage USP. Zero-jitter scheduling and GPU-priority fencing for extreme gaming and compute.",
                "Aura Projector": "AirPlay/Miracast USP. Low-latency, encrypted 8K wireless projection to any Sovereign node on the mesh.",
                "Sovereign Vault Plus": "1Password/Keychain USP. Quantum-secure, mesh-sharded identity and credential management with biometric-first access.",
                "Sigma Sentinel": "Screen Time/Digital Wellbeing USP. Real-time digital noise reduction (Deep Zen) and hardware health warden.",
                "Vision Forge": "Google Magic Editor/Visual Look Up USP. Real-time object identification, local generative media editing, and live captions.",
                "Aura Relay": "iMessage/FaceTime USP. Sovereign, serverless 8K video and text communication with zero-trace metadata.",
                "Neural Shell": "Warp/Termius USP. AI-native terminal with command shoring and session REWIND snapshots.",
                "Hardware Warden": "NVIDIA/Razer USP. Industrial-grade device manager for GPU overclocking, CPU undervolting, and driver sandboxing.",
                "Universal Translator Plus": "DeepL/Translate USP. Real-time, system-wide local translation for text, audio, and video streams.",
                "Sovereign Commerce": "Shopify/Amazon USP. An independent Shopping OS with native catalog, inventory, and logistics engines.",
                "Sigma Core Brain": "Meta-OS USP. Abstracted logic and service adapters that ensure the OS is independent of any specific vendor tool.",
                "Sigma Pulse": "Ambient Sentience USP. Ultra-low-power sentinel mode that keeps the kernel 'alive' at 1% CPU for mesh-sync and wake-word detection.",
                "Semantic Bus": "IPC 3.0 USP. Replacing traditional D-Bus with an AI-moderated intent relay where apps communicate via semantic meaning.",
                "Temporal Loop Computation": "Zero-Crash USP. A probabilistic core that rewinds time (crash logic) and simulates alternative execution paths to find stability.",
                "Entropic Entropy Shield": "Kinetic Security USP. A memory protection layer where sensitive data constantly shifts its address and keys at 10Hz to avoid static exploits.",
                "Sigma Vanguard Security": "McAfee/Defender/VirusTotal USP. A sovereign security suite with real-time heuristic scanning and mesh-based P2P threat intelligence."
            },
            "Advanced Tips": {
                "Spotlight Search": "Press Win+S (Alt+S) from anywhere to search missions, files, or modes (Windows Standard).",
                "Keyboard Shortcuts": "Full Windows parity: Ctrl+C (Copy), Ctrl+V (Paste), Ctrl+Z (Undo), Ctrl+S (Save). See docs/shortcuts.md.",
                "Sovereign Help": "Press F1 from any location to instantly launch this Sovereign Manual (📖).",
                "Stage Manager": "Cluster your active tasks into 'Workspaces', 'Dev Clusters', or 'Media Stacks' in the sidebar.",
                "Terminal": "For power users. Type 'help' to see the full list of Apex commands."
            }
        }

    def get_sections(self) -> List[str]:
        return list(self.MANUAL_DATA.keys())

    def get_content(self, section: str) -> Dict[str, str]:
        return self.MANUAL_DATA.get(section, {})

    def search(self, query: str) -> List[Dict[str, str]]:
        results = []
        query = query.lower()
        for section, topics in self.MANUAL_DATA.items():
            for topic, text in topics.items():
                if query in section.lower() or query in topic.lower() or query in text.lower():
                    results.append({"section": section, "topic": topic, "content": text})
        return results

    def health_check(self) -> str:
        return f"Manual Ready: {len(self.MANUAL_DATA)} sections, Sovereign-indexed."
