# Network Firewall Manager

## Overview

The Network Firewall Manager provides comprehensive firewall management inspired by Linux Mint's firewall and Omarchy's firewall utilities. It supports rule management, protocol filtering, zone configuration, and traffic control.

## Features

- **Firewall Actions**: Allow, Deny, Reject, Log
- **Protocols**: TCP, UDP, ICMP, All
- **Rule Management**: Create, enable, disable, remove rules
- **Source/Destination**: Configure source and destination addresses
- **Port Filtering**: Port-specific rules
- **Firewall Zones**: Organize rules into zones with default actions
- **Interface Assignment**: Assign interfaces to zones
- **Default Rules**: Pre-configured loopback, established connections, and SSH rules
- **Statistics**: Track rule counts by action and zone counts

## Components

### FirewallAction

```rust
pub enum FirewallAction {
    Allow,   // Allow traffic
    Deny,    // Deny traffic silently
    Reject,  // Reject traffic with ICMP error
    Log,     // Log traffic
}
```

### Protocol

```rust
pub enum Protocol {
    TCP,   // TCP protocol
    UDP,   // UDP protocol
    ICMP,  // ICMP protocol
    All,   // All protocols
}
```

### NetworkFirewallRule

Represents a firewall rule with:
- Unique rule ID
- Action (allow/deny/reject/log)
- Protocol
- Source address
- Destination address
- Port (optional)
- Enabled flag
- Description

### NetworkFirewallZone

Represents a firewall zone with:
- Unique zone ID
- Zone name
- Default action
- Interface list

### NetworkFirewallManager

Main management interface with:
- Rule creation and management
- Zone management
- Enable/disable rules
- Default rule configuration
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::security::NetworkFirewallManager;

let manager = NetworkFirewallManager::new();

// List all rules
let rules = manager.list_rules();
for rule in rules {
    println!("{}: {} ({})", rule.id, rule.description, rule.action.as_str());
}
```

### Rule Management

```rust
// Create a rule
let id = manager.create_rule(
    FirewallAction::Allow,
    Protocol::TCP,
    Some(80),
    "Allow HTTP".to_string(),
);

// Get a rule
if let Some(rule) = manager.get_rule(&id) {
    println!("Rule: {}", rule.description);
}

// Enable/disable a rule
manager.enable_rule(&id)?;
manager.disable_rule(&id)?;

// Remove a rule
manager.remove_rule(&id)?;
```

### Custom Rules

```rust
// Add a custom rule
let mut rule = NetworkFirewallRule::new(
    "custom-rule".to_string(),
    FirewallAction::Allow,
    Protocol::TCP,
    "Custom rule".to_string(),
);
rule.set_source("192.168.1.0/24".to_string());
rule.set_destination("0.0.0.0/0".to_string());
rule.set_port(443);
manager.add_rule(rule);
```

### Zone Management

```rust
// Add a zone
let mut zone = NetworkFirewallZone::new(
    "home".to_string(),
    "Home Network".to_string(),
    FirewallAction::Allow,
);
zone.add_interface("eth0".to_string());
manager.add_zone(zone);

// Get a zone
if let Some(zone) = manager.get_zone("home") {
    println!("Zone: {}", zone.name);
}

// List all zones
let zones = manager.list_zones();
```

### Protocol Filtering

```rust
// Allow TCP on port 80
manager.create_rule(
    FirewallAction::Allow,
    Protocol::TCP,
    Some(80),
    "HTTP".to_string(),
);

// Allow UDP on port 53
manager.create_rule(
    FirewallAction::Allow,
    Protocol::UDP,
    Some(53),
    "DNS".to_string(),
);

// Allow ICMP
manager.create_rule(
    FirewallAction::Allow,
    Protocol::ICMP,
    None,
    "ICMP".to_string(),
);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total rules: {}", stats.total_rules);
println!("Enabled rules: {}", stats.enabled_rules);
println!("Allow rules: {}", stats.allow_rules);
println!("Deny rules: {}", stats.deny_rules);
println!("Total zones: {}", stats.total_zones);
```

## Default Configuration

The Network Firewall Manager includes pre-configured rules and zones:

**Default Rules:**
- **loopback**: Allow loopback traffic (127.0.0.1/8)
- **established**: Allow established connections
- **ssh**: Allow SSH on port 22

**Default Zones:**
- **public**: Public zone (default deny)
- **trusted**: Trusted zone (default allow)

## AI Agent Maintenance Instructions

When maintaining the Network Firewall Manager:

1. **IP Validation**: Validate IP addresses and CIDR notation
2. **Port Validation**: Ensure ports are within valid range (1-65535)
3. **Rule Ordering**: Maintain proper rule ordering for evaluation
4. **Zone Consistency**: Ensure zone configurations are consistent
5. **Interface Validation**: Validate interface names before assignment
6. **Rule Conflicts**: Detect and warn about conflicting rules

## Testing

Run the unit tests with:

```bash
cargo test --lib security::firewall_manager
```

## Future Enhancements

- Integration with actual firewall (iptables, nftables, pf)
- Port forwarding/NAT support
- Rate limiting
- Connection tracking
- Stateful inspection
- Application layer filtering
- Firewall profiles
- Import/export rules
- Rule validation and testing
- Real-time logging
- Alert notifications
