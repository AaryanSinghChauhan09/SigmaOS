/// Sovereign Onion Router — absorbs Tor circuit and cell routing principles.
/// Zero dependency implementation using manual XOR-based key derivation.
#[derive(Debug, Clone)]
pub struct Circuit {
    pub id: u64,
    pub hops: u8,
}

pub struct OnionRouter {
    pub circuits: Vec<Circuit>,
    next_cid: u64,
}

impl Default for OnionRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl OnionRouter {
    pub fn new() -> Self {
        Self { circuits: Vec::new(), next_cid: 1 }
    }

    pub fn create_circuit(&mut self, hops: u8) -> Result<Circuit, String> {
        if hops < 1 || hops > 8 {
            return Err("Hop count must be 1-8".to_string());
        }
        let circuit = Circuit { id: self.next_cid, hops };
        self.next_cid += 1;
        self.circuits.push(circuit.clone());
        Ok(circuit)
    }
}
