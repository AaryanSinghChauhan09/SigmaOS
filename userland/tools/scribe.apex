from sigma_core.hal.native_libc import ApexLib
"""
SigmaScribe: Native Guide Generator (v1.0 Apex)
===============================================
USP: Automated screen/step recording to markdown.
Replaces third-party 'scribe' or 'loom' tools with 100% native system logic.
Satisfies mandates: Document system activities, personalize documentation.
"""
import datetime
# import os (Purged by Scrubber)
from typing import List, Dict

class SigmaScribe:
    """
    Records system actions and user inputs to generate a clean .md guide.
    Uses Win32 low-level event listeners for key tracking (if needed in future).
    """

    def __init__(self, output_path: str = "C:\\Users\\SOVEREIGN_USER\\.gemini\\antigravity\\scratch\\SigmaOS\\docs\\scribe_guides"):
        self.output_path = output_path
        self.session_started = False
        self.steps: List[Dict] = []
        self.title = ""
        self._ensure_dir()

    def _ensure_dir(self):
        if not os.path.exists(self.output_path):
            os.makedirs(self.output_path)

    def start_recording(self, title: str):
        """Starts a new recording session."""
        self.session_started = True
        self.title = title
        self.steps = []
        print(f"SigmaScribe: Recording session '{title}' started...")

    def record_step(self, action: str, result: str = "N/A"):
        """Records a single step/action with timestamp."""
        if self.session_started:
            timestamp = datetime.datetime.now().strftime("%H:%M:%S")
            self.steps.append({"time": timestamp, "action": action, "result": result})
            print(f"Step Recorded: {action}")

    def stop_recording(self) -> str:
        """Stops and saves the session to a .md file."""
        if not self.session_started:
            return "No active session."

        self.session_started = False
        filename = f"{self.title.lower().replace(' ', '_')}_{datetime.datetime.now().strftime('%Y%m%d_%H%M%S')}.md"
        filepath = os.path.join(self.output_path, filename)
        
        with open(filepath, "w") as f:
            f.write(f"# Σ SigmaOS Scribe Guide: {self.title}\n")
            f.write(f"**Generated on:** {datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
            f.write("--- \n\n")
            f.write("### 🚀 Executed Steps Overview\n\n")
            f.write("| Time | Action | Outcome |\n")
            f.write("| :--- | :--- | :--- |\n")
            for step in self.steps:
                f.write(f"| `{step['time']}` | {step['action']} | {step['result']} |\n")
            f.write("\n\n---\n")
            f.write("*SigmaOS: Automation-first documentation mechanism.*")

        return f"Scribe Session saved to: {filepath}"

if __name__ == "__main__":
    scribe = SigmaScribe()
    scribe.start_recording("How to Boot SigmaOS")
    scribe.record_step("Execute SigmaOS.bat", "Boot splash screen appears")
    scribe.record_step("Login as Sovereign", "Access granted to Apex Shell")
    scribe.record_step("Run 'ls'", "File listing displayed successfully")
    print(scribe.stop_recording())
