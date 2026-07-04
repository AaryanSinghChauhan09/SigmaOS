// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_proof.rs — Sigma Math Proof Assistant
//
// Implements step-by-step solver for CBSE syllabus mathematics problems,
// including algebra, geometry, trigonometry, and calculus proofs.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Math Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathDomain {
    Algebra,
    Geometry,
    Trigonometry,
    Calculus,
    Statistics,
}

#[derive(Debug, Clone)]
pub struct ProofStep {
    pub step_number: u32,
    pub statement: String,
    pub justification: String,
    pub formula: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MathProblem {
    pub id: String,
    pub title: String,
    pub domain: MathDomain,
    pub class: String,  // CBSE class (6-12)
    pub chapter: String,
    pub problem_statement: String,
    pub given: Vec<String>,
    pub to_prove: String,
    pub difficulty: String,
}

#[derive(Debug, Clone)]
pub struct ProofSolution {
    pub problem_id: String,
    pub steps: Vec<ProofStep>,
    pub final_answer: String,
    pub alternative_methods: Vec<String>,
}

// ─── Math Proof Assistant ────────────────────────────────────────────────────

pub struct ProofAssistant {
    pub problems: HashMap<String, MathProblem>,
    pub solutions: HashMap<String, ProofSolution>,
}

impl ProofAssistant {
    pub fn new() -> Self {
        let mut assistant = ProofAssistant {
            problems: HashMap::new(),
            solutions: HashMap::new(),
        };
        
        assistant.init_algebra_problems();
        assistant.init_geometry_problems();
        assistant.init_trigonometry_problems();
        assistant.init_calculus_problems();
        assistant
    }

    /// Initialize algebra problems
    fn init_algebra_problems(&mut self) {
        // Quadratic Equation Proof
        self.problems.insert("alg_001".to_string(), MathProblem {
            id: "alg_001".to_string(),
            title: "Quadratic Formula Derivation".to_string(),
            domain: MathDomain::Algebra,
            class: "10".to_string(),
            chapter: "Quadratic Equations".to_string(),
            problem_statement: "Derive the quadratic formula for solving ax² + bx + c = 0".to_string(),
            given: vec![
                "Quadratic equation: ax² + bx + c = 0".to_string(),
                "a ≠ 0".to_string(),
            ],
            to_prove: "x = (-b ± √(b² - 4ac)) / 2a".to_string(),
            difficulty: "Medium".to_string(),
        });

        let solution = ProofSolution {
            problem_id: "alg_001".to_string(),
            steps: vec![
                ProofStep {
                    step_number: 1,
                    statement: "Start with the quadratic equation".to_string(),
                    justification: "Given".to_string(),
                    formula: Some("ax² + bx + c = 0".to_string()),
                },
                ProofStep {
                    step_number: 2,
                    statement: "Divide both sides by a".to_string(),
                    justification: "Since a ≠ 0".to_string(),
                    formula: Some("x² + (b/a)x + (c/a) = 0".to_string()),
                },
                ProofStep {
                    step_number: 3,
                    statement: "Move constant term to RHS".to_string(),
                    justification: "Algebraic manipulation".to_string(),
                    formula: Some("x² + (b/a)x = -c/a".to_string()),
                },
                ProofStep {
                    step_number: 4,
                    statement: "Complete the square on LHS".to_string(),
                    justification: "Add (b/2a)² to both sides".to_string(),
                    formula: Some("x² + (b/a)x + (b/2a)² = -c/a + (b/2a)²".to_string()),
                },
                ProofStep {
                    step_number: 5,
                    statement: "Simplify the equation".to_string(),
                    justification: "Perfect square formula".to_string(),
                    formula: Some("(x + b/2a)² = (b² - 4ac) / 4a²".to_string()),
                },
                ProofStep {
                    step_number: 6,
                    statement: "Take square root of both sides".to_string(),
                    justification: "Square root property".to_string(),
                    formula: Some("x + b/2a = ±√(b² - 4ac) / 2a".to_string()),
                },
                ProofStep {
                    step_number: 7,
                    statement: "Solve for x".to_string(),
                    justification: "Isolate x".to_string(),
                    formula: Some("x = (-b ± √(b² - 4ac)) / 2a".to_string()),
                },
            ],
            final_answer: "x = (-b ± √(b² - 4ac)) / 2a".to_string(),
            alternative_methods: vec![
                "Factorization method (when possible)".to_string(),
                "Graphical method".to_string(),
            ],
        };
        self.solutions.insert("alg_001".to_string(), solution);
   

        // Arithmetic Progression Sum Formula
        self.problems.insert("alg_002".to_string(), MathProblem {
            id: "alg_002".to_string(),
            title: "Sum of n Terms of AP".to_string(),
            domain: MathDomain::Algebra,
            class: "10".to_string(),
            chapter: "Arithmetic Progressions".to_string(),
            problem_statement: "Derive the formula for sum of first n terms of an AP".to_string(),
            given: vec![
                "AP: a, a+d, a+2d, ..., a+(n-1)d".to_string(),
                "First term = a, common difference = d".to_string(),
            ],
            to_prove: "Sn = n/2 [2a + (n-1)d]".to_string(),
            difficulty: "Easy".to_string(),
        });

        let solution = ProofSolution {
            problem_id: "alg_002".to_string(),
            steps: vec![
                ProofStep {
                    step_number: 1,
                    statement: "Write the sum in forward order".to_string(),
                    justification: "Definition of sum".to_string(),
                    formula: Some("Sn = a + (a+d) + (a+2d) + ... + [a+(n-1)d]".to_string()),
                },
                ProofStep {
                    step_number: 2,
                    statement: "Write the sum in reverse order".to_string(),
                    justification: "Reverse the terms".to_string(),
                    formula: Some("Sn = [a+(n-1)d] + [a+(n-2)d] + ... + a".to_string()),
                },
                ProofStep {
                    step_number: 3,
                    statement: "Add both equations".to_string(),
                    justification: "Term-wise addition".to_string(),
                    formula: Some("2Sn = [2a+(n-1)d] + [2a+(n-1)d] + ... + [2a+(n-1)d]".to_string()),
                },
                ProofStep {
                    step_number: 4,
                    statement: "Simplify the sum".to_string(),
                    justification: "n identical terms".to_string(),
                    formula: Some("2Sn = n[2a+(n-1)d]".to_string()),
                },
                ProofStep {
                    step_number: 5,
                    statement: "Solve for Sn".to_string(),
                    justification: "Divide by 2".to_string(),
                    formula: Some("Sn = n/2 [2a+(n-1)d]".to_string()),
                },
            ],
            final_answer: "Sn = n/2 [2a + (n-1)d]".to_string(),
            alternative_methods: vec![
                "Using last term: Sn = n/2 (a + l)".to_string(),
            ],
        };
        self.solutions.insert("alg_002".to_string(), solution);
    }

    /// Initialize geometry problems
    fn init_geometry_problems(&mut self) {
        // Pythagoras Theorem Proof
        self.problems.insert("geo_001".to_string(), MathProblem {
            id: "geo_001".to_string(),
            title: "Pythagoras Theorem".to_string(),
            domain: MathDomain::Geometry,
            class: "10".to_string(),
            chapter: "Triangles".to_string(),
            problem_statement: "Prove that in a right triangle, the square of the hypotenuse equals the sum of squares of the other two sides".to_string(),
            given: vec![
                "Right triangle ABC with right angle at B".to_string(),
                "AB = c, BC = a, AC = b (hypotenuse)".to_string(),
            ],
            to_prove: "a² + c² = b²".to_string(),
            difficulty: "Medium".to_string(),
        });

        let solution = ProofSolution {
            problem_id: "geo_001".to_string(),
            steps: vec![
                ProofStep {
                    step_number: 1,
                    statement: "Construct squares on each side".to_string(),
                    justification: "Construction for proof".to_string(),
                    formula: Some("Square on AB, BC, and AC".to_string()),
                },
                ProofStep {
                    step_number: 2,
                    statement: "Draw perpendicular from B to AC meeting at D".to_string(),
                    justification: "Construction".to_string(),
                    formula: Some("BD ⟂ AC".to_string()),
                },
                ProofStep {
                    step_number: 3,
                    statement: "Prove triangles are congruent".to_string(),
                    justification: "By SAS congruence".to_string(),
                    formula: Some("ΔABD ≅ ΔFBC".to_string()),
                },
                ProofStep {
                    step_number: 4,
                    statement: "Area relationships".to_string(),
                    justification: "Using congruence".to_string(),
                    formula: Some("Area(ABFG) = 2 × Area(ΔFBC)".to_string()),
                },
                ProofStep {
                    step_number: 5,
                    statement: "Compare areas of squares".to_string(),
                    justification: "Area equivalence".to_string(),
                    formula: Some("a² + c² = b²".to_string()),
                },
            ],
            final_answer: "a² + c² = b²".to_string(),
            alternative_methods: vec![
                "Similar triangles method".to_string(),
                "Algebraic proof using coordinates".to_string(),
            ],
        };
        self.solutions.insert("geo_001".to_string(), solution);
    }

    /// Initialize trigonometry problems
    fn init_trigonometry_problems(&mut self) {
        // Trigonometric Identities
        self.problems.insert("trig_001".to_string(), MathProblem {
            id: "trig_001".to_string(),
            title: "sin²θ + cos²θ = 1".to_string(),
            domain: MathDomain::Trigonometry,
            class: "10".to_string(),
            chapter: "Trigonometric Identities".to_string(),
            problem_statement: "Prove the fundamental trigonometric identity".to_string(),
            given: vec![
                "Right triangle with angle θ".to_string(),
                "sinθ = opposite/hypotenuse".to_string(),
                "cosθ = adjacent/hypotenuse".to_string(),
            ],
            to_prove: "sin²θ + cos²θ = 1".to_string(),
            difficulty: "Easy".to_string(),
        });

        let solution = ProofSolution {
            problem_id: "trig_001".to_string(),
            steps: vec![
                ProofStep {
                    step_number: 1,
                    statement: "Consider a right triangle ABC".to_string(),
                    justification: "Given".to_string(),
                    formula: Some("∠B = 90°, ∠A = θ".to_string()),
                },
                ProofStep {
                    step_number: 2,
                    statement: "Apply Pythagoras theorem".to_string(),
                    justification: "For right triangle".to_string(),
                    formula: Some("AB² + BC² = AC²".to_string()),
                },
                ProofStep {
                    step_number: 3,
                    statement: "Divide by AC²".to_string(),
                    justification: "Normalize".to_string(),
                    formula: Some("(AB/AC)² + (BC/AC)² = 1".to_string()),
                },
                ProofStep {
                    step_number: 4,
                    statement: "Substitute trigonometric ratios".to_string(),
                    justification: "Definition of sin and cos".to_string(),
                    formula: Some("cos²θ + sin²θ = 1".to_string()),
                },
                ProofStep {
                    step_number: 5,
                    statement: "Rearrange".to_string(),
                    justification: "Final form".to_string(),
                    formula: Some("sin²θ + cos²θ = 1".to_string()),
                },
            ],
            final_answer: "sin²θ + cos²θ = 1".to_string(),
            alternative_methods: vec![
                "Using unit circle".to_string(),
                "Using Euler's formula".to_string(),
            ],
        };
        self.solutions.insert("trig_001".to_string(), solution);
    }

    /// Initialize calculus problems
    fn init_calculus_problems(&mut self) {
        // Derivative of xⁿ
        self.problems.insert("calc_001".to_string(), MathProblem {
            id: "calc_001".to_string(),
            title: "Derivative of xⁿ".to_string(),
            domain: MathDomain::Calculus,
            class: "12".to_string(),
            chapter: "Continuity and Differentiability".to_string(),
            problem_statement: "Prove that d/dx(xⁿ) = nxⁿ⁻¹ using first principles".to_string(),
            given: vec![
                "f(x) = xⁿ".to_string(),
                "Definition of derivative".to_string(),
            ],
            to_prove: "f'(x) = nxⁿ⁻¹".to_string(),
            difficulty: "Medium".to_string(),
        });

        let solution = ProofSolution {
            problem_id: "calc_001".to_string(),
            steps: vec![
                ProofStep {
                    step_number: 1,
                    statement: "Apply first principle definition".to_string(),
                    justification: "Definition of derivative".to_string(),
                    formula: Some("f'(x) = lim(h→0) [f(x+h) - f(x)] / h".to_string()),
                },
                ProofStep {
                    step_number: 2,
                    statement: "Substitute f(x) = xⁿ".to_string(),
                    justification: "Given function".to_string(),
                    formula: Some("f'(x) = lim(h→0) [(x+h)ⁿ - xⁿ] / h".to_string()),
                },
                ProofStep {
                    step_number: 3,
                    statement: "Apply binomial expansion".to_string(),
                    justification: "For (x+h)ⁿ".to_string(),
                    formula: Some("(x+h)ⁿ = xⁿ + nxⁿ⁻¹h + ...".to_string()),
                },
                ProofStep {
                    step_number: 4,
                    statement: "Simplify the expression".to_string(),
                    justification: "Cancel terms".to_string(),
                    formula: Some("f'(x) = lim(h→0) [nxⁿ⁻¹h + higher order terms] / h".to_string()),
                },
                ProofStep {
                    step_number: 5,
                    statement: "Take the limit".to_string(),
                    justification: "As h → 0".to_string(),
                    formula: Some("f'(x) = nxⁿ⁻¹".to_string()),
                },
            ],
            final_answer: "d/dx(xⁿ) = nxⁿ⁻¹".to_string(),
            alternative_methods: vec![
                "Using logarithmic differentiation".to_string(),
                "Using power rule (shortcut)".to_string(),
            ],
        };
        self.solutions.insert("calc_001".to_string(), solution);
    }

    /// Get problem by ID
    pub fn get_problem(&self, id: &str) -> Option<&MathProblem> {
        self.problems.get(id)
    }

    /// Get solution by problem ID
    pub fn get_solution(&self, problem_id: &str) -> Option<&ProofSolution> {
        self.solutions.get(problem_id)
    }

    /// Get problems by domain
    pub fn get_problems_by_domain(&self, domain: MathDomain) -> Vec<&MathProblem> {
        self.problems.values()
            .filter(|p| p.domain == domain)
            .collect()
    }

    /// Get problems by class
    pub fn get_problems_by_class(&self, class: &str) -> Vec<&MathProblem> {
        self.problems.values()
            .filter(|p| p.class == class)
            .collect()
    }

    /// Get all problems
    pub fn get_all_problems(&self) -> Vec<&MathProblem> {
        self.problems.values().collect()
    }

    /// Search problems by keyword
    pub fn search(&self, keyword: &str) -> Vec<&MathProblem> {
        self.problems.values()
            .filter(|p| {
                p.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                p.chapter.to_lowercase().contains(&keyword.to_lowercase()) ||
                p.problem_statement.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let assistant = ProofAssistant::new();
    
    println!("Sigma Math Proof Assistant v0.1 - Step-by-Step CBSE Solutions");
    
    loop {
        println!("\n--- Available Problems ---");
        for problem in assistant.get_all_problems() {
            let domain_str = match problem.domain {
                MathDomain::Algebra => "Algebra",
                MathDomain::Geometry => "Geometry",
                MathDomain::Trigonometry => "Trigonometry",
                MathDomain::Calculus => "Calculus",
                MathDomain::Statistics => "Statistics",
            };
            println!("{} - {} (Class {}, {}) - {}", problem.id, problem.title, problem.class, domain_str, problem.difficulty);
        }
        
        println!("\nCommands: problem <id>, solve <id>, domain <type>, class <n>, search <keyword>, quit");
        println!("Domains: algebra, geometry, trigonometry, calculus, statistics");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "problem" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(problem) = assistant.get_problem(arg) {
                        println!("--- Problem Details ---");
                        println!("Title: {}", problem.title);
                        println!("Class: {}", problem.class);
                        println!("Chapter: {}", problem.chapter);
                        println!("Difficulty: {}", problem.difficulty);
                        println!("\nProblem Statement: {}", problem.problem_statement);
                        println!("\nGiven:");
                        for given in &problem.given {
                            println!("- {}", given);
                        }
                        println!("\nTo Prove: {}", problem.to_prove);
                    }
                }
            }
            "solve" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(solution) = assistant.get_solution(arg) {
                        println!("--- Step-by-Step Solution ---");
                        for step in &solution.steps {
                            println!("\nStep {}: {}", step.step_number, step.statement);
                            println!("Justification: {}", step.justification);
                            if let Some(formula) = &step.formula {
                                println!("Formula: {}", formula);
                            }
                        }
                        println!("\n--- Final Answer ---");
                        println!("{}", solution.final_answer);
                        println!("\nAlternative Methods:");
                        for method in &solution.alternative_methods {
                            println!("- {}", method);
                        }
                    }
                }
            }
            "domain" => {
                if let Some(arg) = parts.get(1) {
                    let domain = match *arg {
                        "algebra" => MathDomain::Algebra,
                        "geometry" => MathDomain::Geometry,
                        "trigonometry" => MathDomain::Trigonometry,
                        "calculus" => MathDomain::Calculus,
                        "statistics" => MathDomain::Statistics,
                        _ => {
                            println!("Unknown domain");
                            continue;
                        }
                    };
                    println!("--- {} Problems ---", arg);
                    for problem in assistant.get_problems_by_domain(domain) {
                        println!("{} - {}", problem.id, problem.title);
                    }
                }
            }
            "class" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Class {} Problems ---", arg);
                    for problem in assistant.get_problems_by_class(arg) {
                        println!("{} - {}", problem.id, problem.title);
                    }
                }
            }
            "search" => {
                if parts.len() >= 2 {
                    let keyword = parts[1..].join(" ");
                    let results = assistant.search(&keyword);
                    println!("--- Search Results for '{}' ---", keyword);
                    for problem in results {
                        println!("{} - {} (Class {})", problem.id, problem.title, problem.class);
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
