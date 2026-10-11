// SigmaOS ALSA Use Case Manager (UCM) Parser & Sequencer
// Manages device routing profiles, volume state transitions, and audio mixer control sequences.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Standard ALSA Mixer Control Command
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UcmControlCommand {
    CSet { control_name: String, value: String },
    Sleep { delay_ms: u32 },
}

/// A specific audio device profile (e.g. "Speaker", "Headphones")
#[derive(Debug, Clone)]
pub struct UcmDeviceProfile {
    pub name: String,
    pub comment: String,
    pub enable_sequence: Vec<UcmControlCommand>,
    pub disable_sequence: Vec<UcmControlCommand>,
}

/// A UCM Verb (e.g. "HiFi", "VoiceCall")
#[derive(Debug, Clone)]
pub struct UcmVerb {
    pub name: String,
    pub comment: String,
    pub devices: BTreeMap<String, UcmDeviceProfile>,
    pub enable_sequence: Vec<UcmControlCommand>,
}

/// ALSA Use Case Manager Card Configuration
#[derive(Debug, Clone)]
pub struct AlsaUcmManager {
    pub card_name: String,
    pub verbs: BTreeMap<String, UcmVerb>,
    pub active_verb: Option<String>,
    pub active_devices: Vec<String>,
    pub applied_mixer_controls: BTreeMap<String, String>,
}

impl AlsaUcmManager {
    pub fn new(card_name: &str) -> Self {
        Self {
            card_name: String::from(card_name),
            verbs: BTreeMap::new(),
            active_verb: None,
            active_devices: Vec::new(),
            applied_mixer_controls: BTreeMap::new(),
        }
    }

    /// Register a verb with its associated device profiles
    pub fn register_verb(&mut self, verb: UcmVerb) {
        self.verbs.insert(verb.name.clone(), verb);
    }

    /// Switch active audio verb (e.g. "HiFi")
    pub fn set_verb(&mut self, verb_name: &str) -> Result<(), &'static str> {
        let verb = self.verbs.get(verb_name).ok_or("Verb not found in UCM profile")?;
        for cmd in &verb.enable_sequence {
            if let UcmControlCommand::CSet { control_name, value } = cmd {
                self.applied_mixer_controls.insert(control_name.clone(), value.clone());
            }
        }
        self.active_verb = Some(String::from(verb_name));
        self.active_devices.clear();
        Ok(())
    }

    /// Enable an audio device within the current verb
    pub fn enable_device(&mut self, device_name: &str) -> Result<(), &'static str> {
        let active_verb_name = self.active_verb.as_ref().ok_or("No active verb set")?;
        let verb = self.verbs.get(active_verb_name).ok_or("Verb missing")?;
        let dev = verb.devices.get(device_name).ok_or("Device profile not found")?;

        for cmd in &dev.enable_sequence {
            if let UcmControlCommand::CSet { control_name, value } = cmd {
                self.applied_mixer_controls.insert(control_name.clone(), value.clone());
            }
        }

        if !self.active_devices.iter().any(|d| d == device_name) {
            self.active_devices.push(String::from(device_name));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alsa_ucm_profile_transitions() {
        let mut ucm = AlsaUcmManager::new("HDA-Intel");

        let mut hifi_devices = BTreeMap::new();
        hifi_devices.insert(
            String::from("Speaker"),
            UcmDeviceProfile {
                name: String::from("Speaker"),
                comment: String::from("Internal Stereo Speakers"),
                enable_sequence: alloc::vec![UcmControlCommand::CSet {
                    control_name: String::from("Speaker Switch"),
                    value: String::from("on"),
                }],
                disable_sequence: Vec::new(),
            },
        );

        let hifi_verb = UcmVerb {
            name: String::from("HiFi"),
            comment: String::from("High Fidelity Playback"),
            devices: hifi_devices,
            enable_sequence: alloc::vec![UcmControlCommand::CSet {
                control_name: String::from("Master Playback Volume"),
                value: String::from("85%"),
            }],
        };

        ucm.register_verb(hifi_verb);

        assert!(ucm.set_verb("HiFi").is_ok());
        assert_eq!(
            ucm.applied_mixer_controls.get("Master Playback Volume"),
            Some(&String::from("85%"))
        );

        assert!(ucm.enable_device("Speaker").is_ok());
        assert_eq!(
            ucm.applied_mixer_controls.get("Speaker Switch"),
            Some(&String::from("on"))
        );
    }
}
