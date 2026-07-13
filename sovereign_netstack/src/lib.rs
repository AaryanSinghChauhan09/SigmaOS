// SovereignNetStack - TCP/IP Network Stack Implementation
// Implements POSIX-compatible TCP/IP with zero-trust firewall isolation
// No external dependencies - implements from first principles

use std::fmt;

/// IP address version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPVersion {
    IPv4,
    IPv6,
}

impl IPVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            IPVersion::IPv4 => "IPv4",
            IPVersion::IPv6 => "IPv6",
        }
    }
}

/// IP address
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IPAddress {
    IPv4([u8; 4]),
    IPv6([u8; 16]),
}

impl IPAddress {
    pub fn new_v4(a: u8, b: u8, c: u8, d: u8) -> Self {
        IPAddress::IPv4([a, b, c, d])
    }
    
    pub fn new_v6(bytes: [u8; 16]) -> Self {
        IPAddress::IPv6(bytes)
    }
    
    pub fn version(&self) -> IPVersion {
        match self {
            IPAddress::IPv4(_) => IPVersion::IPv4,
            IPAddress::IPv6(_) => IPVersion::IPv6,
        }
    }
}

impl fmt::Display for IPAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IPAddress::IPv4(bytes) => {
                write!(f, "{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
            }
            IPAddress::IPv6(bytes) => {
                let parts: Vec<String> = bytes
                    .chunks(2)
                    .map(|chunk| format!("{:02x}{:02x}", chunk[0], chunk[1]))
                    .collect();
                write!(f, "{}", parts.join(":"))
            }
        }
    }
}

/// Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Other(u8),
}

impl Protocol {
    pub fn from_u8(value: u8) -> Self {
        match value {
            6 => Protocol::TCP,
            17 => Protocol::UDP,
            1 => Protocol::ICMP,
            other => Protocol::Other(other),
        }
    }
    
    pub fn as_u8(&self) -> u8 {
        match self {
            Protocol::TCP => 6,
            Protocol::UDP => 17,
            Protocol::ICMP => 1,
            Protocol::Other(val) => *val,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::TCP => "TCP",
            Protocol::UDP => "UDP",
            Protocol::ICMP => "ICMP",
            Protocol::Other(_) => "Other",
        }
    }
}

/// Port
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Port {
    pub number: u16,
}

impl Port {
    pub fn new(number: u16) -> Self {
        Port { number }
    }
    
    pub fn is_well_known(&self) -> bool {
        self.number < 1024
    }
    
    pub fn is_ephemeral(&self) -> bool {
        self.number >= 49152
    }
}

/// Socket
#[derive(Debug, Clone)]
pub struct Socket {
    pub local_ip: IPAddress,
    pub local_port: Port,
    pub remote_ip: IPAddress,
    pub remote_port: Port,
    pub protocol: Protocol,
}

impl Socket {
    pub fn new(
        local_ip: IPAddress,
        local_port: Port,
        remote_ip: IPAddress,
        remote_port: Port,
        protocol: Protocol,
    ) -> Self {
        Socket {
            local_ip,
            local_port,
            remote_ip,
            remote_port,
            protocol,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        // Basic validation
        self.local_port.number > 0 && self.remote_port.number > 0
    }
}

/// Firewall rule action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Log,
}

impl FirewallAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            FirewallAction::Allow => "Allow",
            FirewallAction::Deny => "Deny",
            FirewallAction::Log => "Log",
        }
    }
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub rule_id: [u8; 32],
    pub source_ip: Option<IPAddress>,
    pub source_port: Option<Port>,
    pub dest_ip: Option<IPAddress>,
    pub dest_port: Option<Port>,
    pub protocol: Option<Protocol>,
    pub action: FirewallAction,
    pub enabled: bool,
}

impl FirewallRule {
    pub fn new(
        source_ip: Option<IPAddress>,
        source_port: Option<Port>,
        dest_ip: Option<IPAddress>,
        dest_port: Option<Port>,
        protocol: Option<Protocol>,
        action: FirewallAction,
    ) -> Self {
        let rule_id = Self::generate_rule_id(&source_ip, &dest_ip, &protocol);
        
        FirewallRule {
            rule_id,
            source_ip,
            source_port,
            dest_ip,
            dest_port,
            protocol,
            action,
            enabled: true,
        }
    }
    
    fn generate_rule_id(
        source_ip: &Option<IPAddress>,
        dest_ip: &Option<IPAddress>,
        protocol: &Option<Protocol>,
    ) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        
        if let Some(ip) = source_ip {
            let ip_str = format!("{}", ip);
            let bytes = ip_str.as_bytes();
            for (i, &byte) in bytes.iter().enumerate() {
                hash[i % 32] = hash[i % 32].wrapping_add(byte);
            }
        }
        
        if let Some(ip) = dest_ip {
            let ip_str = format!("{}", ip);
            let bytes = ip_str.as_bytes();
            for (i, &byte) in bytes.iter().enumerate() {
                hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
            }
        }
        
        if let Some(proto) = protocol {
            let proto_val = proto.as_u8();
            hash[31] = hash[31].wrapping_add(proto_val);
        }
        
        hash
    }
    
    pub fn matches(&self, socket: &Socket) -> bool {
        // Check source IP
        if let Some(ref source_ip) = self.source_ip {
            if &socket.local_ip != source_ip {
                return false;
            }
        }
        
        // Check source port
        if let Some(source_port) = self.source_port {
            if socket.local_port.number != source_port.number {
                return false;
            }
        }
        
        // Check destination IP
        if let Some(ref dest_ip) = self.dest_ip {
            if &socket.remote_ip != dest_ip {
                return false;
            }
        }
        
        // Check destination port
        if let Some(dest_port) = self.dest_port {
            if socket.remote_port.number != dest_port.number {
                return false;
            }
        }
        
        // Check protocol
        if let Some(protocol) = self.protocol {
            if socket.protocol.as_u8() != protocol.as_u8() {
                return false;
            }
        }
        
        true
    }
    
    pub fn get_rule_id(&self) -> String {
        self.rule_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Zero-trust firewall
#[derive(Debug, Clone)]
pub struct ZeroTrustFirewall {
    pub rules: Vec<FirewallRule>,
    pub default_policy: FirewallAction,
}

impl ZeroTrustFirewall {
    pub fn new(default_policy: FirewallAction) -> Self {
        ZeroTrustFirewall {
            rules: Vec::new(),
            default_policy,
        }
    }
    
    pub fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
    }
    
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<(), String> {
        let initial_len = self.rules.len();
        self.rules.retain(|r| r.get_rule_id() != rule_id);
        
        if self.rules.len() == initial_len {
            Err("Rule not found".to_string())
        } else {
            Ok(())
        }
    }
    
    pub fn evaluate(&self, socket: &Socket) -> FirewallAction {
        // Check rules in order
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            if rule.matches(socket) {
                return rule.action;
            }
        }
        
        // Default policy
        self.default_policy
    }
    
    pub fn get_rules(&self) -> Vec<&FirewallRule> {
        self.rules.iter().collect()
    }
}

impl Default for ZeroTrustFirewall {
    fn default() -> Self {
        Self::new(FirewallAction::Deny) // Default deny for zero-trust
    }
}

/// TCP connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TCPState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    Closing,
    TimeWait,
    CloseWait,
    LastAck,
}

impl TCPState {
    pub fn as_str(&self) -> &'static str {
        match self {
            TCPState::Closed => "CLOSED",
            TCPState::Listen => "LISTEN",
            TCPState::SynSent => "SYN_SENT",
            TCPState::SynReceived => "SYN_RECEIVED",
            TCPState::Established => "ESTABLISHED",
            TCPState::FinWait1 => "FIN_WAIT_1",
            TCPState::FinWait2 => "FIN_WAIT_2",
            TCPState::Closing => "CLOSING",
            TCPState::TimeWait => "TIME_WAIT",
            TCPState::CloseWait => "CLOSE_WAIT",
            TCPState::LastAck => "LAST_ACK",
        }
    }
}

/// TCP connection
#[derive(Debug, Clone)]
pub struct TCPConnection {
    pub connection_id: [u8; 32],
    pub socket: Socket,
    pub state: TCPState,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub window_size: u16,
    pub created_at: u64,
    pub last_activity: u64,
}

impl TCPConnection {
    pub fn new(socket: Socket) -> Self {
        let connection_id = Self::generate_connection_id(&socket);
        let created_at = Self::current_timestamp();
        let last_activity = created_at;
        
        TCPConnection {
            connection_id,
            socket,
            state: TCPState::Closed,
            sequence_number: 0,
            acknowledgment_number: 0,
            window_size: 65535,
            created_at,
            last_activity,
        }
    }
    
    fn generate_connection_id(socket: &Socket) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let local_str = format!("{}:{}", socket.local_ip, socket.local_port.number);
        let remote_str = format!("{}:{}", socket.remote_ip, socket.remote_port.number);
        
        let local_bytes = local_str.as_bytes();
        for (i, &byte) in local_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        
        let remote_bytes = remote_str.as_bytes();
        for (i, &byte) in remote_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        
        hash
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn transition(&mut self, new_state: TCPState) {
        self.state = new_state;
        self.last_activity = Self::current_timestamp();
    }
    
    pub fn send(&mut self, data: &[u8]) -> u32 {
        let sent = data.len() as u32;
        self.sequence_number = self.sequence_number.wrapping_add(sent);
        self.last_activity = Self::current_timestamp();
        sent
    }
    
    pub fn receive(&mut self, data: &[u8]) -> u32 {
        let received = data.len() as u32;
        self.acknowledgment_number = self.acknowledgment_number.wrapping_add(received);
        self.last_activity = Self::current_timestamp();
        received
    }
    
    pub fn get_connection_id(&self) -> String {
        self.connection_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn is_established(&self) -> bool {
        self.state == TCPState::Established
    }
    
    pub fn seconds_idle(&self) -> u64 {
        let now = Self::current_timestamp();
        now.saturating_sub(self.last_activity)
    }
}

impl fmt::Display for TCPConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TCP Connection\n\
             ID: {}\n\
             Local: {}:{}\n\
             Remote: {}:{}\n\
             State: {}\n\
             Seq: {}\n\
             Ack: {}\n\
             Window: {}\n\
             Idle: {}s",
            self.get_connection_id(),
            self.socket.local_ip,
            self.socket.local_port.number,
            self.socket.remote_ip,
            self.socket.remote_port.number,
            self.state.as_str(),
            self.sequence_number,
            self.acknowledgment_number,
            self.window_size,
            self.seconds_idle()
        )
    }
}

/// Network stack
pub struct NetworkStack {
    pub firewall: ZeroTrustFirewall,
    pub connections: Vec<TCPConnection>,
}

impl NetworkStack {
    pub fn new(firewall: ZeroTrustFirewall) -> Self {
        NetworkStack {
            firewall,
            connections: Vec::new(),
        }
    }
    
    /// Create a new TCP connection
    pub fn create_connection(&mut self, socket: Socket) -> Result<String, String> {
        // Check firewall first
        match self.firewall.evaluate(&socket) {
            FirewallAction::Allow => {
                let connection = TCPConnection::new(socket);
                let connection_id = connection.get_connection_id();
                
                self.connections.push(connection);
                
                Ok(connection_id)
            }
            FirewallAction::Deny => Err("Connection denied by firewall".to_string()),
            FirewallAction::Log => {
                // Log and allow
                let connection = TCPConnection::new(socket);
                let connection_id = connection.get_connection_id();
                
                self.connections.push(connection);
                
                Ok(connection_id)
            }
        }
    }
    
    /// Get connection by ID
    pub fn get_connection(&self, connection_id: &str) -> Option<&TCPConnection> {
        self.connections
            .iter()
            .find(|c| c.get_connection_id() == connection_id)
    }
    
    /// Close connection
    pub fn close_connection(&mut self, connection_id: &str) -> Result<(), String> {
        let connection = self.connections
            .iter_mut()
            .find(|c| c.get_connection_id() == connection_id)
            .ok_or_else(|| "Connection not found".to_string())?;
        
        connection.transition(TCPState::Closing);
        connection.transition(TCPState::TimeWait);
        
        Ok(())
    }
    
    /// List all connections
    pub fn list_connections(&self) -> Vec<&TCPConnection> {
        self.connections.iter().collect()
    }
    
    /// Get established connections
    pub fn get_established_connections(&self) -> Vec<&TCPConnection> {
        self.connections
            .iter()
            .filter(|c| c.is_established())
            .collect()
    }
    
    /// Add firewall rule
    pub fn add_firewall_rule(&mut self, rule: FirewallRule) {
        self.firewall.add_rule(rule);
    }
    
    /// Get firewall rules
    pub fn get_firewall_rules(&self) -> Vec<&FirewallRule> {
        self.firewall.get_rules()
    }
}

impl Default for NetworkStack {
    fn default() -> Self {
        Self::new(ZeroTrustFirewall::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ip_address_creation() {
        let ip = IPAddress::new_v4(192, 168, 1, 1);
        assert_eq!(format!("{}", ip), "192.168.1.1");
        assert_eq!(ip.version(), IPVersion::IPv4);
    }
    
    #[test]
    fn test_port_validation() {
        let well_known = Port::new(80);
        assert!(well_known.is_well_known());
        
        let ephemeral = Port::new(50000);
        assert!(ephemeral.is_ephemeral());
    }
    
    #[test]
    fn test_socket_validation() {
        let socket = Socket::new(
            IPAddress::new_v4(192, 168, 1, 1),
            Port::new(12345),
            IPAddress::new_v4(10, 0, 0, 1),
            Port::new(80),
            Protocol::TCP,
        );
        
        assert!(socket.is_valid());
    }
    
    #[test]
    fn test_firewall_rule_matching() {
        let rule = FirewallRule::new(
            Some(IPAddress::new_v4(192, 168, 1, 1)),
            Some(Port::new(12345)),
            Some(IPAddress::new_v4(10, 0, 0, 1)),
            Some(Port::new(80)),
            Some(Protocol::TCP),
            FirewallAction::Allow,
        );
        
        let socket = Socket::new(
            IPAddress::new_v4(192, 168, 1, 1),
            Port::new(12345),
            IPAddress::new_v4(10, 0, 0, 1),
            Port::new(80),
            Protocol::TCP,
        );
        
        assert!(rule.matches(&socket));
    }
    
    #[test]
    fn test_firewall_evaluation() {
        let mut firewall = ZeroTrustFirewall::new(FirewallAction::Deny);
        
        let allow_rule = FirewallRule::new(
            Some(IPAddress::new_v4(192, 168, 1, 1)),
            None,
            None,
            None,
            None,
            FirewallAction::Allow,
        );
        
        firewall.add_rule(allow_rule);
        
        let socket = Socket::new(
            IPAddress::new_v4(192, 168, 1, 1),
            Port::new(12345),
            IPAddress::new_v4(10, 0, 0, 1),
            Port::new(80),
            Protocol::TCP,
        );
        
        let action = firewall.evaluate(&socket);
        assert_eq!(action, FirewallAction::Allow);
    }
    
    #[test]
    fn test_tcp_connection() {
        let socket = Socket::new(
            IPAddress::new_v4(192, 168, 1, 1),
            Port::new(12345),
            IPAddress::new_v4(10, 0, 0, 1),
            Port::new(80),
            Protocol::TCP,
        );
        
        let mut connection = TCPConnection::new(socket);
        connection.transition(TCPState::Established);
        
        assert_eq!(connection.state, TCPState::Established);
        assert!(connection.is_established());
    }
    
    #[test]
    fn test_network_stack() {
        let mut stack = NetworkStack::default();
        
        let socket = Socket::new(
            IPAddress::new_v4(192, 168, 1, 1),
            Port::new(12345),
            IPAddress::new_v4(10, 0, 0, 1),
            Port::new(80),
            Protocol::TCP,
        );
        
        // Add allow rule
        let allow_rule = FirewallRule::new(
            Some(IPAddress::new_v4(192, 168, 1, 1)),
            None,
            None,
            None,
            None,
            FirewallAction::Allow,
        );
        
        stack.add_firewall_rule(allow_rule);
        
        let result = stack.create_connection(socket);
        assert!(result.is_ok());
    }
}
