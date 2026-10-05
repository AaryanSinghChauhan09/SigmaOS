# Desktop Network Manager

## Overview

The Desktop Network Manager provides comprehensive network management inspired by Linux Mint's NetworkManager and Omarchy's network utilities. It supports multiple connection types, connection status tracking, security configuration, and IP address management.

## Features

- **Connection Types**: Ethernet, WiFi, VPN, Bluetooth, Other
- **Connection Status**: Disconnected, Connecting, Connected, Failed
- **Security Types**: None, WEP, WPA-PSK, WPA-EAP, WPA2-PSK, WPA2-EAP, WPA3-PSK, WPA3-EAP
- **Connection Management**: Add, remove, and manage network connections
- **Status Management**: Track and update connection status
- **Connect/Disconnect**: Connect and disconnect connections
- **Active Connection**: Track currently active connection
- **Auto-Connect**: Configure auto-connect for connections
- **IP Configuration**: IP address, gateway, and DNS server management
- **Connection Filtering**: Filter connections by type or status
- **Statistics**: Track connection counts and status distribution

## Components

### NetConnectionType

```rust
pub enum NetConnectionType {
    Ethernet,  // Wired Ethernet connection
    WiFi,      // Wireless WiFi connection
    VPN,       // Virtual Private Network
    Bluetooth, // Bluetooth connection
    Other,     // Other connection type
}
```

### NetConnectionStatus

```rust
pub enum NetConnectionStatus {
    Disconnected, // Connection is disconnected
    Connecting,   // Connection is connecting
    Connected,     // Connection is connected
    Failed,        // Connection failed
}
```

### SecurityType

```rust
pub enum SecurityType {
    None,       // No security
    WEP,        // WEP security
    WPA_PSK,    // WPA-PSK security
    WPA_EAP,    // WPA-EAP security
    WPA2_PSK,   // WPA2-PSK security
    WPA2_EAP,   // WPA2-EAP security
    WPA3_PSK,   // WPA3-PSK security
    WPA3_EAP,   // WPA3-EAP security
}
```

### NetworkConnection

Network connection with:
- Connection ID and name
- Connection type
- Status
- Interface name
- IP address
- Gateway
- DNS servers
- Security type
- Auto-connect flag

### DesktopNetworkManager

Main management interface with:
- Connection management (add, remove, retrieve)
- Status management
- Connect/disconnect functionality
- Active connection tracking
- Auto-connect configuration
- IP address, gateway, and DNS configuration
- Connection filtering by type or status
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopNetworkManager;

let mut manager = DesktopNetworkManager::new();

// Get configuration
println!("Total connections: {}", manager.get_connections().len());
println!("Active connection: {:?}", manager.get_active_connection());
```

### Connection Management

```rust
// Add connection
let conn = NetworkConnection::new(
    "custom".to_string(),
    "Custom Connection".to_string(),
    NetConnectionType::WiFi,
    "wlan1".to_string(),
)
.with_security(SecurityType::WPA3_PSK);

let id = manager.add_connection(conn);

// Remove connection
manager.remove_connection(&id);
```

### Status Management

```rust
// Set connection status
manager.set_connection_status(&conn_id, NetConnectionStatus::Connected);
manager.set_connection_status(&conn_id, NetConnectionStatus::Disconnected);
```

### Connect/Disconnect

```rust
// Connect
manager.connect(&conn_id);

// Disconnect
manager.disconnect(&conn_id);

// Get active connection
if let Some(active) = manager.get_active_connection() {
    println!("Active: {}", active.name);
}
```

### Auto-Connect

```rust
// Enable auto-connect
manager.set_auto_connect(&conn_id, true);

// Disable auto-connect
manager.set_auto_connect(&conn_id, false);
```

### IP Configuration

```rust
// Set IP address
manager.update_ip_address(&conn_id, "192.168.1.100".to_string());

// Set gateway
manager.update_gateway(&conn_id, "192.168.1.1".to_string());

// Set DNS servers
let dns = vec![
    "8.8.8.8".to_string(),
    "8.8.4.4".to_string(),
];
manager.update_dns_servers(&conn_id, dns);
```

### Connection Filtering

```rust
// Get connections by type
let ethernet = manager.get_connections_by_type(NetConnectionType::Ethernet);
let wifi = manager.get_connections_by_type(NetConnectionType::WiFi);
let vpn = manager.get_connections_by_type(NetConnectionType::VPN);

// Get connections by status
let connected = manager.get_connections_by_status(NetConnectionStatus::Connected);
let disconnected = manager.get_connections_by_status(NetConnectionStatus::Disconnected);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total connections: {}", stats.total_connections);
println!("Connected: {}", stats.connected);
println!("Ethernet connections: {}", stats.ethernet_connections);
println!("WiFi connections: {}", stats.wifi_connections);
println!("VPN connections: {}", stats.vpn_connections);
println!("Active connection set: {}", stats.active_connection_set);
```

## Default Connections

The manager includes default connections:

- **Wired Connection 1**: Ethernet on eth0
- **WiFi Connection**: WiFi on wlan0 with WPA2-PSK security

## Default Configuration

The Network Manager includes default configuration:

- **Default Connections**: 2 (Ethernet and WiFi)
- **Active Connection**: None (initially disconnected)

## AI Agent Maintenance Instructions

When maintaining the Network Manager:

1. **NetworkManager Integration**: Integrate with NetworkManager daemon
2. **WiFi Scanning**: Implement WiFi network scanning
3. **WiFi Hotspot**: Add WiFi hotspot creation
4. **VPN Configuration**: Add VPN configuration and management
5. **Bonding**: Add network bonding/teaming
6. **Bridge**: Add network bridge configuration
7. **VLAN**: Add VLAN configuration
8. **IPv6**: Add full IPv6 support
9. **Metered Connections**: Add metered connection tracking
10. **Connection Profiles**: Add connection profiles for different networks

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::network_manager
```

## Future Enhancements

- NetworkManager daemon integration
- WiFi network scanning
- WiFi hotspot creation
- VPN configuration and management
- Network bonding/teaming
- Network bridge configuration
- VLAN configuration
- Full IPv6 support
- Metered connection tracking
- Connection profiles for different networks
