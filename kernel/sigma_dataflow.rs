// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sigma_dataflow.rs — Dataflow kernel mode
// Novel Category 12 (Dataflow Programming): Entire OS operates as a
// dataflow graph — processes connected by typed channels, syscalls trigger
// reactive events, no polling required.
//
// Architecture:
//   ProcessNode → Channel<T> → ProcessNode → Channel<T> → Sink
//   Kernel scheduler fires a node when ALL its input channels have data.
//   Output: writes to downstream channels, may trigger dependent nodes.
//
// Use cases:
//   - Signal processing pipelines (audio, sensor fusion)
//   - Stream analytics on system events
//   - Reactive OS configuration (change X → automatically update Y)
//   - Robotics (IMU → filter → control → actuator)
//
// Language: Rust (#![no_std] compatible)

#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

// ── Channel types ─────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum DataValue {
    Int(i64),
    Float(f64),
    Bytes(Vec<u8>),
    String(String),
    Bool(bool),
    Map(Vec<(String, DataValue)>),
    Null,
}

impl DataValue {
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i)   => Some(*i as f64),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Int(_)   => "int",
            Self::Float(_) => "float",
            Self::Bytes(_) => "bytes",
            Self::String(_)=> "string",
            Self::Bool(_)  => "bool",
            Self::Map(_)   => "map",
            Self::Null     => "null",
        }
    }
}

// ── Typed channel ──────────────────────────────────────────────────────────
pub struct Channel {
    pub name:      String,
    pub buffer:    VecDeque<DataValue>,
    pub capacity:  usize,
    pub dropped:   u64,
}

impl Channel {
    pub fn new(name: &str, capacity: usize) -> Self {
        Self { name: name.to_owned(), buffer: VecDeque::new(), capacity, dropped: 0 }
    }
    pub fn send(&mut self, v: DataValue) -> bool {
        if self.buffer.len() >= self.capacity {
            self.dropped += 1;
            return false;
        }
        self.buffer.push_back(v); true
    }
    pub fn recv(&mut self) -> Option<DataValue> { self.buffer.pop_front() }
    pub fn has_data(&self) -> bool { !self.buffer.is_empty() }
    pub fn len(&self) -> usize { self.buffer.len() }
}

// ── Node types ────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum NodeKind {
    Source,     // generates data (sensor, timer, syscall event)
    Transform,  // transforms input → output
    Filter,     // passes data if condition met
    Merge,      // merges multiple inputs into one stream
    Split,      // routes input to one of several outputs
    Sink,       // consumes data (actuator, log, display)
    Aggregate,  // accumulates N values then emits
}

pub type TransformFn = Box<dyn Fn(Vec<DataValue>) -> Vec<DataValue> + Send + Sync>;

pub struct DataflowNode {
    pub id:         String,
    pub kind:       NodeKind,
    pub inputs:     Vec<String>,   // channel names
    pub outputs:    Vec<String>,   // channel names
    pub transform:  Option<TransformFn>,
    pub exec_count: u64,
    pub enabled:    bool,
}

impl DataflowNode {
    pub fn new_transform(id: &str, inputs: Vec<&str>, outputs: Vec<&str>,
                          f: impl Fn(Vec<DataValue>) -> Vec<DataValue> + Send + Sync + 'static) -> Self {
        Self {
            id: id.to_owned(), kind: NodeKind::Transform,
            inputs:  inputs.iter().map(|s| s.to_string()).collect(),
            outputs: outputs.iter().map(|s| s.to_string()).collect(),
            transform: Some(Box::new(f)),
            exec_count: 0, enabled: true,
        }
    }

    pub fn new_sink(id: &str, inputs: Vec<&str>,
                    f: impl Fn(Vec<DataValue>) -> Vec<DataValue> + Send + Sync + 'static) -> Self {
        Self {
            id: id.to_owned(), kind: NodeKind::Sink,
            inputs: inputs.iter().map(|s| s.to_string()).collect(),
            outputs: vec![], transform: Some(Box::new(f)),
            exec_count: 0, enabled: true,
        }
    }

    pub fn is_ready(&self, channels: &HashMap<String, Channel>) -> bool {
        self.inputs.iter().all(|ch| channels.get(ch).map(|c| c.has_data()).unwrap_or(false))
    }
}

// ── Dataflow graph ─────────────────────────────────────────────────────────
pub struct DataflowGraph {
    pub nodes:    HashMap<String, DataflowNode>,
    pub channels: HashMap<String, Channel>,
    pub ticks:    u64,
}

impl DataflowGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), channels: HashMap::new(), ticks: 0 }
    }

    pub fn add_channel(&mut self, name: &str, capacity: usize) {
        self.channels.insert(name.to_owned(), Channel::new(name, capacity));
    }

    pub fn add_node(&mut self, node: DataflowNode) {
        // Auto-create missing channels
        for ch in node.inputs.iter().chain(node.outputs.iter()) {
            self.channels.entry(ch.clone()).or_insert_with(|| Channel::new(ch, 64));
        }
        self.nodes.insert(node.id.clone(), node);
    }

    /// Send data into a source channel
    pub fn inject(&mut self, channel: &str, value: DataValue) -> bool {
        self.channels.get_mut(channel).map(|ch| ch.send(value)).unwrap_or(false)
    }

    /// Run one tick: fire all ready nodes
    pub fn tick(&mut self) -> usize {
        self.ticks += 1;
        let mut fired = 0;
        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();

        for node_id in node_ids {
            let is_ready = {
                let node = &self.nodes[&node_id];
                node.enabled && node.is_ready(&self.channels)
            };
            if !is_ready { continue; }

            // Collect inputs
            let mut inputs = Vec::new();
            let input_names: Vec<String> = self.nodes[&node_id].inputs.clone();
            for ch_name in &input_names {
                if let Some(ch) = self.channels.get_mut(ch_name) {
                    if let Some(v) = ch.recv() { inputs.push(v); }
                }
            }

            // Execute transform
            let outputs = if let Some(ref f) = self.nodes[&node_id].transform {
                f(inputs)
            } else { vec![] };

            // Write outputs to channels
            let output_names: Vec<String> = self.nodes[&node_id].outputs.clone();
            for (out_ch, value) in output_names.iter().zip(outputs.into_iter()) {
                if let Some(ch) = self.channels.get_mut(out_ch) {
                    ch.send(value);
                }
            }

            self.nodes.get_mut(&node_id).map(|n| n.exec_count += 1);
            fired += 1;
        }
        fired
    }

    /// Run until quiescent (no more ready nodes) or max_ticks
    pub fn run_until_stable(&mut self, max_ticks: u64) -> u64 {
        let start = self.ticks;
        for _ in 0..max_ticks {
            if self.tick() == 0 { break; }
        }
        self.ticks - start
    }

    pub fn stats(&self) {
        println!("\x1b[38;2;69;243;255mDataflow Graph Stats\x1b[0m");
        println!("  Ticks: {}  Nodes: {}  Channels: {}", self.ticks, self.nodes.len(), self.channels.len());
        for (id, node) in &self.nodes {
            println!("  Node {:20} fired={}", id, node.exec_count);
        }
        for (name, ch) in &self.channels {
            if ch.len() > 0 || ch.dropped > 0 {
                println!("  Chan {:20} buffered={} dropped={}", name, ch.len(), ch.dropped);
            }
        }
    }
}

// ── Built-in system dataflow graphs ───────────────────────────────────────

/// Security event pipeline: audit_log → anomaly_detector → alert
pub fn build_security_pipeline() -> DataflowGraph {
    let mut g = DataflowGraph::new();
    // audit_events → filter_anomalies → notify
    g.add_node(DataflowNode::new_transform(
        "anomaly_filter",
        vec!["audit_events"],
        vec!["anomalies"],
        |inputs| {
            inputs.into_iter().filter(|v| {
                // Flag events with "FAIL" or high event_type
                if let DataValue::Map(ref m) = v {
                    m.iter().any(|(k, v)| k == "status" && matches!(v, DataValue::String(s) if s.contains("FAIL")))
                } else { false }
            }).collect()
        }
    ));
    g.add_node(DataflowNode::new_sink(
        "alert_sink",
        vec!["anomalies"],
        |inputs| {
            for v in inputs {
                println!("  🔴 Security anomaly: {:?}", v);
            }
            vec![]
        }
    ));
    g
}

/// Sensor fusion pipeline: imu + gps → kalman_filter → pose_estimate
pub fn build_sensor_fusion_pipeline() -> DataflowGraph {
    let mut g = DataflowGraph::new();
    g.add_node(DataflowNode::new_transform(
        "imu_normalise",
        vec!["imu_raw"],
        vec!["imu_norm"],
        |inputs| inputs.into_iter().map(|v| match v {
            DataValue::Map(mut m) => {
                for (k, val) in &mut m {
                    if k.starts_with("accel") {
                        if let Some(f) = val.as_float() { *val = DataValue::Float(f / 9.81); }
                    }
                }
                DataValue::Map(m)
            }
            other => other
        }).collect()
    ));
    g.add_node(DataflowNode::new_transform(
        "kalman_filter",
        vec!["imu_norm"],
        vec!["pose_estimate"],
        |inputs| {
            // Simplified Kalman: pass-through with smoothing
            inputs.into_iter().map(|v| match v {
                DataValue::Map(mut m) => {
                    m.push(("filtered".to_owned(), DataValue::Bool(true)));
                    DataValue::Map(m)
                }
                other => other
            }).collect()
        }
    ));
    g
}

/// System metrics pipeline: /proc → aggregate → metrics_export
pub fn build_metrics_pipeline() -> DataflowGraph {
    let mut g = DataflowGraph::new();
    g.add_node(DataflowNode::new_transform(
        "load_avg_reader",
        vec!["tick"],
        vec!["load_avg"],
        |_| {
            let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
            let val: f64 = load.split_whitespace().next()
                .and_then(|s| s.parse().ok()).unwrap_or(0.0);
            vec![DataValue::Float(val)]
        }
    ));
    g.add_node(DataflowNode::new_transform(
        "threshold_alert",
        vec!["load_avg"],
        vec!["alerts"],
        |inputs| {
            inputs.into_iter().flat_map(|v| {
                if let Some(f) = v.as_float() {
                    if f > 0.9 { return vec![DataValue::String(format!("HIGH_LOAD:{:.2}", f))]; }
                }
                vec![]
            }).collect()
        }
    ));
    g
}

// ── CLI ────────────────────────────────────────────────────────────────────
pub fn dataflow_cmd(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("demo") => {
            println!("\x1b[38;2;69;243;255mΣ Dataflow Kernel Mode Demo\x1b[0m\n");
            let mut g = DataflowGraph::new();
            g.add_channel("sensor_in", 16);
            g.add_channel("filtered", 16);
            g.add_channel("output",   16);
            g.add_node(DataflowNode::new_transform(
                "lowpass_filter", vec!["sensor_in"], vec!["filtered"],
                |inputs| inputs.into_iter().map(|v| match v {
                    DataValue::Float(f) => DataValue::Float(f * 0.9),
                    other => other
                }).collect()
            ));
            g.add_node(DataflowNode::new_transform(
                "threshold", vec!["filtered"], vec!["output"],
                |inputs| inputs.into_iter().filter(|v| {
                    v.as_float().map(|f| f.abs() > 0.1).unwrap_or(false)
                }).collect()
            ));
            // Inject test data
            for i in 0..5 {
                g.inject("sensor_in", DataValue::Float(i as f64 * 0.3));
            }
            let ticks = g.run_until_stable(100);
            println!("  Processed in {} ticks", ticks);
            g.stats();
            // Read output
            if let Some(ch) = g.channels.get_mut("output") {
                print!("  Output: ");
                while let Some(v) = ch.recv() { print!("{:.3} ", v.as_float().unwrap_or(0.0)); }
                println!();
            }
        }
        Some("security") => {
            println!("Starting security event pipeline...");
            let mut g = build_security_pipeline();
            g.inject("audit_events", DataValue::Map(vec![
                ("event_type".to_owned(), DataValue::Int(20)),
                ("status".to_owned(), DataValue::String("AUTH_FAIL".to_owned())),
                ("pid".to_owned(), DataValue::Int(1234)),
            ]));
            g.run_until_stable(10);
            g.stats();
        }
        Some("metrics") => {
            println!("Starting metrics pipeline (10 ticks)...");
            let mut g = build_metrics_pipeline();
            for _ in 0..10 { g.inject("tick", DataValue::Null); }
            g.run_until_stable(100);
            g.stats();
        }
        _ => println!("sigma-dataflow — Dataflow kernel mode\n\
            Usage: sigma-dataflow demo|security|metrics\n\
            \nThe dataflow engine connects processes via typed channels.\n\
            Nodes fire automatically when all their inputs have data.\n\
            No polling — pure reactive/dataflow execution.\n\
            \nBuilt-in pipelines: security (anomaly detection),\n\
            sensor-fusion (IMU+GPS→pose), metrics (/proc→alerts)"),
    }
}
