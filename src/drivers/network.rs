// SigmaOS Network Driver
// Hardware abstraction for network interfaces

use crate::security::CapabilityToken;

/// Network interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    Ethernet,
    WiFi,
    Virtual,
}

/// Network command
#[derive(Debug, Clone)]
pub enum NetworkCommand {
    SendPacket { data: Vec<u8> },
    ReceivePacket,
    GetMACAddress,
    SetIP { ip: String },
    Connect { ssid: String, password: String },
}

/// Network driver interface
pub struct NetworkDriver {
    pub interface_type: NetworkType,
    pub mac_address: [u8; 6],
    pub ip_address: String,
    pub capabilities: CapabilityToken,
    pub connected: bool,
}

impl NetworkDriver {
    pub fn new(interface_type: NetworkType) -> Self {
        Self {
            interface_type,
            mac_address: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ip_address: "0.0.0.0".to_string(),
            capabilities: CapabilityToken::new(),
            connected: false,
        }
    }

    pub fn execute_command(&mut self, command: NetworkCommand) -> Result<Vec<u8>, NetworkError> {
        match command {
            NetworkCommand::SendPacket { data: _ } => {
                if !self.connected {
                    return Err(NetworkError::NotConnected);
                }
                // Simulate packet transmission
                Ok(vec![])
            }
            NetworkCommand::ReceivePacket => {
                if !self.connected {
                    return Err(NetworkError::NotConnected);
                }
                // Simulate packet reception
                Ok(vec![])
            }
            NetworkCommand::GetMACAddress => Ok(self.mac_address.to_vec()),
            NetworkCommand::SetIP { ip } => {
                self.ip_address = ip;
                Ok(vec![])
            }
            NetworkCommand::Connect {
                ssid: _,
                password: _,
            } => {
                if self.interface_type != NetworkType::WiFi {
                    return Err(NetworkError::InvalidOperation);
                }
                // Simulate WiFi connection
                self.connected = true;
                Ok(vec![])
            }
        }
    }

    pub fn set_mac_address(&mut self, mac: [u8; 6]) {
        self.mac_address = mac;
    }

    pub fn set_capabilities(&mut self, capabilities: CapabilityToken) {
        self.capabilities = capabilities;
    }

    pub fn has_capability(&self, capability: u64) -> bool {
        (self.capabilities.bits() & capability) != 0
    }
}

impl Default for NetworkDriver {
    fn default() -> Self {
        Self::new(NetworkType::Ethernet)
    }
}

/// Network errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    NotConnected,
    InvalidOperation,
    PermissionDenied,
    TransmissionFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let network = NetworkDriver::new(NetworkType::Ethernet);
        assert_eq!(network.interface_type, NetworkType::Ethernet);
        assert!(!network.connected);
    }

    #[test]
    fn test_get_mac_address() {
        let mut network = NetworkDriver::new(NetworkType::Ethernet);
        network.set_mac_address([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        let command = NetworkCommand::GetMACAddress;
        let result = network.execute_command(command).unwrap();
        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn test_set_ip() {
        let mut network = NetworkDriver::new(NetworkType::Ethernet);
        let command = NetworkCommand::SetIP {
            ip: "192.168.1.1".to_string(),
        };
        assert!(network.execute_command(command).is_ok());
        assert_eq!(network.ip_address, "192.168.1.1");
    }

    #[test]
    fn test_not_connected_error() {
        let mut network = NetworkDriver::new(NetworkType::Ethernet);
        let command = NetworkCommand::SendPacket {
            data: vec![1, 2, 3],
        };
        assert!(network.execute_command(command).is_err());
    }
}
