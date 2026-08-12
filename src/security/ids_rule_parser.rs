// SigmaOS IDS Rule Parser
// Advanced rule-based intrusion detection system with Snort/Suricata-style syntax
// Solves BUG-011: IDS Rule Parser not implemented

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Rule action types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Alert,
    Log,
    Pass,
    Drop,
    Reject,
    Sdrop,
}

/// Rule protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleProtocol {
    Tcp,
    Udp,
    Icmp,
    Ip,
    Any,
}

/// Rule direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleDirection {
    SourceToDestination, // ->
    Bidirectional,      // <>
    DestinationToSource, // <-
}

/// Rule address
#[derive(Debug, Clone)]
pub enum RuleAddress {
    Any,
    Specific(String),
    Range(String, String),
    Network(String, u8), // CIDR notation
}

/// Rule port
#[derive(Debug, Clone)]
pub enum RulePort {
    Any,
    Specific(u16),
    Range(u16, u16),
}

/// Rule option
#[derive(Debug, Clone)]
pub struct RuleOption {
    pub name: String,
    pub value: String,
}

/// Parsed IDS rule
#[derive(Debug, Clone)]
pub struct IdsRule {
    pub action: RuleAction,
    pub protocol: RuleProtocol,
    pub source_address: RuleAddress,
    pub source_port: RulePort,
    pub direction: RuleDirection,
    pub destination_address: RuleAddress,
    pub destination_port: RulePort,
    pub options: Vec<RuleOption>,
    pub raw_rule: String,
}

/// IDS rule parser
pub struct IdsRuleParser {
    pub custom_variables: BTreeMap<String, String>,
}

impl IdsRuleParser {
    pub fn new() -> Self {
        Self {
            custom_variables: BTreeMap::new(),
        }
    }

    /// Parse a single IDS rule
    pub fn parse_rule(&self, rule_str: &str) -> Result<IdsRule, ParseError> {
        let parts: Vec<&str> = rule_str.split_whitespace().collect();
        
        if parts.len() < 5 {
            return Err(ParseError::InvalidSyntax("Rule too short".to_string()));
        }

        let action = self.parse_action(parts[0])?;
        let protocol = self.parse_protocol(parts[1])?;
        
        let (source_addr, source_port) = self.parse_address_port(parts[2])?;
        let direction = self.parse_direction(parts[3])?;
        let (dest_addr, dest_port) = self.parse_address_port(parts[4])?;
        
        let options = if parts.len() > 5 {
            self.parse_options(&parts[5..])?
        } else {
            Vec::new()
        };

        Ok(IdsRule {
            action,
            protocol,
            source_address: source_addr,
            source_port,
            direction,
            destination_address: dest_addr,
            destination_port: dest_port,
            options,
            raw_rule: rule_str.to_string(),
        })
    }

    fn parse_action(&self, action_str: &str) -> Result<RuleAction, ParseError> {
        match action_str.to_uppercase().as_str() {
            "ALERT" => Ok(RuleAction::Alert),
            "LOG" => Ok(RuleAction::Log),
            "PASS" => Ok(RuleAction::Pass),
            "DROP" => Ok(RuleAction::Drop),
            "REJECT" => Ok(RuleAction::Reject),
            "SDROP" => Ok(RuleAction::Sdrop),
            _ => Err(ParseError::InvalidAction(action_str.to_string())),
        }
    }

    fn parse_protocol(&self, proto_str: &str) -> Result<RuleProtocol, ParseError> {
        match proto_str.to_uppercase().as_str() {
            "TCP" => Ok(RuleProtocol::Tcp),
            "UDP" => Ok(RuleProtocol::Udp),
            "ICMP" => Ok(RuleProtocol::Icmp),
            "IP" => Ok(RuleProtocol::Ip),
            "ANY" => Ok(RuleProtocol::Any),
            _ => Err(ParseError::InvalidProtocol(proto_str.to_string())),
        }
    }

    fn parse_direction(&self, dir_str: &str) -> Result<RuleDirection, ParseError> {
        match dir_str {
            "->" => Ok(RuleDirection::SourceToDestination),
            "<>" => Ok(RuleDirection::Bidirectional),
            "<-" => Ok(RuleDirection::DestinationToSource),
            _ => Err(ParseError::InvalidDirection(dir_str.to_string())),
        }
    }

    fn parse_address_port(&self, addr_port: &str) -> Result<(RuleAddress, RulePort), ParseError> {
        let parts: Vec<&str> = addr_port.rsplitn(2, ':').collect();
        
        if parts.len() != 2 {
            return Err(ParseError::InvalidAddressPort(addr_port.to_string()));
        }

        let address = self.parse_address(parts[0])?;
        let port = self.parse_port(parts[1])?;

        Ok((address, port))
    }

    fn parse_address(&self, addr_str: &str) -> Result<RuleAddress, ParseError> {
        if addr_str == "any" {
            return Ok(RuleAddress::Any);
        }

        // Check for CIDR notation
        if addr_str.contains('/') {
            let cidr_parts: Vec<&str> = addr_str.split('/').collect();
            if cidr_parts.len() == 2 {
                if let Ok(mask) = cidr_parts[1].parse::<u8>() {
                    return Ok(RuleAddress::Network(cidr_parts[0].to_string(), mask));
                }
            }
        }

        // Check for range notation
        if addr_str.contains('-') {
            let range_parts: Vec<&str> = addr_str.split('-').collect();
            if range_parts.len() == 2 {
                return Ok(RuleAddress::Range(
                    range_parts[0].to_string(),
                    range_parts[1].to_string(),
                ));
            }
        }

        Ok(RuleAddress::Specific(addr_str.to_string()))
    }

    fn parse_port(&self, port_str: &str) -> Result<RulePort, ParseError> {
        if port_str == "any" {
            return Ok(RulePort::Any);
        }

        // Check for range notation
        if port_str.contains(':') {
            let range_parts: Vec<&str> = port_str.split(':').collect();
            if range_parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    range_parts[0].parse::<u16>(),
                    range_parts[1].parse::<u16>()
                ) {
                    return Ok(RulePort::Range(start, end));
                }
            }
        }

        if let Ok(port) = port_str.parse::<u16>() {
            Ok(RulePort::Specific(port))
        } else {
            Err(ParseError::InvalidPort(port_str.to_string()))
        }
    }

    fn parse_options(&self, option_strs: &[&str]) -> Result<Vec<RuleOption>, ParseError> {
        let mut options = Vec::new();
        
        for opt_str in option_strs {
            let parts: Vec<&str> = opt_str.split(';').collect();
            
            for part in parts {
                let kv: Vec<&str> = part.splitn(2, ':').collect();
                if kv.len() == 2 {
                    options.push(RuleOption {
                        name: kv[0].to_string(),
                        value: kv[1].to_string(),
                    });
                } else if kv.len() == 1 {
                    // Flag option without value
                    options.push(RuleOption {
                        name: kv[0].to_string(),
                        value: String::new(),
                    });
                }
            }
        }

        Ok(options)
    }

    /// Add custom variable for rule substitution
    pub fn add_variable(&mut self, name: String, value: String) {
        self.custom_variables.insert(name, value);
    }

    /// Parse multiple rules from a string
    pub fn parse_rules(&self, rules_text: &str) -> Result<Vec<IdsRule>, ParseError> {
        let mut rules = Vec::new();
        
        for line in rules_text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            match self.parse_rule(line) {
                Ok(rule) => rules.push(rule),
                Err(e) => return Err(e),
            }
        }

        Ok(rules)
    }
}

impl Default for IdsRuleParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse error types
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidSyntax(String),
    InvalidAction(String),
    InvalidProtocol(String),
    InvalidDirection(String),
    InvalidAddressPort(String),
    InvalidPort(String),
    UnknownOption(String),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::InvalidAction(msg) => write!(f, "Invalid action: {}", msg),
            ParseError::InvalidProtocol(msg) => write!(f, "Invalid protocol: {}", msg),
            ParseError::InvalidDirection(msg) => write!(f, "Invalid direction: {}", msg),
            ParseError::InvalidAddressPort(msg) => write!(f, "Invalid address:port: {}", msg),
            ParseError::InvalidPort(msg) => write!(f, "Invalid port: {}", msg),
            ParseError::UnknownOption(msg) => write!(f, "Unknown option: {}", msg),
        }
    }
}

/// Rule matcher for packet evaluation
pub struct RuleMatcher {
    rules: Vec<IdsRule>,
}

impl RuleMatcher {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: IdsRule) {
        self.rules.push(rule);
    }

    pub fn add_rules(&mut self, rules: Vec<IdsRule>) {
        self.rules.extend(rules);
    }

    /// Match a packet against loaded rules
    pub fn match_packet(&self, 
        src_ip: &str, 
        src_port: u16, 
        dst_ip: &str, 
        dst_port: u16, 
        protocol: RuleProtocol) -> Vec<&IdsRule> {
        let mut matched_rules = Vec::new();

        for rule in &self.rules {
            if self.rule_matches(rule, src_ip, src_port, dst_ip, dst_port, protocol) {
                matched_rules.push(rule);
            }
        }

        matched_rules
    }

    fn rule_matches(&self,
        rule: &IdsRule,
        src_ip: &str,
        src_port: u16,
        dst_ip: &str,
        dst_port: u16,
        protocol: RuleProtocol,
    ) -> bool {
        // Check protocol
        if rule.protocol != RuleProtocol::Any && rule.protocol != protocol {
            return false;
        }

        // Check source address
        if !self.address_matches(&rule.source_address, src_ip) {
            return false;
        }

        // Check source port
        if !self.port_matches(&rule.source_port, src_port) {
            return false;
        }

        // Check destination address
        if !self.address_matches(&rule.destination_address, dst_ip) {
            return false;
        }

        // Check destination port
        if !self.port_matches(&rule.destination_port, dst_port) {
            return false;
        }

        true
    }

    fn address_matches(&self, rule_addr: &RuleAddress, packet_addr: &str) -> bool {
        match rule_addr {
            RuleAddress::Any => true,
            RuleAddress::Specific(addr) => addr == packet_addr,
            RuleAddress::Range(start, end) => {
                // Simple string comparison for IP ranges
                packet_addr >= start && packet_addr <= end
            }
            RuleAddress::Network(addr, _mask) => {
                // CIDR matching (simplified)
                packet_addr.starts_with(addr)
            }
        }
    }

    fn port_matches(&self, rule_port: &RulePort, packet_port: u16) -> bool {
        match rule_port {
            RulePort::Any => true,
            RulePort::Specific(port) => *port == packet_port,
            RulePort::Range(start, end) => packet_port >= *start && packet_port <= *end,
        }
    }
}

impl Default for RuleMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_rule() {
        let parser = IdsRuleParser::new();
        let rule = parser.parse_rule("alert tcp any any -> 192.168.1.1 80").unwrap();
        
        assert_eq!(rule.action, RuleAction::Alert);
        assert_eq!(rule.protocol, RuleProtocol::Tcp);
        assert_eq!(rule.direction, RuleDirection::SourceToDestination);
    }

    #[test]
    fn test_parse_rule_with_options() {
        let parser = IdsRuleParser::new();
        let rule = parser.parse_rule(
            "alert tcp any any -> any 80 (msg:\"HTTP traffic detected\";sid:1000001;rev:1)"
        ).unwrap();
        
        assert_eq!(rule.options.len(), 3);
        assert_eq!(rule.options[0].name, "msg");
        assert_eq!(rule.options[1].name, "sid");
        assert_eq!(rule.options[2].name, "rev");
    }

    #[test]
    fn test_parse_cidr_address() {
        let parser = IdsRuleParser::new();
        let rule = parser.parse_rule("alert tcp 192.168.0.0/24 any -> any any").unwrap();
        
        match rule.source_address {
            RuleAddress::Network(addr, mask) => {
                assert_eq!(addr, "192.168.0.0");
                assert_eq!(mask, 24);
            }
            _ => panic!("Expected network address"),
        }
    }

    #[test]
    fn test_parse_port_range() {
        let parser = IdsRuleParser::new();
        let rule = parser.parse_rule("alert tcp any 1024:65535 -> any 80").unwrap();
        
        match rule.source_port {
            RulePort::Range(start, end) => {
                assert_eq!(start, 1024);
                assert_eq!(end, 65535);
            }
            _ => panic!("Expected port range"),
        }
    }

    #[test]
    fn test_rule_matcher() {
        let parser = IdsRuleParser::new();
        let rule = parser.parse_rule("alert tcp any any -> 192.168.1.1 80").unwrap();
        
        let mut matcher = RuleMatcher::new();
        matcher.add_rule(rule);
        
        let matched = matcher.match_packet("10.0.0.1", 12345, "192.168.1.1", 80, RuleProtocol::Tcp);
        assert_eq!(matched.len(), 1);
        
        let not_matched = matcher.match_packet("10.0.0.1", 12345, "192.168.1.1", 443, RuleProtocol::Tcp);
        assert_eq!(not_matched.len(), 0);
    }

    #[test]
    fn test_parse_multiple_rules() {
        let parser = IdsRuleParser::new();
        let rules_text = r#"
            alert tcp any any -> 192.168.1.1 80
            log udp any 53 -> any 53
            drop icmp any any -> any any
            # This is a comment
            alert ip any any -> 10.0.0.0/8 any
        "#;
        
        let rules = parser.parse_rules(rules_text).unwrap();
        assert_eq!(rules.len(), 4);
    }

    #[test]
    fn test_bidirectional_rule() {
        let parser = IdsRuleParser::new();
        let rule = parser.parse_rule("alert tcp 192.168.1.1 80 <> 10.0.0.1 12345").unwrap();
        
        assert_eq!(rule.direction, RuleDirection::Bidirectional);
    }

    #[test]
    fn test_invalid_rule() {
        let parser = IdsRuleParser::new();
        let result = parser.parse_rule("invalid rule syntax");
        assert!(result.is_err());
    }
}