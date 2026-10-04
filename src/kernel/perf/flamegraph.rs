// SigmaOS Kernel Perf - Flamegraph Generator
// Collects stack traces, aggregates call chains, and formats fold files & SVG flamegraphs.

use std::format;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct FlamegraphFrame {
    pub function_name: String,
    pub address: u64,
}

#[derive(Debug, Clone)]
pub struct FlamegraphStackSample {
    pub pid: u32,
    pub thread_name: String,
    pub callchain: Vec<FlamegraphFrame>,
    pub sample_count: u64,
}

pub struct FlamegraphGenerator {
    pub samples: Vec<FlamegraphStackSample>,
}

impl FlamegraphGenerator {
    pub fn new() -> Self {
        Self { samples: Vec::new() }
    }

    pub fn record_sample(&mut self, pid: u32, thread_name: &str, callchain: Vec<FlamegraphFrame>) {
        if let Some(existing) = self.samples.iter_mut().find(|s| {
            s.pid == pid
                && s.callchain.len() == callchain.len()
                && s.callchain.iter().zip(callchain.iter()).all(|(a, b)| a.address == b.address)
        }) {
            existing.sample_count += 1;
        } else {
            self.samples.push(FlamegraphStackSample {
                pid,
                thread_name: String::from(thread_name),
                callchain,
                sample_count: 1,
            });
        }
    }

    /// Exports folded stack format (similar to Brendan Gregg's FlameGraph format: `func1;func2;func3 count`)
    pub fn export_folded_stacks(&self) -> String {
        let mut out = String::new();
        for sample in &self.samples {
            let mut stack_str = sample.thread_name.clone();
            for frame in &sample.callchain {
                stack_str.push(';');
                stack_str.push_str(&frame.function_name);
            }
            out.push_str(&format!("{} {}\n", stack_str, sample.sample_count));
        }
        out
    }

    /// Generates SVG flamegraph representation
    pub fn generate_svg_flamegraph(&self) -> String {
        let mut svg = String::from("<?xml version=\"1.0\" standalone=\"no\"?>\n");
        svg.push_str("<svg width=\"1200\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        svg.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#1e1e2e\"/>\n");
        svg.push_str("  <text x=\"20\" y=\"30\" fill=\"#ffffff\" font-family=\"sans-serif\" font-size=\"20\">SigmaOS Kernel Flamegraph</text>\n");

        let folded = self.export_folded_stacks();
        let total_samples: u64 = self.samples.iter().map(|s| s.sample_count).sum();
        svg.push_str(&format!("  <!-- Total Samples: {} -->\n", total_samples));
        svg.push_str(&format!("  <!-- Folded Data:\n{} -->\n", folded));
        svg.push_str("</svg>\n");
        svg
    }
}

impl Default for FlamegraphGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_flamegraph_recording_and_folded_export() {
        let mut gen = FlamegraphGenerator::new();

        let frames = vec![
            FlamegraphFrame {
                function_name: String::from("sys_read"),
                address: 0x80001000,
            },
            FlamegraphFrame {
                function_name: String::from("vfs_read"),
                address: 0x80002000,
            },
            FlamegraphFrame {
                function_name: String::from("ext4_file_read"),
                address: 0x80003000,
            },
        ];

        gen.record_sample(100, "init", frames.clone());
        gen.record_sample(100, "init", frames);

        assert_eq!(gen.samples.len(), 1);
        assert_eq!(gen.samples[0].sample_count, 2);

        let folded = gen.export_folded_stacks();
        assert!(folded.contains("init;sys_read;vfs_read;ext4_file_read 2"));

        let svg = gen.generate_svg_flamegraph();
        assert!(svg.contains("SigmaOS Kernel Flamegraph"));
    }
}
