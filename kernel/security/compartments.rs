//! SigmaOS Compartmentalization Module (QubesOS/AppArmor inspired)
//! Phase 3: Security & Sandboxing Prototypes

#[allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;

/// A strict execution compartment, analogous to a QubesOS "Qube" or a lightweight VM sandbox.
/// SigmaOS compartments rely on Ring 1/Ring 3 paging separation + Capability tokens.
pub struct Compartment {
    pub id: SigmaU32,
    pub name: &'static str,
    /// Memory boundaries for this compartment
    pub base_addr: SigmaU64,
    pub limit_addr: SigmaU64,
    /// Whether the compartment has network access
    pub network_isolated: bool,
    /// Whether the compartment has filesystem access (beyond its own virtual root)
    pub storage_isolated: bool,
    /// Cryptographic capability token tied to this compartment
    pub capability_token: SigmaU64,
}

impl Compartment {
    /// Create a new strict compartment with maximum isolation
    pub const fn new_strict(id: SigmaU32, name: &'static str, base_addr: SigmaU64, limit_addr: SigmaU64) -> Self {
        Self {
            id,
            name,
            base_addr,
            limit_addr,
            network_isolated: true,
            storage_isolated: true,
            capability_token: 0, // Uninitialized token
        }
    }

    /// Grant a specific capability to this compartment
    pub fn grant_capability(&mut self, token: SigmaU64) {
        self.capability_token = token;
    }

    /// Validate if an address is within the compartment's strict memory sandbox
    pub fn validate_memory_access(&self, addr: SigmaU64) -> bool {
        addr >= self.base_addr && addr < self.limit_addr
    }
}

// Global registry of active compartments (static for no_alloc)
const MAX_COMPARTMENTS: usize = 32;
static mut COMPARTMENTS: [Compartment; MAX_COMPARTMENTS] = [
    Compartment::new_strict(0, "UNINIT", 0, 0); MAX_COMPARTMENTS
];
static mut COMPARTMENT_COUNT: usize = 0;

/// Register a new secure compartment
pub unsafe fn register_compartment(compartment: Compartment) -> Result<(), &'static str> {
    if COMPARTMENT_COUNT >= MAX_COMPARTMENTS {
        return Err("Compartment limit reached");
    }
    COMPARTMENTS[COMPARTMENT_COUNT] = compartment;
    COMPARTMENT_COUNT += 1;
    Ok(())
}
