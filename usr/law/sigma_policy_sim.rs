// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/law/sigma_policy_sim.rs — Sigma Policy Simulation
//
// Implements sandbox for testing how new laws affect workplaces,
// helping law students and professionals understand policy impact.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Policy Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolicyDomain {
    Labour,
    Environment,
    Taxation,
    Corporate,
    DataProtection,
}

#[derive(Debug, Clone)]
pub struct WorkplaceParameter {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PolicyScenario {
    pub id: String,
    pub name: String,
    pub domain: PolicyDomain,
    pub description: String,
    pub parameters: Vec<WorkplaceParameter>,
    pub outcomes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub scenario_id: String,
    pub before_state: HashMap<String, f64>,
    pub after_state: HashMap<String, f64>,
    pub impact_analysis: Vec<String>,
    pub recommendations: Vec<String>,
}

// ─── Policy Simulation Engine ─────────────────────────────────────────────────

pub struct PolicySimulator {
    pub scenarios: HashMap<String, PolicyScenario>,
    pub results: Vec<SimulationResult>,
    pub current_parameters: HashMap<String, f64>,
}

impl PolicySimulator {
    pub fn new() -> Self {
        let mut simulator = PolicySimulator {
            scenarios: HashMap::new(),
            results: Vec::new(),
            current_parameters: HashMap::new(),
        };
        
        simulator.init_scenarios();
        simulator.init_default_parameters();
        simulator
    }

    /// Initialize policy scenarios
    fn init_scenarios(&mut self) {
        // Minimum Wage Increase Scenario
        self.scenarios.insert("scen_001".to_string(), PolicyScenario {
            id: "scen_001".to_string(),
            name: "Minimum Wage Increase".to_string(),
            domain: PolicyDomain::Labour,
            description: "Simulate impact of increasing minimum wage by 20%".to_string(),
            parameters: vec![
                WorkplaceParameter {
                    name: "Current Minimum Wage".to_string(),
                    value: 15000.0,
                    unit: "₹/month".to_string(),
                    description: "Current minimum wage rate".to_string(),
                },
                WorkplaceParameter {
                    name: "Proposed Increase".to_string(),
                    value: 20.0,
                    unit: "%".to_string(),
                    description: "Percentage increase in minimum wage".to_string(),
                },
                WorkplaceParameter {
                    name: "Number of Employees".to_string(),
                    value: 100.0,
                    unit: "count".to_string(),
                    description: "Total workforce size".to_string(),
                },
            ],
            outcomes: vec![
                "Increased labor costs".to_string(),
                "Potential reduction in hiring".to_string(),
                "Improved employee satisfaction".to_string(),
                "Increased consumer spending power".to_string(),
            ],
        });

        // Work From Home Policy Scenario
        self.scenarios.insert("scen_002".to_string(), PolicyScenario {
            id: "scen_002".to_string(),
            name: "Work From Home Mandate".to_string(),
            domain: PolicyDomain::Labour,
            description: "Simulate impact of mandatory 3-day work from home policy".to_string(),
            parameters: vec![
                WorkplaceParameter {
                    name: "Office Space Cost".to_string(),
                    value: 500000.0,
                    unit: "₹/month".to_string(),
                    description: "Current office space rental cost".to_string(),
                },
                WorkplaceParameter {
                    name: "WFH Days per Week".to_string(),
                    value: 3.0,
                    unit: "days".to_string(),
                    description: "Number of work from home days".to_string(),
                },
                WorkplaceParameter {
                    name: "Employee Productivity".to_string(),
                    value: 95.0,
                    unit: "%".to_string(),
                    description: "Current productivity level".to_string(),
                },
            ],
            outcomes: vec![
                "Reduced office costs".to_string(),
                "Potential productivity changes".to_string(),
                "Infrastructure requirements for remote work".to_string(),
                "Employee work-life balance improvement".to_string(),
            ],
        });

        // Environmental Tax Scenario
        self.scenarios.insert("scen_003".to_string(), PolicyScenario {
            id: "scen_003".to_string(),
            name: "Carbon Tax Implementation".to_string(),
            domain: PolicyDomain::Environment,
            description: "Simulate impact of carbon tax on manufacturing operations".to_string(),
            parameters: vec![
                WorkplaceParameter {
                    name: "Carbon Emissions".to_string(),
                    value: 1000.0,
                    unit: "tons/year".to_string(),
                    description: "Annual carbon emissions".to_string(),
                },
                WorkplaceParameter {
                    name: "Carbon Tax Rate".to_string(),
                    value: 2000.0,
                    unit: "₹/ton".to_string(),
                    description: "Tax rate per ton of carbon".to_string(),
                },
                WorkplaceParameter {
                    name: "Production Volume".to_string(),
                    value: 10000.0,
                    unit: "units/year".to_string(),
                    description: "Annual production volume".to_string(),
                },
            ],
            outcomes: vec![
                "Increased operational costs".to_string(),
                "Incentive for green technology adoption".to_string(),
                "Potential price increase for products".to_string(),
                "Reduced environmental impact".to_string(),
            ],
        });

        // Data Protection Law Scenario
        self.scenarios.insert("scen_004".to_string(), PolicyScenario {
            id: "scen_004".to_string(),
            name: "Data Protection Compliance".to_string(),
            domain: PolicyDomain::DataProtection,
            description: "Simulate impact of new data protection law on business operations".to_string(),
            parameters: vec![
                WorkplaceParameter {
                    name: "Customer Records".to_string(),
                    value: 50000.0,
                    unit: "records".to_string(),
                    description: "Number of customer records stored".to_string(),
                },
                WorkplaceParameter {
                    name: "Compliance Cost per Record".to_string(),
                    value: 100.0,
                    unit: "₹/record".to_string(),
                    description: "Cost to comply per record".to_string(),
                },
                WorkplaceParameter {
                    name: "Non-Compliance Penalty".to_string(),
                    value: 1000000.0,
                    unit: "₹".to_string(),
                    description: "Penalty for non-compliance".to_string(),
                },
            ],
            outcomes: vec![
                "Increased compliance costs".to_string(),
                "Enhanced customer trust".to_string(),
                "Required infrastructure upgrades".to_string(),
                "Potential data breach liability reduction".to_string(),
            ],
        });
    }

    /// Initialize default workplace parameters
    fn init_default_parameters(&mut self) {
        self.current_parameters.insert("labor_cost".to_string(), 1500000.0);
        self.current_parameters.insert("productivity".to_string(), 95.0);
        self.current_parameters.insert("operational_cost".to_string(), 5000000.0);
        self.current_parameters.insert("employee_satisfaction".to_string(), 75.0);
    }

    /// Get scenario by ID
    pub fn get_scenario(&self, id: &str) -> Option<&PolicyScenario> {
        self.scenarios.get(id)
    }

    /// Get all scenarios
    pub fn get_all_scenarios(&self) -> Vec<&PolicyScenario> {
        self.scenarios.values().collect()
    }

    /// Get scenarios by domain
    pub fn get_scenarios_by_domain(&self, domain: PolicyDomain) -> Vec<&PolicyScenario> {
        self.scenarios.values()
            .filter(|s| s.domain == domain)
            .collect()
    }

    /// Run simulation for scenario
    pub fn run_simulation(&mut self, scenario_id: &str) -> Result<SimulationResult, String> {
        if let Some(scenario) = self.scenarios.get(scenario_id) {
            let before_state = self.current_parameters.clone();
            let mut after_state = before_state.clone();
            let mut impact_analysis = Vec::new();
            let mut recommendations = Vec::new();

            // Simulate based on scenario type
            match scenario.domain {
                PolicyDomain::Labour => {
                    if scenario.name.contains("Minimum Wage") {
                        let wage_increase = scenario.parameters.iter()
                            .find(|p| p.name.contains("Increase"))
                            .map(|p| p.value / 100.0)
                            .unwrap_or(0.2);
                        
                        let current_labor_cost = before_state.get("labor_cost").copied().unwrap_or(0.0);
                        after_state.insert("labor_cost".to_string(), current_labor_cost * (1.0 + wage_increase));
                        
                        impact_analysis.push(format!("Labor cost increased by {:.0}%", wage_increase * 100.0));
                        impact_analysis.push("Potential reduction in hiring by 5-10%".to_string());
                        
                        recommendations.push("Consider automation to offset labor costs".to_string());
                        recommendations.push("Gradual implementation to allow adjustment".to_string());
                    } else if scenario.name.contains("Work From Home") {
                        let wfh_days = scenario.parameters.iter()
                            .find(|p| p.name.contains("WFH"))
                            .map(|p| p.value)
                            .unwrap_or(3.0);
                        
                        let office_cost = scenario.parameters.iter()
                            .find(|p| p.name.contains("Office"))
                            .map(|p| p.value)
                            .unwrap_or(0.0);
                        
                        let savings = (wfh_days / 5.0) * office_cost;
                        let current_op_cost = before_state.get("operational_cost").copied().unwrap_or(0.0);
                        after_state.insert("operational_cost".to_string(), current_op_cost - savings);
                        
                        impact_analysis.push(format!("Office cost savings: ₹{:.0}", savings));
                        impact_analysis.push("Productivity may vary by 5-10%".to_string());
                        
                        recommendations.push("Invest in collaboration tools".to_string());
                        recommendations.push("Implement clear WFH policies".to_string());
                    }
                }
                PolicyDomain::Environment => {
                    if scenario.name.contains("Carbon") {
                        let emissions = scenario.parameters.iter()
                            .find(|p| p.name.contains("Emissions"))
                            .map(|p| p.value)
                            .unwrap_or(0.0);
                        
                        let tax_rate = scenario.parameters.iter()
                            .find(|p| p.name.contains("Tax"))
                            .map(|p| p.value)
                            .unwrap_or(0.0);
                        
                        let tax_cost = emissions * tax_rate;
                        let current_op_cost = before_state.get("operational_cost").copied().unwrap_or(0.0);
                        after_state.insert("operational_cost".to_string(), current_op_cost + tax_cost);
                        
                        impact_analysis.push(format!("Annual carbon tax: ₹{:.0}", tax_cost));
                        impact_analysis.push("Incentive for green technology adoption".to_string());
                        
                        recommendations.push("Invest in energy-efficient equipment".to_string());
                        recommendations.push("Explore renewable energy options".to_string());
                    }
                }
                PolicyDomain::DataProtection => {
                    if scenario.name.contains("Data Protection") {
                        let records = scenario.parameters.iter()
                            .find(|p| p.name.contains("Records"))
                            .map(|p| p.value)
                            .unwrap_or(0.0);
                        
                        let compliance_cost = scenario.parameters.iter()
                            .find(|p| p.name.contains("Compliance"))
                            .map(|p| p.value)
                            .unwrap_or(0.0);
                        
                        let total_compliance_cost = records * compliance_cost;
                        let current_op_cost = before_state.get("operational_cost").copied().unwrap_or(0.0);
                        after_state.insert("operational_cost".to_string(), current_op_cost + total_compliance_cost);
                        
                        impact_analysis.push(format!("Compliance cost: ₹{:.0}", total_compliance_cost));
                        impact_analysis.push("Enhanced customer trust and reputation".to_string());
                        
                        recommendations.push("Implement data governance framework".to_string());
                        recommendations.push("Train staff on data protection".to_string());
                    }
                }
                _ => {
                    impact_analysis.push("Generic policy impact simulation".to_string());
                }
            }

            let result = SimulationResult {
                scenario_id: scenario_id.to_string(),
                before_state,
                after_state,
                impact_analysis,
                recommendations,
            };
            
            self.results.push(result.clone());
            Ok(result)
        } else {
            Err("Scenario not found".to_string())
        }
    }

    /// Get simulation results
    pub fn get_results(&self) -> &[SimulationResult] {
        &self.results
    }

    /// Get result by scenario ID
    pub fn get_result_by_scenario(&self, scenario_id: &str) -> Option<&SimulationResult> {
        self.results.iter().find(|r| r.scenario_id == scenario_id)
    }

    /// Reset parameters to defaults
    pub fn reset_parameters(&mut self) {
        self.init_default_parameters();
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut simulator = PolicySimulator::new();
    
    println!("Sigma Policy Simulation v0.1 - Workplace Law Testing");
    
    loop {
        println!("\n--- Available Scenarios ---");
        for scenario in simulator.get_all_scenarios() {
            let domain_str = match scenario.domain {
                PolicyDomain::Labour => "Labour",
                PolicyDomain::Environment => "Environment",
                PolicyDomain::Taxation => "Taxation",
                PolicyDomain::Corporate => "Corporate",
                PolicyDomain::DataProtection => "Data Protection",
            };
            println!("{} - {} ({})", scenario.id, scenario.name, domain_str);
        }
        
        println!("\n--- Current Parameters ---");
        for (key, value) in &simulator.current_parameters {
            println!("{}: {:.2}", key, value);
        }
        
        println!("\nCommands: scenario <id>, run <id>, results, result <id>, reset, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "scenario" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(scenario) = simulator.get_scenario(arg) {
                        println!("--- Scenario Details ---");
                        println!("Name: {}", scenario.name);
                        println!("Domain: {:?}", scenario.domain);
                        println!("Description: {}", scenario.description);
                        println!("\nParameters:");
                        for param in &scenario.parameters {
                            println!("  {}: {:.1} {} - {}", param.name, param.value, param.unit, param.description);
                        }
                        println!("\nExpected Outcomes:");
                        for outcome in &scenario.outcomes {
                            println!("- {}", outcome);
                        }
                    }
                }
            }
            "run" => {
                if let Some(arg) = parts.get(1) {
                    match simulator.run_simulation(arg) {
                        Ok(result) => {
                            println!("--- Simulation Results ---");
                            println!("\nBefore State:");
                            for (key, value) in &result.before_state {
                                println!("  {}: {:.2}", key, value);
                            }
                            println!("\nAfter State:");
                            for (key, value) in &result.after_state {
                                let before = result.before_state.get(key).copied().unwrap_or(0.0);
                                let change = value - before;
                                let change_str = if change > 0.0 { format!("(+{:.2})", change) } else { format!("({:.2})", change) };
                                println!("  {}: {:.2} {}", key, value, change_str);
                            }
                            println!("\nImpact Analysis:");
                            for analysis in &result.impact_analysis {
                                println!("- {}", analysis);
                            }
                            println!("\nRecommendations:");
                            for rec in &result.recommendations {
                                println!("- {}", rec);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "results" => {
                println!("--- All Simulation Results ---");
                for result in simulator.get_results() {
                    if let Some(scenario) = simulator.get_scenario(&result.scenario_id) {
                        println!("{} - {}", scenario.name, result.scenario_id);
                    }
                }
            }
            "result" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(result) = simulator.get_result_by_scenario(arg) {
                        println!("--- Simulation Result ---");
                        println!("\nImpact Analysis:");
                        for analysis in &result.impact_analysis {
                            println!("- {}", analysis);
                        }
                        println!("\nRecommendations:");
                        for rec in &result.recommendations {
                            println!("- {}", rec);
                        }
                    }
                }
            }
            "reset" => {
                simulator.reset_parameters();
                println!("Parameters reset to defaults");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
