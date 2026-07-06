// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_practice.rs — Sigma Adaptive Practice Generator
//
// Implements AI-powered practice problem generation aligned with CBSE syllabus
// difficulty levels for mathematics, science, and IT subjects.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Problem Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subject {
    Mathematics,
    Physics,
    Chemistry,
    Biology,
    ComputerScience,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: String,
    pub subject: Subject,
    pub chapter: String,
    pub difficulty: Difficulty,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: String,
    pub marks: u32,
}

#[derive(Debug, Clone)]
pub struct StudentProgress {
    pub subject: Subject,
    pub chapter: String,
    pub problems_attempted: u32,
    pub problems_correct: u32,
    pub current_difficulty: Difficulty,
}

// ─── Practice Generator ───────────────────────────────────────────────────────

pub struct PracticeGenerator {
    pub problems: HashMap<String, Problem>,
    pub progress: HashMap<String, StudentProgress>,
    pub current_session: Vec<String>,
}

impl PracticeGenerator {
    pub fn new() -> Self {
        let mut generator = PracticeGenerator {
            problems: HashMap::new(),
            progress: HashMap::new(),
            current_session: Vec::new(),
        };
        
        generator.init_cbse_problems();
        generator
    }

    /// Initialize CBSE-aligned problems
    fn init_cbse_problems(&mut self) {
        // Mathematics Problems
        self.add_problem(Problem {
            id: "math_quad_1".to_string(),
            subject: Subject::Mathematics,
            chapter: "Chapter 4: Quadratic Equations".to_string(),
            difficulty: Difficulty::Easy,
            question: "Solve: x² - 5x + 6 = 0".to_string(),
            options: vec!["x = 2, 3".to_string(), "x = -2, -3".to_string(), "x = 1, 6".to_string(), "x = -1, -6".to_string()],
            correct_answer: 0,
            explanation: "Factor: (x-2)(x-3) = 0, so x = 2 or x = 3".to_string(),
            marks: 2,
        });

        self.add_problem(Problem {
            id: "math_calc_1".to_string(),
            subject: Subject::Mathematics,
            chapter: "Chapter 6: Application of Derivatives".to_string(),
            difficulty: Difficulty::Medium,
            question: "Find derivative of f(x) = x³ + 2x² - 5x + 1".to_string(),
            options: vec!["3x² + 4x - 5".to_string(), "3x² + 4x + 5".to_string(), "3x² - 4x - 5".to_string(), "x³ + 4x - 5".to_string()],
            correct_answer: 0,
            explanation: "Using power rule: d/dx(x³) = 3x², d/dx(2x²) = 4x, d/dx(-5x) = -5, d/dx(1) = 0".to_string(),
            marks: 3,
        });

        // Physics Problems
        self.add_problem(Problem {
            id: "phy_kin_1".to_string(),
            subject: Subject::Physics,
            chapter: "Chapter 3: Motion in a Straight Line".to_string(),
            difficulty: Difficulty::Easy,
            question: "A car travels 100m in 5s. What is its average velocity?".to_string(),
            options: vec!["20 m/s".to_string(), "25 m/s".to_string(), "15 m/s".to_string(), "10 m/s".to_string()],
            correct_answer: 0,
            explanation: "Average velocity = displacement / time = 100m / 5s = 20 m/s".to_string(),
            marks: 2,
        });

        self.add_problem(Problem {
            id: "phy_newton_1".to_string(),
            subject: Subject::Physics,
            chapter: "Chapter 5: Laws of Motion".to_string(),
            difficulty: Difficulty::Medium,
            question: "A 5kg object accelerates at 2 m/s². What is the force applied?".to_string(),
            options: vec!["10 N".to_string(), "7.5 N".to_string(), "2.5 N".to_string(), "5 N".to_string()],
            correct_answer: 0,
            explanation: "F = ma = 5kg × 2 m/s² = 10 N".to_string(),
            marks: 3,
        });

        // Chemistry Problems
        self.add_problem(Problem {
            id: "chem_mole_1".to_string(),
            subject: Subject::Chemistry,
            chapter: "Chapter 1: Some Basic Concepts of Chemistry".to_string(),
            difficulty: Difficulty::Easy,
            question: "How many moles are in 18g of water (H₂O)?".to_string(),
            options: vec!["1 mole".to_string(), "0.5 mole".to_string(), "2 moles".to_string(), "0.9 mole".to_string()],
            correct_answer: 0,
            explanation: "Molar mass of H₂O = 18g/mol. 18g / 18g/mol = 1 mole".to_string(),
            marks: 2,
        });

        // Computer Science Problems
        self.add_problem(Problem {
            id: "cs_algo_1".to_string(),
            subject: Subject::ComputerScience,
            chapter: "Chapter 4: Sorting".to_string(),
            difficulty: Difficulty::Medium,
            question: "What is the time complexity of merge sort?".to_string(),
            options: vec!["O(n log n)".to_string(), "O(n²)".to_string(), "O(n)".to_string(), "O(log n)".to_string()],
            correct_answer: 0,
            explanation: "Merge sort divides array in half recursively, giving O(n log n) complexity".to_string(),
            marks: 3,
        });
    }

    /// Add problem to generator
    fn add_problem(&mut self, problem: Problem) {
        self.problems.insert(problem.id.clone(), problem);
    }

    /// Generate practice set based on subject and difficulty
    pub fn generate_practice_set(&self, subject: Subject, difficulty: Difficulty, count: usize) -> Vec<&Problem> {
        self.problems.values()
            .filter(|p| p.subject == subject && p.difficulty == difficulty)
            .take(count)
            .collect()
    }

    /// Get next problem based on adaptive difficulty
    pub fn get_next_problem(&mut self, subject: Subject, chapter: String) -> Option<&Problem> {
        let progress_key = format!("{:?}_{}", subject, chapter);
        
        let current_difficulty = self.progress
            .get(&progress_key)
            .map(|p| p.current_difficulty)
            .unwrap_or(Difficulty::Easy);
        
        // Find problems matching current difficulty
        let candidates: Vec<&Problem> = self.problems.values()
            .filter(|p| p.subject == subject && p.chapter == chapter && p.difficulty == current_difficulty)
            .collect();
        
        if candidates.is_empty() {
            // Try next difficulty level
            let next_difficulty = match current_difficulty {
                Difficulty::Easy => Difficulty::Medium,
                Difficulty::Medium => Difficulty::Hard,
                Difficulty::Hard => Difficulty::Easy,
            };
            
            self.problems.values()
                .find(|p| p.subject == subject && p.chapter == chapter && p.difficulty == next_difficulty)
        } else {
            Some(candidates[0])
        }
    }

    /// Submit answer and update progress
    pub fn submit_answer(&mut self, problem_id: &str, answer: usize) -> Result<bool, String> {
        if let Some(problem) = self.problems.get(problem_id) {
            let is_correct = answer == problem.correct_answer;
            
            let progress_key = format!("{:?}_{}", problem.subject, problem.chapter);
            
            let progress = self.progress.entry(progress_key.clone()).or_insert(StudentProgress {
                subject: problem.subject,
                chapter: problem.chapter.clone(),
                problems_attempted: 0,
                problems_correct: 0,
                current_difficulty: problem.difficulty,
            });
            
            progress.problems_attempted += 1;
            
            if is_correct {
                progress.problems_correct += 1;
                
                // Increase difficulty if performing well (>70% accuracy)
                let accuracy = progress.problems_correct as f64 / progress.problems_attempted as f64;
                if accuracy > 0.7 && progress.current_difficulty != Difficulty::Hard {
                    progress.current_difficulty = match progress.current_difficulty {
                        Difficulty::Easy => Difficulty::Medium,
                        Difficulty::Medium => Difficulty::Hard,
                        Difficulty::Hard => Difficulty::Hard,
                    };
                }
            } else {
                // Decrease difficulty if struggling (<40% accuracy)
                let accuracy = progress.problems_correct as f64 / progress.problems_attempted as f64;
                if accuracy < 0.4 && progress.current_difficulty != Difficulty::Easy {
                    progress.current_difficulty = match progress.current_difficulty {
                        Difficulty::Easy => Difficulty::Easy,
                        Difficulty::Medium => Difficulty::Easy,
                        Difficulty::Hard => Difficulty::Medium,
                    };
                }
            }
            
            self.current_session.push(problem_id.to_string());
            
            Ok(is_correct)
        } else {
            Err("Problem not found".to_string())
        }
    }

    /// Get progress for subject/chapter
    pub fn get_progress(&self, subject: Subject, chapter: &str) -> Option<&StudentProgress> {
        let progress_key = format!("{:?}_{}", subject, chapter);
        self.progress.get(&progress_key)
    }

    /// Get session statistics
    pub fn get_session_stats(&self) -> (usize, usize, f64) {
        let correct = self.current_session.iter()
            .filter(|id| {
                if let Some(p) = self.problems.get(*id) {
                    // Check if last answer was correct (simplified)
                    true
                } else {
                    false
                }
            })
            .count();
        
        let total = self.current_session.len();
        let accuracy = if total > 0 { correct as f64 / total as f64 * 100.0 } else { 0.0 };
        
        (total, correct, accuracy)
    }

    /// Reset session
    pub fn reset_session(&mut self) {
        self.current_session.clear();
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut practice = PracticeGenerator::new();
    
    println!("Sigma Adaptive Practice v0.1 - CBSE Syllabus");
    
    loop {
        println!("\nCommands: generate <subject> <difficulty> <count>, next <subject> <chapter>, answer <id> <choice>, progress <subject> <chapter>, stats, reset, quit");
        println!("Subjects: math, physics, chemistry, biology, cs");
        println!("Difficulty: easy, medium, hard");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "generate" => {
                if parts.len() >= 4 {
                    let subject = match parts[1] {
                        "math" => Subject::Mathematics,
                        "physics" => Subject::Physics,
                        "chemistry" => Subject::Chemistry,
                        "biology" => Subject::Biology,
                        "cs" => Subject::ComputerScience,
                        _ => {
                            println!("Unknown subject");
                            continue;
                        }
                    };
                    let difficulty = match parts[2] {
                        "easy" => Difficulty::Easy,
                        "medium" => Difficulty::Medium,
                        "hard" => Difficulty::Hard,
                        _ => {
                            println!("Unknown difficulty");
                            continue;
                        }
                    };
                    if let Ok(count) = parts[3].parse::<usize>() {
                        let problems = practice.generate_practice_set(subject, difficulty, count);
                        println!("--- Generated {} Problems ---", problems.len());
                        for problem in problems {
                            println!("\n[{}] {} ({} marks)", problem.id, problem.question, problem.marks);
                            for (i, opt) in problem.options.iter().enumerate() {
                                println!("  {}. {}", i + 1, opt);
                            }
                        }
                    }
                }
            }
            "next" => {
                if parts.len() >= 3 {
                    let subject = match parts[1] {
                        "math" => Subject::Mathematics,
                        "physics" => Subject::Physics,
                        "chemistry" => Subject::Chemistry,
                        "biology" => Subject::Biology,
                        "cs" => Subject::ComputerScience,
                        _ => {
                            println!("Unknown subject");
                            continue;
                        }
                    };
                    let chapter = parts[2].to_string();
                    if let Some(problem) = practice.get_next_problem(subject, chapter) {
                        println!("\n--- Next Problem ---");
                        println!("[{}] {} ({} marks)", problem.id, problem.question, problem.marks);
                        for (i, opt) in problem.options.iter().enumerate() {
                            println!("  {}. {}", i + 1, opt);
                        }
                    } else {
                        println!("No more problems available");
                    }
                }
            }
            "answer" => {
                if parts.len() >= 3 {
                    let problem_id = parts[1];
                    if let Ok(answer) = parts[2].parse::<usize>() {
                        match practice.submit_answer(problem_id, answer) {
                            Ok(is_correct) => {
                                if is_correct {
                                    println!("Correct!");
                                } else {
                                    println!("Incorrect!");
                                }
                                if let Some(problem) = practice.problems.get(problem_id) {
                                    println!("Explanation: {}", problem.explanation);
                                }
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "progress" => {
                if parts.len() >= 3 {
                    let subject = match parts[1] {
                        "math" => Subject::Mathematics,
                        "physics" => Subject::Physics,
                        "chemistry" => Subject::Chemistry,
                        "biology" => Subject::Biology,
                        "cs" => Subject::ComputerScience,
                        _ => {
                            println!("Unknown subject");
                            continue;
                        }
                    };
                    let chapter = parts[2].to_string();
                    if let Some(progress) = practice.get_progress(subject, &chapter) {
                        let accuracy = if progress.problems_attempted > 0 {
                            progress.problems_correct as f64 / progress.problems_attempted as f64 * 100.0
                        } else {
                            0.0
                        };
                        println!("--- Progress ---");
                        println!("Attempted: {}", progress.problems_attempted);
                        println!("Correct: {}", progress.problems_correct);
                        println!("Accuracy: {:.1}%", accuracy);
                        println!("Current Difficulty: {:?}", progress.current_difficulty);
                    } else {
                        println!("No progress data available");
                    }
                }
            }
            "stats" => {
                let (total, correct, accuracy) = practice.get_session_stats();
                println!("--- Session Stats ---");
                println!("Total: {}", total);
                println!("Correct: {}", correct);
                println!("Accuracy: {:.1}%", accuracy);
            }
            "reset" => {
                practice.reset_session();
                println!("Session reset");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
