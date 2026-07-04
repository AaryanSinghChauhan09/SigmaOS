// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/observability/sigma_metrics.rs — Sigma Metrics (Prometheus/Grafana)
//
// Implements Prometheus/Grafana-style metrics collection with time series
// data, labels, aggregation, and querying capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Metrics Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: String,  // counter, gauge, histogram, summary
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct HistogramBucket {
    pub upper_bound: f64,
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct Histogram {
    pub name: String,
    pub sum: f64,
    pub count: u64,
    pub buckets: Vec<HistogramBucket>,
}

#[derive(Debug, Clone)]
pub struct Query {
    pub metric_name: String,
    pub label_filters: Vec<String>,
    pub range_start: u64,
    pub range_end: u64,
}

// ─── Metrics Manager ────────────────────────────────────────────────────

pub struct MetricsManager {
    pub metrics: Vec<Metric>,
    pub histograms: HashMap<String, Histogram>,
    pub targets: Vec<String>,
    pub scraping_enabled: bool,
}

impl MetricsManager {
    pub fn new() -> Self {
        let mut manager = MetricsManager {
            metrics: Vec::new(),
            histograms: HashMap::new(),
            targets: Vec::new(),
            scraping_enabled: true,
        };
        
        manager.init_sample_metrics();
        manager.init_sample_histograms();
        manager
    }

    /// Initialize sample metrics
    fn init_sample_metrics(&mut self) {
        let mut labels1 = HashMap::new();
        labels1.insert("method".to_string(), "GET".to_string());
        labels1.insert("path".to_string(), "/api/users".to_string());
        labels1.insert("status".to_string(), "200".to_string());
        
        self.metrics.push(Metric {
            name: "http_requests_total".to_string(),
            metric_type: "counter".to_string(),
            value: 15234.0,
            labels: labels1,
            timestamp: 1704067200,
        });

        let mut labels2 = HashMap::new();
        labels2.insert("method".to_string(), "POST".to_string());
        labels2.insert("path".to_string(), "/api/users".to_string());
        labels2.insert("status".to_string(), "201".to_string());
        
        self.metrics.push(Metric {
            name: "http_requests_total".to_string(),
            metric_type: "counter".to_string(),
            value: 3421.0,
            labels: labels2,
            timestamp: 1704067200,
        });

        let mut labels3 = HashMap::new();
        labels3.insert("instance".to_string(), "server-01".to_string());
        labels3.insert("job".to_string(), "node_exporter".to_string());
        
        self.metrics.push(Metric {
            name: "cpu_usage_percent".to_string(),
            metric_type: "gauge".to_string(),
            value: 45.7,
            labels: labels3,
            timestamp: 1704067200,
        });
    }

    /// Initialize sample histograms
    fn init_sample_histograms(&mut self) {
        let buckets = vec![
            HistogramBucket { upper_bound: 0.1, count: 100 },
            HistogramBucket { upper_bound: 0.5, count: 250 },
            HistogramBucket { upper_bound: 1.0, count: 400 },
            HistogramBucket { upper_bound: 5.0, count: 480 },
            HistogramBucket { upper_bound: f64::INFINITY, count: 500 },
        ];
        
        self.histograms.insert("http_request_duration_seconds".to_string(), Histogram {
            name: "http_request_duration_seconds".to_string(),
            sum: 234.5,
            count: 500,
            buckets,
        });
    }

    /// Add metric
    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    /// Increment counter
    pub fn increment_counter(&mut self, name: &str, labels: HashMap<String, String>, value: f64) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: "counter".to_string(),
            value,
            labels,
            timestamp: current_timestamp(),
        };
        self.metrics.push(metric);
    }

    /// Set gauge value
    pub fn set_gauge(&mut self, name: &str, labels: HashMap<String, String>, value: f64) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: "gauge".to_string(),
            value,
            labels,
            timestamp: current_timestamp(),
        };
        self.metrics.push(metric);
    }

    /// Observe histogram
    pub fn observe_histogram(&mut self, name: &str, value: f64) {
        if let Some(histogram) = self.histograms.get_mut(name) {
            histogram.sum += value;
            histogram.count += 1;
            
            for bucket in &mut histogram.buckets {
                if value <= bucket.upper_bound {
                    bucket.count += 1;
                }
            }
        }
    }

    /// Query metrics
    pub fn query(&self, query: Query) -> Vec<&Metric> {
        self.metrics.iter()
            .filter(|m| m.name == query.metric_name)
            .filter(|m| {
                query.label_filters.iter().all(|filter| {
                    let parts: Vec<&str> = filter.split('=').collect();
                    if parts.len() == 2 {
                        m.labels.get(parts[0]).map(|v| v == parts[1]).unwrap_or(false)
                    } else {
                        true
                    }
                })
            })
            .collect()
    }

    /// Get metric by name
    pub fn get_metrics_by_name(&self, name: &str) -> Vec<&Metric> {
        self.metrics.iter().filter(|m| m.name == name).collect()
    }

    /// Get histogram by name
    pub fn get_histogram(&self, name: &str) -> Option<&Histogram> {
        self.histograms.get(name)
    }

    /// Add scrape target
    pub fn add_target(&mut self, target: String) {
        self.targets.push(target);
    }

    /// Get all targets
    pub fn get_all_targets(&self) -> Vec<&String> {
        self.targets.iter().collect()
    }

    /// Toggle scraping
    pub fn toggle_scraping(&mut self) {
        self.scraping_enabled = !self.scraping_enabled;
    }
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = MetricsManager::new();
    
    println!("Sigma Metrics v0.1 - Prometheus/Grafana Style");
    
    loop {
        println!("\n--- Metrics Status ---");
        println!("Scraping: {}", manager.scraping_enabled);
        println!("Metrics: {}", manager.metrics.len());
        println!("Histograms: {}", manager.histograms.len());
        println!("Targets: {}", manager.targets.len());
        
        println!("\nCommands: inc_counter <name> <value>, set_gauge <name> <value>, observe <histogram> <value>, query <name>, metrics <name>, histogram <name>, add_target <url>, targets, toggle, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "inc_counter" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    if let Ok(value) = parts[2].parse::<f64>() {
                        manager.increment_counter(&name, HashMap::new(), value);
                        println!("Counter incremented");
                    }
                }
            }
            "set_gauge" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    if let Ok(value) = parts[2].parse::<f64>() {
                        manager.set_gauge(&name, HashMap::new(), value);
                        println!("Gauge set");
                    }
                }
            }
            "observe" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    if let Ok(value) = parts[2].parse::<f64>() {
                        manager.observe_histogram(&name, value);
                        println!("Histogram observed");
                    }
                }
            }
            "query" => {
                if let Some(arg) = parts.get(1) {
                    let query = Query {
                        metric_name: arg.to_string(),
                        label_filters: vec![],
                        range_start: 0,
                        range_end: current_timestamp(),
                    };
                    let results = manager.query(query);
                    println!("--- Query Results ---");
                    for metric in results {
                        println!("{} = {} {:?}", metric.name, metric.value, metric.labels);
                    }
                }
            }
            "metrics" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Metrics: {} ---", arg);
                    for metric in manager.get_metrics_by_name(arg) {
                        println!("{} = {} {:?}", metric.name, metric.value, metric.labels);
                    }
                }
            }
            "histogram" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(histogram) = manager.get_histogram(arg) {
                        println!("--- Histogram: {} ---", histogram.name);
                        println!("Sum: {}, Count: {}", histogram.sum, histogram.count);
                        println!("Buckets:");
                        for bucket in &histogram.buckets {
                            println!("  <= {}: {}", bucket.upper_bound, bucket.count);
                        }
                    }
                }
            }
            "add_target" => {
                if let Some(arg) = parts.get(1) {
                    manager.add_target(arg.to_string());
                    println!("Target added");
                }
            }
            "targets" => {
                println!("--- Scrape Targets ---");
                for target in manager.get_all_targets() {
                    println!("{}", target);
                }
            }
            "toggle" => {
                manager.toggle_scraping();
                println!("Scraping {}", if manager.scraping_enabled { "enabled" } else { "disabled" });
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
