# Network Manager

## Overview

The Network Manager provides comprehensive network management inspired by Linux Mint's NetworkManager and Omarchy's network utilities. It supports WiFi, Ethernet, VPN, and network configuration management.

## Features

- **Connection Types**: Ethernet, WiFi, VPN, Bluetooth, Cellular
- **Connection Status**: Disconnected, Connecting, Connected, Failed
- **WiFi Security**: Open, WEP, WPA-PSK, WPA2-PSK, WPA3-PSK, WPA-EAP
- **Network Connections**: Configurable network connection definitions
- **WiFi Scanning**: Discover available WiFi networks
- **Auto-Connect**: Automatic connection to known networks
- **IP Configuration**: IP address, gateway, DNS server management
- **Network Statistics**: Connection count and type statistics

## Components

### ConnectionType

```rust
pub enum ConnectionType {
    Ethernet,  // Wired Ethernet connection
    WiFi,      // Wireless WiFi connection
    VPN,       // Virtual Private Network
    Bluetooth, // Bluetooth network
    Cellular,  // Cellular/mobile data
}
```

### ConnectionStatus

```rust
pub enum ConnectionStatus {
    Disconnected,  // Not connected
    Connecting,     // Connection in progress
    Connected,      // Successfully connected
    Failed,         // Connection failed
}
```

### WiFiSecurity

```rust
pub enum WiFiSecurity {
    Open,      // No security
    WEP,       // WEP encryption
    WPA_PSK,   // WPA Personal
    WPA2_PSK,  // WPA2 Personal
    WPA3_PSK,  // WPA3 Personal
    WPA_EAP,   // WPA Enterprise
}
```

### NetworkConnection

Represents a network connection with:
- Connection ID and name
- Connection type
- Status
- Interface name
- IP address, gateway, DNS servers
- Auto-connect flag

### WiFiNetwork

Represents a WiFi network with:
- SSID and BSSID
- Security type
- Signal strength (0-100)
- Frequency (MHz)
- Hidden network flag

### NetworkConfig

Network manager configuration including:
- Auto-connect enabled
- Metered connections
- IPv6 enabled
- Firewall enabled

### NetworkManager

Main management interface with:
- Connection registration and management
- Connect/disconnect operations
- WiFi scanning
- WiFi connection with password
- Type-based filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::networking::NetworkManager;

let mut manager = NetworkManager::new();

// List all connections
let connections = manager.list_connections();

// Get configuration
let config = manager.get_config();
```

### Connection Management

```rust
// Add a network connection
let conn = NetworkConnection::new(
    "eth0".to_string(),
    "Wired Connection".to_string(),
    ConnectionType::Ethernet,
    "eth0".to_string(),
);
manager.add_connection(conn);

// Connect to network
manager.connect("eth0")?;

// Disconnect from network
manager.disconnect("eth0")?;
```

### WiFi Management

```rust
// Scan for WiFi networks
let networks = manager.scan_wifi();

// Connect to WiFi network
manager.connect_wifi("HomeNetwork", Some("password".to_string()))?;

// Connect to open network
manager.connect_wifi("FreeWiFi", None)?;
```

### Filtering Connections

```rust
// List WiFi connections
let wifi = manager.list_by_type(ConnectionType::WiFi);

// List Ethernet connections
let ethernet = manager.list_by_type(ConnectionType::Ethernet);

// List connected connections
let connected = manager.list_connected();
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total connections: {}", stats.total_connections);
println!("Connected: {}", stats.connected_count);
println!("WiFi networks found: {}", stats.wifi_networks_found);
```

## AI Agent Maintenance Instructions

When maintaining the Network Manager:

1. **Connection Safety**: Ensure connection commands are validated before execution
2. **WiFi Security**: Maintain proper security type handling for WiFi connections
3. **IP Configuration**: Ensure IP address, gateway, and DNS are properly assigned
4. **Auto-Connect**: Maintain safe auto-connect behavior
5. **Connection State**: Ensure accurate connection status tracking
6. **Scanning Accuracy**: Keep WiFi scan results accurate and up-to-date

## Testing

Run the unit tests with:

```bash
cargo test --lib networking::network_manager
```

## Future Enhancements

- Integration with actual NetworkManager (Linux) or similar
- Hotspot configuration
- Bonding and teaming support
- VPN configuration (OpenVPN, WireGuard)
- IPv6 support
- Network metrics and monitoring
- Connection profiles
- DNS over HTTPS/TLS
