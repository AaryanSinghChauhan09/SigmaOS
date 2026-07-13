// SovereignNetStack CLI
// Command-line interface for network stack management

use sovereign_netstack::{FirewallAction, FirewallRule, IPAddress, NetworkStack, Port, Protocol, Socket, TCPState, ZeroTrustFirewall};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut stack = NetworkStack::default();
    
    match args[1].as_str() {
        "connect" => handle_connect(&mut stack, &args),
        "list" => handle_list(&stack),
        "close" => handle_close(&mut stack, &args),
        "rule" => handle_rule(&mut stack, &args),
        "firewall" => handle_firewall(&stack),
        "established" => handle_established(&stack),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SovereignNetStack CLI");
    println!();
    println!("Usage:");
    println!("  netstack connect <local_ip> <local_port> <remote_ip> <remote_port> <protocol>");
    println!("  netstack list");
    println!("  netstack close <connection_id>");
    println!("  netstack rule <action> <source_ip> <dest_ip> <protocol>");
    println!("  netstack firewall");
    println!("  netstack established");
    println!();
    println!("Example:");
    println!("  netstack connect 192.168.1.1 12345 10.0.0.1 80 tcp");
    println!("  netstack rule allow 192.168.1.1 10.0.0.1 tcp");
    println!("  netstack close connection_id");
}

fn handle_connect(stack: &mut NetworkStack, args: &[String]) {
    if args.len() < 7 {
        eprintln!("Error: Insufficient arguments for connect command");
        print_usage();
        std::process::exit(1);
    }
    
    let local_ip = parse_ip(&args[2]);
    let local_port: u16 = args[3].parse().expect("Invalid local port");
    let remote_ip = parse_ip(&args[4]);
    let remote_port: u16 = args[5].parse().expect("Invalid remote port");
    let protocol_str = &args[6];
    
    let protocol = match protocol_str.to_lowercase().as_str() {
        "tcp" => Protocol::TCP,
        "udp" => Protocol::UDP,
        "icmp" => Protocol::ICMP,
        _ => {
            eprintln!("Error: Invalid protocol. Use: tcp, udp, icmp");
            std::process::exit(1);
        }
    };
    
    let socket = Socket::new(
        local_ip,
        Port::new(local_port),
        remote_ip,
        Port::new(remote_port),
        protocol,
    );
    
    match stack.create_connection(socket) {
        Ok(connection_id) => {
            println!("Connection created successfully!");
            println!("Connection ID: {}", connection_id);
            println!();
            
            if let Some(connection) = stack.get_connection(&connection_id) {
                println!("{}", connection);
            }
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_list(stack: &NetworkStack) {
    let connections = stack.list_connections();
    
    if connections.is_empty() {
        println!("No connections found.");
        return;
    }
    
    println!("Network Connections ({}):", connections.len());
    println!();
    
    for connection in connections {
        println!("ID: {}", connection.get_connection_id());
        println!("Local: {}:{} ({})", 
            connection.socket.local_ip,
            connection.socket.local_port.number,
            connection.socket.protocol.as_str()
        );
        println!("Remote: {}:{}",
            connection.socket.remote_ip,
            connection.socket.remote_port.number
        );
        println!("State: {}", connection.state.as_str());
        println!();
    }
}

fn handle_close(stack: &mut NetworkStack, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Connection ID required");
        print_usage();
        std::process::exit(1);
    }
    
    let connection_id = &args[2];
    
    match stack.close_connection(connection_id) {
        Ok(_) => {
            println!("Connection closed successfully");
            if let Some(connection) = stack.get_connection(connection_id) {
                println!("New state: {}", connection.state.as_str());
            }
        }
        Err(e) => {
            eprintln!("Failed to close connection: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_rule(stack: &mut NetworkStack, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for rule command");
        print_usage();
        std::process::exit(1);
    }
    
    let action_str = &args[2];
    let source_ip_str = &args[3];
    let dest_ip_str = &args[4];
    let protocol_str = &args[5];
    
    let action = match action_str.to_lowercase().as_str() {
        "allow" => FirewallAction::Allow,
        "deny" => FirewallAction::Deny,
        "log" => FirewallAction::Log,
        _ => {
            eprintln!("Error: Invalid action. Use: allow, deny, log");
            std::process::exit(1);
        }
    };
    
    let source_ip = if source_ip_str == "any" {
        None
    } else {
        Some(parse_ip(source_ip_str))
    };
    
    let dest_ip = if dest_ip_str == "any" {
        None
    } else {
        Some(parse_ip(dest_ip_str))
    };
    
    let protocol = if protocol_str == "any" {
        None
    } else {
        Some(match protocol_str.to_lowercase().as_str() {
            "tcp" => Protocol::TCP,
            "udp" => Protocol::UDP,
            "icmp" => Protocol::ICMP,
            _ => {
                eprintln!("Error: Invalid protocol. Use: tcp, udp, icmp, any");
                std::process::exit(1);
            }
        })
    };
    
    let rule = FirewallRule::new(source_ip, None, dest_ip, None, protocol, action);
    
    stack.add_firewall_rule(rule);
    
    println!("Firewall rule added successfully");
    println!("Action: {}", action.as_str());
}

fn handle_firewall(stack: &NetworkStack) {
    let rules = stack.get_firewall_rules();
    
    if rules.is_empty() {
        println!("No firewall rules configured.");
        return;
    }
    
    println!("Firewall Rules ({}):", rules.len());
    println!();
    
    for rule in rules {
        println!("Rule ID: {}", rule.get_rule_id());
        println!("Action: {}", rule.action.as_str());
        println!("Enabled: {}", rule.enabled);
        
        if let Some(ref ip) = rule.source_ip {
            println!("Source IP: {}", ip);
        }
        
        if let Some(ref ip) = rule.dest_ip {
            println!("Dest IP: {}", ip);
        }
        
        if let Some(protocol) = rule.protocol {
            println!("Protocol: {}", protocol.as_str());
        }
        
        println!();
    }
}

fn handle_established(stack: &NetworkStack) {
    let connections = stack.get_established_connections();
    
    if connections.is_empty() {
        println!("No established connections.");
        return;
    }
    
    println!("Established Connections ({}):", connections.len());
    println!();
    
    for connection in connections {
        println!("ID: {}", connection.get_connection_id());
        println!("Local: {}:{}",
            connection.socket.local_ip,
            connection.socket.local_port.number
        );
        println!("Remote: {}:{}",
            connection.socket.remote_ip,
            connection.socket.remote_port.number
        );
        println!();
    }
}

fn parse_ip(s: &str) -> IPAddress {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 4 {
        let bytes: Vec<u8> = parts.iter().map(|p| p.parse().unwrap_or(0)).collect();
        IPAddress::new_v4(bytes[0], bytes[1], bytes[2], bytes[3])
    } else {
        // Default to localhost if parsing fails
        IPAddress::new_v4(127, 0, 0, 1)
    }
}
