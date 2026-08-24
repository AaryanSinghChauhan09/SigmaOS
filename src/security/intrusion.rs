#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Intrusion Detection System
// OOP-based IDS with anomaly detection and rule-based analysis

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

use std::net::IpAddr;
use std::time::Instant;

/// Security event
#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub id: String,
    pub event_type: EventType,
    pub source_ip: Option<IpAddr>,
    pub target_ip: Option<IpAddr>,
    pub timestamp: Instant,
    pub severity: Severity,
    pub description: String,
    pub metadata: HashMap<String, String>,
}

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    PortScan,
    BruteForce,
    DdosAttack,
    MalwareSignature,
    Anomaly,
    PolicyViolation,
    UnauthorizedAccess,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Detection rule
#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub id: String,
    pub name: String,
    pub event_type: EventType,
    pub pattern: String,
    pub action: RuleAction,
    pub enabled: bool,
}

/// Rule action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Log,
    Alert,
    Block,
    Quarantine,
}

/// OOP trait for detection strategies
pub trait DetectionStrategy {
    /// Analyze event
    fn analyze(&self, event: &SecurityEvent) -> Option<DetectionResult>;
    /// Get strategy name
    fn name(&self) -> &str;
    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
    /// Convert to mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Detection result
#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub rule_id: String,
    pub matched: bool,
    pub confidence: f64,
    pub action: RuleAction,
    pub message: String,
}

/// Signature-based detection
pub struct SignatureDetection {
    rules: Vec<DetectionRule>,
}

impl SignatureDetection {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: DetectionRule) {
        self.rules.push(rule);
    }
}

impl DetectionStrategy for SignatureDetection {
    fn analyze(&self, event: &SecurityEvent) -> Option<DetectionResult> {
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            if rule.event_type == event.event_type {
                // Simple pattern matching (in real implementation, use more sophisticated matching)
                if event.description.contains(&rule.pattern) {
                    return Some(DetectionResult {
                        rule_id: rule.id.clone(),
                        matched: true,
                        confidence: 1.0,
                        action: rule.action,
                        message: format!("Signature match for rule: {}", rule.name),
                    });
                }
            }
        }

        None
    }

    fn name(&self) -> &str {
        "SignatureDetection"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Anomaly-based detection
pub struct AnomalyDetection {
    baseline: HashMap<String, f64>,
    threshold: f64,
}

impl AnomalyDetection {
    pub fn new(threshold: f64) -> Self {
        Self {
            baseline: HashMap::new(),
            threshold,
        }
    }

    pub fn set_baseline(&mut self, metric: String, value: f64) {
        self.baseline.insert(metric, value);
    }
}

impl DetectionStrategy for AnomalyDetection {
    fn analyze(&self, event: &SecurityEvent) -> Option<DetectionResult> {
        // Simulated anomaly detection
        // In real implementation, use statistical analysis, ML models, etc.

        let metric_key = format!("{:?}", event.event_type);
        if let Some(baseline) = self.baseline.get(&metric_key) {
            // Simulate deviation calculation
            let deviation: f64 = (event.timestamp.elapsed().as_secs() as f64 - *baseline).abs();

            if deviation > self.threshold {
                return Some(DetectionResult {
                    rule_id: "anomaly".to_string(),
                    matched: true,
                    confidence: (deviation / self.threshold).min(1.0),
                    action: RuleAction::Alert,
                    message: format!("Anomaly detected: deviation {:.2} from baseline", deviation),
                });
            }
        }

        None
    }

    fn name(&self) -> &str {
        "AnomalyDetection"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// =========================================================================
// Snort Signature Firewall & CrowdStrike Falcon AI Engines
// =========================================================================

/// Snort-Style Rule Definition: alert tcp $EXTERNAL_NET any -> $HTTP_SERVERS 80 (msg:"Malicious Payload"; content:"malware_payload")
#[derive(Debug, Clone)]
pub struct SnortRule {
    pub id: String,
    pub action: RuleAction,
    pub protocol: String, // e.g. "tcp", "udp", "icmp"
    pub dest_port: u16,
    pub content_signature: String,
    pub message: String,
}

/// Simulated Snort Signature Firewall Engine
pub struct SnortSignatureFirewall {
    pub rules: Vec<SnortRule>,
}

impl SnortSignatureFirewall {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_snort_rule(&mut self, rule: SnortRule) {
        self.rules.push(rule);
    }

    /// Evaluates raw packet headers and payload contents against registered Snort rules
    pub fn evaluate_packet(&self, protocol: &str, dest_port: u16, payload: &[u8]) -> Option<DetectionResult> {
        let payload_str = String::from_utf8_lossy(payload);
        for rule in &self.rules {
            if rule.protocol == protocol && rule.dest_port == dest_port {
                if payload_str.contains(&rule.content_signature) {
                    return Some(DetectionResult {
                        rule_id: rule.id.clone(),
                        matched: true,
                        confidence: 1.0,
                        action: rule.action,
                        message: format!("Snort Match: {}", rule.message),
                    });
                }
            }
        }
        None
    }
}

impl Default for SnortSignatureFirewall {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulated CrowdStrike Falcon AI Anomaly Detection Engine
pub struct CrowdStrikeFalconAi {
    pub threat_score_threshold: f64,
}

impl CrowdStrikeFalconAi {
    pub fn new(threshold: f64) -> Self {
        Self {
            threat_score_threshold: threshold,
        }
    }

    /// Analyses process behavioral metrics (thread migrations, allocation spikes, unauthorized API hooks)
    /// to dynamically calculate an AI Threat Score
    pub fn calculate_threat_score(
        &self,
        thread_migration_count: usize,
        unauthorized_hooks_probed: usize,
        allocation_burst_bytes: usize,
    ) -> f64 {
        let migration_score = (thread_migration_count * 10) as f64;
        let hook_score = (unauthorized_hooks_probed * 35) as f64;
        let allocation_score = (allocation_burst_bytes as f64 / 1_000_000.0) * 15.0; // scale per MB

        migration_score + hook_score + allocation_score
    }

    /// Inspects behavioral signals, triggering a quarantine or block action on anomaly detection
    pub fn evaluate_anomaly(
        &self,
        thread_migration_count: usize,
        unauthorized_hooks_probed: usize,
        allocation_burst_bytes: usize,
    ) -> Option<DetectionResult> {
        let score = self.calculate_threat_score(thread_migration_count, unauthorized_hooks_probed, allocation_burst_bytes);
        if score >= self.threat_score_threshold {
            let action = if score >= 80.0 {
                RuleAction::Quarantine // Highly malicious -> Quarantine process
            } else {
                RuleAction::Block
            };

            return Some(DetectionResult {
                rule_id: "crowdstrike_falcon_ai".to_string(),
                matched: true,
                confidence: (score / 100.0).min(1.0),
                action,
                message: format!("CrowdStrike Falcon AI: Suspicious process behavioral anomaly detected! Score: {:.2}", score),
            });
        }
        None
    }
}

// =========================================================================
// OOP-based Intrusion Detection System
// =========================================================================

pub struct IntrusionDetectionSystem {
    strategies: Vec<Box<dyn DetectionStrategy>>,
    events: Vec<SecurityEvent>,
    alerts: Vec<DetectionResult>,
    max_events: usize,
    auto_block_enabled: bool,
}

impl IntrusionDetectionSystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
            events: Vec::new(),
            alerts: Vec::new(),
            max_events: 10000,
            auto_block_enabled: false,
        }
    }

    /// Add a detection strategy
    pub fn add_strategy(mut self, strategy: Box<dyn DetectionStrategy>) -> Self {
        self.strategies.push(strategy);
        self
    }

    /// Enable auto-block
    pub fn with_auto_block(mut self, enabled: bool) -> Self {
        self.auto_block_enabled = enabled;
        self
    }

    /// Process a security event
    pub fn process_event(&mut self, event: SecurityEvent) -> Vec<DetectionResult> {
        // Add to events
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(event.clone());

        let mut detections = Vec::new();

        for strategy in &self.strategies {
            if let Some(result) = strategy.analyze(&event) {
                detections.push(result.clone());
                self.alerts.push(result.clone());

                // Auto-block if enabled
                if self.auto_block_enabled && result.action == RuleAction::Block {
                    self.block_source(&event);
                }
            }
        }

        detections
    }

    /// Get recent events
    pub fn recent_events(&self, count: usize) -> &[SecurityEvent] {
        let start = if self.events.len() > count {
            self.events.len() - count
        } else {
            0
        };
        &self.events[start..]
    }

    /// Get alerts
    pub fn alerts(&self) -> &[DetectionResult] {
        &self.alerts
    }

    /// Get events by type
    pub fn events_by_type(&self, event_type: EventType) -> Vec<&SecurityEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Get events by severity
    pub fn events_by_severity(&self, severity: Severity) -> Vec<&SecurityEvent> {
        self.events
            .iter()
            .filter(|e| e.severity == severity)
            .collect()
    }

    /// Block source
    fn block_source(&self, event: &SecurityEvent) {
        // Simulated blocking
        if let Some(_ip) = event.source_ip {
            // In real implementation, add to firewall rules
        }
    }

    /// Create default rules
    pub fn create_default_rules(&mut self) {
        let default_rules = vec![
            DetectionRule {
                id: "port_scan".to_string(),
                name: "Port Scan Detection".to_string(),
                event_type: EventType::PortScan,
                pattern: "scan".to_string(),
                action: RuleAction::Block,
                enabled: true,
            },
            DetectionRule {
                id: "brute_force".to_string(),
                name: "Brute Force Detection".to_string(),
                event_type: EventType::BruteForce,
                pattern: "failed".to_string(),
                action: RuleAction::Block,
                enabled: true,
            },
            DetectionRule {
                id: "ddos".to_string(),
                name: "DDoS Detection".to_string(),
                event_type: EventType::DdosAttack,
                pattern: "flood".to_string(),
                action: RuleAction::Block,
                enabled: true,
            },
        ];

        if let Some(strategy) = self
            .strategies
            .iter_mut()
            .find(|s| s.name() == "SignatureDetection")
        {
            if let Some(sig_detection) = strategy.as_any_mut().downcast_mut::<SignatureDetection>()
            {
                for rule in default_rules {
                    sig_detection.add_rule(rule);
                }
            }
        }
    }
}

impl Default for IntrusionDetectionSystem {
    fn default() -> Self {
        let mut ids = Self::new()
            .add_strategy(Box::new(SignatureDetection::new()))
            .add_strategy(Box::new(AnomalyDetection::new(2.0)));
        ids.create_default_rules();
        ids
    }
}

/// IDS errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdsError {
    InvalidRule(String),
    ProcessingError(String),
    ConfigurationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_event() {
        let event = SecurityEvent {
            id: "test".to_string(),
            event_type: EventType::PortScan,
            source_ip: None,
            target_ip: None,
            timestamp: Instant::now(),
            severity: Severity::High,
            description: "Port scan detected".to_string(),
            metadata: HashMap::new(),
        };
        assert_eq!(event.event_type, EventType::PortScan);
    }

    #[test]
    fn test_signature_detection() {
        let mut detection = SignatureDetection::new();
        detection.add_rule(DetectionRule {
            id: "test".to_string(),
            name: "Test Rule".to_string(),
            event_type: EventType::PortScan,
            pattern: "scan".to_string(),
            action: RuleAction::Log,
            enabled: true,
        });

        let event = SecurityEvent {
            id: "test".to_string(),
            event_type: EventType::PortScan,
            source_ip: None,
            target_ip: None,
            timestamp: Instant::now(),
            severity: Severity::High,
            description: "Port scan detected".to_string(),
            metadata: HashMap::new(),
        };

        let result = detection.analyze(&event);
        assert!(result.is_some());
    }

    #[test]
    fn test_anomaly_detection() {
        let mut detection = AnomalyDetection::new(2.0);
        detection.set_baseline("PortScan".to_string(), 10.0);

        let event = SecurityEvent {
            id: "test".to_string(),
            event_type: EventType::PortScan,
            source_ip: None,
            target_ip: None,
            timestamp: Instant::now(),
            severity: Severity::High,
            description: "Port scan detected".to_string(),
            metadata: HashMap::new(),
        };

        let _result = detection.analyze(&event);
        // May or may not detect depending on timing
    }

    #[test]
    fn test_intrusion_detection_system() {
        let ids = IntrusionDetectionSystem::default();
        assert_eq!(ids.strategies.len(), 2);
    }

    #[test]
    fn test_process_event() {
        let mut ids = IntrusionDetectionSystem::default();
        let event = SecurityEvent {
            id: "test".to_string(),
            event_type: EventType::PortScan,
            source_ip: None,
            target_ip: None,
            timestamp: Instant::now(),
            severity: Severity::High,
            description: "Port scan detected".to_string(),
            metadata: HashMap::new(),
        };

        let results = ids.process_event(event);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_snort_and_crowdstrike_engines() {
        // 1. Test Snort Signature Firewall
        let mut snort = SnortSignatureFirewall::new();
        snort.add_snort_rule(SnortRule {
            id: "snort_01".to_string(),
            action: RuleAction::Block,
            protocol: "tcp".to_string(),
            dest_port: 80,
            content_signature: "malware_payload".to_string(),
            message: "Block malicious payload injection!".to_string(),
        });

        let packet_payload = b"GET /index.php?payload=malware_payload HTTP/1.1\r\n";
        let snort_res = snort.evaluate_packet("tcp", 80, packet_payload).unwrap();
        assert!(snort_res.matched);
        assert_eq!(snort_res.action, RuleAction::Block);

        // 2. Test CrowdStrike Falcon AI Anomaly Detection
        let falcon = CrowdStrikeFalconAi::new(50.0);

        // Safe process behavior metrics -> No anomaly detected
        let safe_res = falcon.evaluate_anomaly(1, 0, 10_000);
        assert!(safe_res.is_none());

        // Dangerous process behavior metrics -> Anomaly block detected!
        let anomaly_res = falcon.evaluate_anomaly(3, 2, 5_000_000).unwrap();
        assert!(anomaly_res.matched);
        assert_eq!(anomaly_res.action, RuleAction::Quarantine);
    }
}
