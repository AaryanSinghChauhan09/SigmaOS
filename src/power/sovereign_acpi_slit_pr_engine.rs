// SigmaOS Physical ACPI SLIT Table Parsing & NUMA Topology PR Engine
// Implements PR-formatted deployment for ACPI 6.5 System Locality Information Table (SLIT)
// parsing, dynamic NUMA inter-node distance matrix population, and scheduler integration.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(feature = "standalone_test"))]
use crate::power::acpi_table_parser::{ParsedSlit, SLIT_SIGNATURE};

#[cfg(feature = "standalone_test")]
#[path = "acpi_table_parser.rs"]
pub mod acpi_table_parser;
#[cfg(feature = "standalone_test")]
use acpi_table_parser::{ParsedSlit, SLIT_SIGNATURE};

/// ACPI SLIT PR Submission Metadata
#[derive(Debug, Clone)]
pub struct AcpiSlitPrSubmissionSpec {
    pub pr_id: u32,
    pub title: String,
    pub description: String,
    pub target_subsystem: String,
    pub affected_components: Vec<String>,
}

/// Dynamic ACPI SLIT Table PR Deployer
pub struct AcpiSlitNumaTopologyPrDeployer {
    pub current_slit: Option<ParsedSlit>,
    pub parsed_localities_count: u64,
    pub max_distance: u8,
    pub min_remote_distance: u8,
}

impl Default for AcpiSlitNumaTopologyPrDeployer {
    fn default() -> Self {
        Self::new()
    }
}

impl AcpiSlitNumaTopologyPrDeployer {
    pub fn new() -> Self {
        Self {
            current_slit: None,
            parsed_localities_count: 0,
            max_distance: 10,
            min_remote_distance: 255,
        }
    }

    pub fn generate_pr_spec(&self) -> AcpiSlitPrSubmissionSpec {
        AcpiSlitPrSubmissionSpec {
            pr_id: 209,
            title: String::from("PR #209: Physical ACPI SLIT Table Parsing for Real NUMA Distance Matrix"),
            description: String::from(
                "Replaces hardcoded NUMA distance assumptions with ACPI 6.5 SLIT table binary parser, populating N x N locality matrices across multi-socket CPUs.",
            ),
            target_subsystem: String::from("Power/ACPI/KernelScheduler"),
            affected_components: vec![
                String::from("src/power/acpi_table_parser.rs"),
                String::from("src/kernel/numa_scheduler.rs"),
                String::from("src/power/sovereign_acpi_slit_pr_engine.rs"),
            ],
        }
    }

    pub fn parse_and_load_slit_table(&mut self, table_bytes: &[u8]) -> Result<u64, &'static str> {
        let slit = ParsedSlit::parse_from_bytes(table_bytes)
            .ok_or("Failed to parse physical ACPI SLIT table")?;

        let localities = slit.number_of_localities;
        self.parsed_localities_count = localities;

        let mut max_dist = 10u8;
        let mut min_remote = 255u8;

        for row in 0..localities as usize {
            for col in 0..localities as usize {
                let dist = slit.matrix[row][col];
                if dist > max_dist {
                    max_dist = dist;
                }
                if row != col && dist < min_remote {
                    min_remote = dist;
                }
            }
        }

        self.max_distance = max_dist;
        self.min_remote_distance = if min_remote == 255 { 20 } else { min_remote };
        self.current_slit = Some(slit);

        Ok(localities)
    }

    pub fn get_locality_distance(&self, from: usize, to: usize) -> u32 {
        if let Some(ref slit) = self.current_slit {
            if from < slit.matrix.len() && to < slit.matrix[from].len() {
                return slit.matrix[from][to] as u32;
            }
        }
        if from == to {
            10
        } else {
            20
        }
    }

    pub fn verify_slit_readiness(&self) -> bool {
        if let Some(ref slit) = self.current_slit {
            slit.number_of_localities > 0 && !slit.matrix.is_empty()
        } else {
            false
        }
    }
}

/// Master PR Suite for ACPI SLIT NUMA Distance Matrix Deployment
pub struct SovereignAcpiSlitPrMasterSuite {
    pub deployer: AcpiSlitNumaTopologyPrDeployer,
}

impl Default for SovereignAcpiSlitPrMasterSuite {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignAcpiSlitPrMasterSuite {
    pub fn new() -> Self {
        Self {
            deployer: AcpiSlitNumaTopologyPrDeployer::new(),
        }
    }

    /// Construct a synthetic ACPI 6.5 SLIT table binary for a 2-node socket
    pub fn build_synthetic_slit_table(
        node_count: usize,
        local_dist: u8,
        remote_dist: u8,
    ) -> Vec<u8> {
        let mut bytes = vec![0u8; 44 + (node_count * node_count)];
        bytes[0..4].copy_from_slice(SLIT_SIGNATURE);
        let total_len = bytes.len() as u32;
        bytes[4..8].copy_from_slice(&total_len.to_le_bytes());

        let count_u64 = node_count as u64;
        bytes[36..44].copy_from_slice(&count_u64.to_le_bytes());

        let matrix_start = 44usize;
        for row in 0..node_count {
            for col in 0..node_count {
                let cell_idx = matrix_start + (row * node_count) + col;
                if row == col {
                    bytes[cell_idx] = local_dist;
                } else {
                    bytes[cell_idx] = remote_dist;
                }
            }
        }

        bytes
    }

    pub fn run_full_verification(&mut self) -> bool {
        let slit_bytes = Self::build_synthetic_slit_table(2, 10, 21);
        let load_res = self.deployer.parse_and_load_slit_table(&slit_bytes);
        if load_res.is_err() {
            return false;
        }

        if self.deployer.get_locality_distance(0, 0) != 10 {
            return false;
        }

        if self.deployer.get_locality_distance(0, 1) != 21 {
            return false;
        }

        self.deployer.verify_slit_readiness()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_slit_table_parsing_and_pr_spec() {
        let mut deployer = AcpiSlitNumaTopologyPrDeployer::new();
        let pr_spec = deployer.generate_pr_spec();

        assert_eq!(pr_spec.pr_id, 209);
        assert!(pr_spec.title.contains("ACPI SLIT Table Parsing"));

        let slit_bytes = SovereignAcpiSlitPrMasterSuite::build_synthetic_slit_table(4, 10, 16);
        let count = deployer
            .parse_and_load_slit_table(&slit_bytes)
            .expect("Parsing SLIT table should succeed");

        assert_eq!(count, 4);
        assert_eq!(deployer.get_locality_distance(0, 0), 10);
        assert_eq!(deployer.get_locality_distance(0, 1), 16);
        assert_eq!(deployer.get_locality_distance(2, 3), 16);
        assert!(deployer.verify_slit_readiness());
    }

    #[test]
    fn test_acpi_slit_master_suite() {
        let mut suite = SovereignAcpiSlitPrMasterSuite::new();
        assert!(suite.run_full_verification());
    }
}
