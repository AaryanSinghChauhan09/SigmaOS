use std::vec::Vec;
use std::string::{String, ToString};
use std::collections::BTreeMap;

/// AI Anomaly Detection Firewall
/// Inspired by CrowdStrike Falcon and Snort, providing ML-based behavioral analysis
/// for detecting security anomalies in system behavior

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyType {
    UnusualProcessBehavior,
    SuspiciousNetworkActivity,
    AbnormalResourceUsage,
    UnauthorizedFileAccess,
    StrangeSystemCallPattern,
    PotentialMalwareSignature,
}

#[derive(Debug, Clone)]
pub struct AnomalyEvent {
    pub timestamp: u64,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub source_process: String,
    pub description: String,
    pub confidence_score: f32, // 0.0 to 1.0
    pub affected_resources: Vec<String>,
}

impl AnomalyEvent {
    pub fn new(
        anomaly_type: AnomalyType,
        severity: AnomalySeverity,
        source_process: &str,
        description: &str,
        confidence_score: f32,
    ) -> Self {
        Self {
            timestamp: 0, // Would be set by system
            anomaly_type,
            severity,
            source_process: source_process.to_string(),
            description: description.to_string(),
            confidence_score: confidence_score.clamp(0.0, 1.0),
            affected_resources: Vec::new(),
        }
    }

    pub fn with_affected_resources(mut self, resources: Vec<String>) -> Self {
        self.affected_resources = resources;
        self
    }
}

/// Behavioral baseline for normal system activity
#[derive(Debug, Clone)]
pub struct BehavioralBaseline {
    pub process_normal_cpu_usage: BTreeMap<String, f32>, // process_id -> normal cpu %
    pub process_normal_memory_usage: BTreeMap<String, f32>, // process_id -> normal memory %
    pub normal_network_connections: BTreeMap<String, u32>, // process_id -> normal connection count
    pub normal_file_access_patterns: BTreeMap<String, Vec<String>>, // process_id -> allowed paths
    pub baseline_window_size: u64, // seconds of data to consider for baseline
}

impl BehavioralBaseline {
    pub fn new() -> Self {
        Self {
            process_normal_cpu_usage: BTreeMap::new(),
            process_normal_memory_usage: BTreeMap::new(),
            normal_network_connections: BTreeMap::new(),
            normal_file_access_patterns: BTreeMap::new(),
            baseline_window_size: 3600, // 1 hour default
        }
    }

    pub fn update_cpu_baseline(&mut self, process: &str, usage: f32) {
        let entry = self.process_normal_cpu_usage.entry(process.to_string()).or_insert(0.0);
        // Exponential moving average
        *entry = *entry * 0.9 + usage * 0.1;
    }

    pub fn update_memory_baseline(&mut self, process: &str, usage: f32) {
        let entry = self.process_normal_memory_usage.entry(process.to_string()).or_insert(0.0);
        *entry = *entry * 0.9 + usage * 0.1;
    }

    pub fn add_allowed_path(&mut self, process: &str, path: &str) {
        let entry = self.normal_file_access_patterns
            .entry(process.to_string())
            .or_insert_with(Vec::new);
        if !entry.contains(&path.to_string()) {
            entry.push(path.to_string());
        }
    }
}

/// AI-based anomaly detection engine
pub struct AiAnomalyDetector {
    pub baseline: BehavioralBaseline,
    pub detected_anomalies: Vec<AnomalyEvent>,
    pub detection_threshold: f32, // confidence threshold for alerting
    pub learning_mode: bool, // if true, updates baseline instead of alerting
}

impl AiAnomalyDetector {
    pub fn new() -> Self {
        Self {
            baseline: BehavioralBaseline::new(),
            detected_anomalies: Vec::new(),
            detection_threshold: 0.7, // 70% confidence threshold
            learning_mode: true, // Start in learning mode
        }
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.detection_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    pub fn with_learning_mode(mut self, enabled: bool) -> Self {
        self.learning_mode = enabled;
        self
    }

    /// Analyze CPU usage for anomalies
    pub fn analyze_cpu_usage(&mut self, process: &str, current_usage: f32) -> Option<AnomalyEvent> {
        if let Some(&normal_usage) = self.baseline.process_normal_cpu_usage.get(process) {
            let deviation = (current_usage - normal_usage).abs();
            let relative_deviation = if normal_usage > 0.0 {
                deviation / normal_usage
            } else {
                deviation
            };

            if relative_deviation > 2.0 { // More than 200% deviation
                let confidence = (relative_deviation / 5.0).clamp(0.0, 1.0);
                
                if confidence >= self.detection_threshold && !self.learning_mode {
                    let anomaly = AnomalyEvent::new(
                        AnomalyType::UnusualProcessBehavior,
                        if relative_deviation > 5.0 { AnomalySeverity::Critical } else { AnomalySeverity::High },
                        process,
                        &format!("CPU usage anomaly: {:.1}% vs baseline {:.1}%", current_usage, normal_usage),
                        confidence,
                    );
                    self.detected_anomalies.push(anomaly.clone());
                    return Some(anomaly);
                }
            } else if self.learning_mode {
                self.baseline.update_cpu_baseline(process, current_usage);
            }
        } else if self.learning_mode {
            self.baseline.update_cpu_baseline(process, current_usage);
        }

        None
    }

    /// Analyze memory usage for anomalies
    pub fn analyze_memory_usage(&mut self, process: &str, current_usage: f32) -> Option<AnomalyEvent> {
        if let Some(&normal_usage) = self.baseline.process_normal_memory_usage.get(process) {
            let deviation = (current_usage - normal_usage).abs();
            let relative_deviation = if normal_usage > 0.0 {
                deviation / normal_usage
            } else {
                deviation
            };

            if relative_deviation > 1.5 { // More than 150% deviation
                let confidence = (relative_deviation / 3.0).clamp(0.0, 1.0);
                
                if confidence >= self.detection_threshold && !self.learning_mode {
                    let anomaly = AnomalyEvent::new(
                        AnomalyType::AbnormalResourceUsage,
                        if relative_deviation > 3.0 { AnomalySeverity::High } else { AnomalySeverity::Medium },
                        process,
                        &format!("Memory usage anomaly: {:.1}% vs baseline {:.1}%", current_usage, normal_usage),
                        confidence,
                    );
                    self.detected_anomalies.push(anomaly.clone());
                    return Some(anomaly);
                }
            } else if self.learning_mode {
                self.baseline.update_memory_baseline(process, current_usage);
            }
        } else if self.learning_mode {
            self.baseline.update_memory_baseline(process, current_usage);
        }

        None
    }

    /// Analyze network activity for anomalies
    pub fn analyze_network_activity(&mut self, process: &str, connection_count: u32) -> Option<AnomalyEvent> {
        if let Some(&normal_count) = self.baseline.normal_network_connections.get(process) {
            let ratio = if normal_count > 0 {
                connection_count as f32 / normal_count as f32
            } else {
                connection_count as f32
            };

            if ratio > 3.0 { // More than 3x normal connections
                let confidence = (ratio / 10.0).clamp(0.0, 1.0);
                
                if confidence >= self.detection_threshold && !self.learning_mode {
                    let anomaly = AnomalyEvent::new(
                        AnomalyType::SuspiciousNetworkActivity,
                        if ratio > 10.0 { AnomalySeverity::Critical } else { AnomalySeverity::High },
                        process,
                        &format!("Network connection anomaly: {} vs baseline {}", connection_count, normal_count),
                        confidence,
                    );
                    self.detected_anomalies.push(anomaly.clone());
                    return Some(anomaly);
                }
            }
        } else if self.learning_mode {
            self.baseline.normal_network_connections.insert(process.to_string(), connection_count);
        }

        None
    }

    /// Analyze file access for anomalies
    pub fn analyze_file_access(&mut self, process: &str, accessed_path: &str) -> Option<AnomalyEvent> {
        if let Some(allowed_paths) = self.baseline.normal_file_access_patterns.get(process) {
            if !allowed_paths.contains(&accessed_path.to_string()) {
                // Check if it's a suspicious path
                let is_suspicious = accessed_path.contains("/etc/") || 
                                   accessed_path.contains("/root/") ||
                                   accessed_path.contains("/var/log/") ||
                                   accessed_path.contains(".ssh/") ||
                                   accessed_path.contains(".config/");

                if is_suspicious && !self.learning_mode {
                    let anomaly = AnomalyEvent::new(
                        AnomalyType::UnauthorizedFileAccess,
                        AnomalySeverity::High,
                        process,
                        &format!("Unauthorized file access: {}", accessed_path),
                        0.85,
                    ).with_affected_resources(vec![accessed_path.to_string()]);
                    self.detected_anomalies.push(anomaly.clone());
                    return Some(anomaly);
                }
            }
        } else if self.learning_mode {
            self.baseline.add_allowed_path(process, accessed_path);
        }

        None
    }

    /// Get critical anomalies that need immediate attention
    pub fn get_critical_anomalies(&self) -> Vec<&AnomalyEvent> {
        self.detected_anomalies
            .iter()
            .filter(|a| a.severity == AnomalySeverity::Critical)
            .collect()
    }

    /// Get high severity anomalies
    pub fn get_high_severity_anomalies(&self) -> Vec<&AnomalyEvent> {
        self.detected_anomalies
            .iter()
            .filter(|a| a.severity == AnomalySeverity::High)
            .collect()
    }

    /// Clear old anomalies (keep recent ones)
    pub fn clear_old_anomalies(&mut self, keep_count: usize) {
        if self.detected_anomalies.len() > keep_count {
            let remove_count = self.detected_anomalies.len() - keep_count;
            for _ in 0..remove_count {
                self.detected_anomalies.remove(0);
            }
        }
    }

    /// Get anomaly statistics
    pub fn get_statistics(&self) -> AnomalyStatistics {
        let total = self.detected_anomalies.len();
        let critical = self.detected_anomalies.iter().filter(|a| a.severity == AnomalySeverity::Critical).count();
        let high = self.detected_anomalies.iter().filter(|a| a.severity == AnomalySeverity::High).count();
        let medium = self.detected_anomalies.iter().filter(|a| a.severity == AnomalySeverity::Medium).count();
        let low = self.detected_anomalies.iter().filter(|a| a.severity == AnomalySeverity::Low).count();

        AnomalyStatistics {
            total_anomalies: total,
            critical_count: critical,
            high_count: high,
            medium_count: medium,
            low_count: low,
            learning_mode: self.learning_mode,
        }
    }
}

impl Default for AiAnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct AnomalyStatistics {
    pub total_anomalies: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub learning_mode: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_event_creation() {
        let event = AnomalyEvent::new(
            AnomalyType::UnusualProcessBehavior,
            AnomalySeverity::High,
            "test_process",
            "Test anomaly description",
            0.8,
        );
        assert_eq!(event.confidence_score, 0.8);
        assert_eq!(event.severity, AnomalySeverity::High);
    }

    #[test]
    fn test_behavioral_baseline() {
        let mut baseline = BehavioralBaseline::new();
        baseline.update_cpu_baseline("test", 50.0);
        assert_eq!(baseline.process_normal_cpu_usage.get("test"), Some(&50.0));
    }

    #[test]
    fn test_ai_anomaly_detector_learning_mode() {
        let mut detector = AiAnomalyDetector::new().with_learning_mode(true);
        
        // Should update baseline in learning mode
        detector.analyze_cpu_usage("test", 30.0);
        assert_eq!(detector.baseline.process_normal_cpu_usage.get("test"), Some(&30.0));
    }

    #[test]
    fn test_ai_anomaly_detector_detection_mode() {
        let mut detector = AiAnomalyDetector::new()
            .with_learning_mode(false)
            .with_threshold(0.5);

        // Set baseline
        detector.baseline.update_cpu_baseline("test", 10.0);
        
        // Anomalous usage should be detected
        let anomaly = detector.analyze_cpu_usage("test", 50.0);
        assert!(anomaly.is_some());
        assert_eq!(detector.detected_anomalies.len(), 1);
    }

    #[test]
    fn test_file_access_anomaly_detection() {
        let mut detector = AiAnomalyDetector::new()
            .with_learning_mode(false)
            .with_threshold(0.5);

        detector.baseline.add_allowed_path("test", "/home/user/documents");
        
        // Suspicious file access should be detected
        let anomaly = detector.analyze_file_access("test", "/etc/passwd");
        assert!(anomaly.is_some());
        assert_eq!(anomaly.unwrap().anomaly_type, AnomalyType::UnauthorizedFileAccess);
    }

    #[test]
    fn test_network_activity_anomaly() {
        let mut detector = AiAnomalyDetector::new()
            .with_learning_mode(false)
            .with_threshold(0.5);

        detector.baseline.normal_network_connections.insert("test".to_string(), 5);
        
        // High connection count should be detected
        let anomaly = detector.analyze_network_activity("test", 50);
        assert!(anomaly.is_some());
    }

    #[test]
    fn test_anomaly_statistics() {
        let mut detector = AiAnomalyDetector::new();
        
        detector.detected_anomalies.push(AnomalyEvent::new(
            AnomalyType::UnusualProcessBehavior,
            AnomalySeverity::Critical,
            "test",
            "Test",
            0.9,
        ));
        
        let stats = detector.get_statistics();
        assert_eq!(stats.total_anomalies, 1);
        assert_eq!(stats.critical_count, 1);
    }
}