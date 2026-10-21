// Sovereign Network Manager v3

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Loopback,
    Bridge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
    Configuring,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: InterfaceType,
    pub state: InterfaceState,
}

#[derive(Debug, Default)]
pub struct SigmaNetworkManager3 {
    pub interfaces: Vec<NetworkInterface>,
}

impl SigmaNetworkManager3 {
    pub fn new() -> Self {
        Self::default()
    }
}
