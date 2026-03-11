"""
SigmaOS Accessibility Hub
============================
USP: Modular accessibility that treats every user need as a first-class feature.

Competition comparison:
  Windows  → Ease of Access center (often clumsy UI, limited native integration).
  macOS    → Superb native integration (VoiceOver, Zoom), heavily tied to proprietary ecosystem.
  Linux    → Very fragmented (Orca is standard, but doesn't work well across all toolkits).
  SigmaOS  → OmniAccess: Screen reading, UI magnification, color filtering,
             and neuro-divergence modes integrated directly into the window manager.

Core innovations:
  1. AI-Powered Screen Describer — Not just reading text, but interpreting visual context.
  2. NeuroDivergent Profiles     — Focus modes that eliminate animations, mute chaotic colors, enhance text.
  3. OmniVoice Control           — Sovereign offline voice control of the entire OS.
  4. High-Contrast Projection    — Renders any foreign app (Win/Mac) in Sigma's high-contrast theme.
"""
from dataclasses import dataclass
from enum import Enum, auto


class AccessMode(Enum):
    VISUAL       = "Visual Assistance"
    AUDITORY     = "Auditory Assistance"
    MOTOR        = "Motor Control"
    NEURO_FOCUS  = "Neuro-Divergent Focus"

class InputMode(Enum):
    STANDARD = auto()
    VOICE_NATIVE = auto()
    GESTURE_HAPTIC = auto()
    EYE_TRACKING = auto()


import threading
try:
    import pyttsx3
    SPEECH_AVAILABLE = True
except ImportError:
    SPEECH_AVAILABLE = False


class SigmaAccessibilityHub:
    """Centralized Accessibility Control for SigmaOS."""

    def __init__(self):
        self._active_features: dict[str, bool] = {
            "screen_reader": False,
            "ai_describer": False,
            "high_contrast": False,
            "color_blind_filter": False,
            "magnifier": False,
            "voice_control": False,
            "neuro_focus": False,
        }
        self._stats = {"sessions_assisted": 0}
        self._speech_engine = None
        self._speech_lock = threading.Lock()
        
        if SPEECH_AVAILABLE:
            try:
                temp_engine = pyttsx3.init()
                temp_engine.setProperty('rate', 150)
                temp_engine.setProperty('volume', 0.9)
                self._speech_engine = temp_engine
            except Exception:
                # Use a local flag or just catch silently
                pass
                
        self._current_input_mode = InputMode.STANDARD
        
        self.ACCESSIBILITY_PROFILES = {
            "low_vision_mode": {
                "name": "Low Vision Mode",
                "features": {"magnifier": True, "high_contrast": True, "screen_reader": True, "ai_describer": True},
                "input_mode": InputMode.VOICE_NATIVE
            },
            "sensory_focus": {
                "name": "Sensory Focus Mode",
                "features": {"neuro_focus": True, "color_blind_filter": False, "magnifier": False},
                "input_mode": InputMode.STANDARD
            },
            "motor_assistance": {
                "name": "Motor Assistance Mode",
                "features": {"voice_control": True, "ai_describer": False},
                "input_mode": InputMode.EYE_TRACKING
            }
        }

    def speak(self, text: str, interrupt: bool = True):
        """USP: Sovereign TTS - Reads text aloud for visually impaired users."""
        if not SPEECH_AVAILABLE or not self._active_features["screen_reader"]:
            return False
            
        def _say():
            with self._speech_lock:
                try:
                    engine = pyttsx3.init()
                    engine.setProperty('rate', 150)
                    engine.say(text)
                    engine.runAndWait()
                    engine.stop() # Clean up
                except Exception:
                    pass
        
        threading.Thread(target=_say, daemon=True).start()
        return True

    def toggle_feature(self, feature: str, state: bool | None = None) -> dict:
        """Enable or disable a specific accessibility feature."""
        if feature not in self._active_features:
            return {"error": f"Unknown accessibility feature '{feature}'."}
            
        new_state = state if state is not None else not self._active_features[feature]
        self._active_features[feature] = new_state
        self._stats["sessions_assisted"] += 1
        
        status = "ENABLED" if new_state else "DISABLED"
        
        # Simulated side-effects
        if feature == "neuro_focus" and new_state:
            effect_msg = "Animations disabled. Notification sounds muted. High-legibility font activated."
        elif feature == "ai_describer" and new_state:
            effect_msg = "Sovereign AI Vision model loaded to VRAM for real-time screen context."
        elif feature == "high_contrast" and new_state:
            effect_msg = "Forced UI projection to AMOLED Dark + High Contrast styling."
        elif feature == "screen_reader" and new_state:
            effect_msg = "Sovereign Screen Reader active. Ready to announce UI events."
            self.speak("Screen reader enabled. Welcome to SigmaOS Sovereign.")
        else:
            effect_msg = "Feature toggled at the window-manager level."
            
        return {
            "feature": feature,
            "state": status,
            "message": f"OmniAccess: {feature.upper()} is now {status}. {effect_msg}"
        }

    def switch_input_mode(self, new_mode: str) -> dict[str, str]:
        """USP: Adaptive Input Modes - Seamlesly transition between gesture, voice, and eye tracking."""
        try:
            mode_enum = getattr(InputMode, new_mode.upper())
            self._current_input_mode = mode_enum
            # Signal HAL to load required drivers
            return {"status": "SUCCESS", "message": f"Adaptive Input switched to: {mode_enum.name}"}
        except (KeyError, AttributeError):
            return {"error": "Invalid Input Mode requested."}

    def apply_profile(self, profile_key: str) -> dict[str, str]:
        """USP: Personalized Accessibility Profiles. Loads an entire user setup instantly."""
        prof: dict = self.ACCESSIBILITY_PROFILES.get(profile_key, {})
        if not prof:
            return {"error": "Profile not found."}
            
        messages = []
        raw_feat = prof.get("features", {})
        if not isinstance(raw_feat, dict): raw_feat = {}
        for feat in list(raw_feat.keys()):
            state = raw_feat[feat]
            res = self.toggle_feature(str(feat), bool(state))
            if "message" in res: messages.append(res["message"])
            
        i_mode_val = prof.get("input_mode", InputMode.STANDARD)
        i_mode = i_mode_val if isinstance(i_mode_val, InputMode) else InputMode.STANDARD
        self._current_input_mode = i_mode
        messages.append(f"Input Mode forced to: {i_mode.name}")
        
        # Inclusive Gamification
        self._stats["sessions_assisted"] += 5  # Give a boost for using a full profile
        
        return {"status": "PROFILE APPLIED", "summary": " | ".join(messages)}

    def list_active(self) -> list[str]:
        return [f for f, active in self._active_features.items() if active]

    def describe_screen(self, ui_context: str) -> dict:
        """Simulates the AI-Powered Screen Describer processing a GUI state."""
        if not self._active_features["ai_describer"]:
            return {"error": "AI Describer is not enabled."}
            
        description = f"The screen currently shows a {ui_context}."
        if "Settings" in ui_context:
            description += " There are 4 toggles available, focusing on Networking."
        
        self.speak(description)
        return {
            "input": ui_context,
            "spoken_text": description,
            "message": f"AI Describer: '{description}' (Synthesized via local neural TTS)"
        }

    def process_voice_command(self, transcript: str) -> dict:
        """Simulates OmniVoice offline action processing."""
        if not self._active_features.get("voice_control", False):
            return {"error": "OmniVoice is offline. Please enable the feature."}
            
        cmd = transcript.lower()
        if "open" in cmd and "browser" in cmd:
            action = "Launched OmniBrowser"
        elif "close" in cmd:
            action = "Closed active window"
        elif "read" in cmd:
            action = "Triggered Screen Reader on active paragraph"
        else:
            action = "Command not recognized"
            
        return {
            "transcript": transcript,
            "action": action,
            "message": f"OmniVoice: Processed offline command -> '{action}'"
        }

    def health_check(self) -> str:
        s = self._active_features
        active = sum(1 for v in s.values() if v)
        return f"OK — Features active: {active}/7. Assisted interactions: {self._stats['sessions_assisted']}."


if __name__ == "__main__":
    hub = SigmaAccessibilityHub()
    print(hub.toggle_feature("ai_describer", True)["message"])
    print(hub.describe_screen("Settings Panel overlay")["message"])
    print(hub.toggle_feature("voice_control", True)["message"])
    print(hub.process_voice_command("Open the Browser")["message"])
    print(hub.toggle_feature("neuro_focus", True)["message"])
    print("Currently active:", hub.list_active())
