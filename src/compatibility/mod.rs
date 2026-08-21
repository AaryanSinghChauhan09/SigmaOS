// Compatibility Layer for Foreign Subsystems
// Provides translation layers, ABI adapters, and distro parity engines

pub mod abi_extended;
pub mod abi_translator;
pub mod fedora_domination;

pub use abi_extended::*;
pub use fedora_domination::*;
