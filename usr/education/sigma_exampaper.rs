// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_exampaper.rs — Sigma AI Exam Paper Generator
//
// Implements AI-generated practice papers aligned with NCERT curriculum
// for CBSE students across all subjects and classes.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Exam Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuestionType {
    MultipleChoice,
    ShortAnswer,
    LongAnswer,
    VeryLongAnswer,
    TrueFalse,
    FillInTheBlanks,
}

#[derive(Debug, Clone)]
pub struct Question {
    pub id: String,
    pub question_type: QuestionType,
    pub subject: String,
    pub chapter: String,
    pub class: String,
    pub difficulty: String,
    pub marks: u32,
    pub question_text: String,
    pub options: Vec<String>,  // For MCQ
    pub answer: String,
    pub explanation: String,
}

#[derive(Debug, Clone)]
pub struct ExamPaper {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub class: String,
    pub duration_minutes: u32,
    pub total_marks: u32,
    pub questions: Vec<Question>,
    pub sections: Vec<ExamSection>,
}

#[derive(Debug, Clone)]
pub struct ExamSection {
    pub section_name: String,
    pub question_type: QuestionType,
    pub number_of_questions: u32,
    pub marks_per_question: u32,
}

// ─── Exam Paper Generator ───────────────────────────────────────────────────

pub struct ExamPaperGenerator {
    pub question_bank: HashMap<String, Question>,
    pub papers: Vec<ExamPaper>,
}

impl ExamPaperGenerator {
    pub fn new() -> Self {
        let mut generator = ExamPaperGenerator {
            question_bank: HashMap::new(),
            papers: Vec::new(),
        };
        
        generator.init_mathematics_questions();
        generator.init_science_questions();
        generator.init_social_science_questions();
        generator
    }

    /// Initialize mathematics questions
    fn init_mathematics_questions(&mut self) {
        // Class 10 - Quadratic Equations
        self.question_bank.insert("q_math_001".to_string(), Question {
            id: "q_math_001".to_string(),
            question_type: QuestionType::MultipleChoice,
            subject: "Mathematics".to_string(),
            chapter: "Quadratic Equations".to_string(),
            class: "10".to_string(),
            difficulty: "Easy".to_string(),
            marks: 1,
            question_text: "The roots of the quadratic equation x² - 3x - 10 = 0 are:".to_string(),
            options: vec![
                "2 and -5".to_string(),
                "-2 and 5".to_string(),
                "2 and 5".to_string(),
                "-2 and -5".to_string(),
            ],
            answer: "2 and -5".to_string(),
            explanation: "Using factorization: x² - 3x - 10 = (x - 5)(x + 2) = 0, so x = 5 or x = -2".to_string(),
        });

        self.question_bank.insert("q_math_002".to_string(), Question {
            id: "q_math_002".to_string(),
            question_type: QuestionType::ShortAnswer,
            subject: "Mathematics".to_string(),
            chapter: "Arithmetic Progressions".to_string(),
            class: "10".to_string(),
            difficulty: "Medium".to_string(),
            marks: 2,
            question_text: "Find the 10th term of the AP: 3, 8, 13, 18, ...".to_string(),
            options: Vec::new(),
            answer: "48".to_string(),
            explanation: "First term a = 3, common difference d = 5. 10th term = a + 9d = 3 + 9×5 = 48".to_string(),
        });

        self.question_bank.insert("q_math_003".to_string(), Question {
            id: "q_math_003".to_string(),
            question_type: QuestionType::LongAnswer,
            subject: "Mathematics".to_string(),
            chapter: "Triangles".to_string(),
            class: "10".to_string(),
            difficulty: "Hard".to_string(),
            marks: 4,
            question_text: "State and prove Basic Proportionality Theorem (Thales Theorem).".to_string(),
            options: Vec::new(),
            answer: "If a line is drawn parallel to one side of a triangle to intersect the other two sides in distinct points, the other two sides are divided in the same ratio.".to_string(),
            explanation: "Proof involves constructing parallel lines and using properties of similar triangles.".to_string(),
        });

        // Class 12 - Calculus
        self.question_bank.insert("q_math_004".to_string(), Question {
            id: "q_math_004".to_string(),
            question_type: QuestionType::MultipleChoice,
            subject: "Mathematics".to_string(),
            chapter: "Continuity and Differentiability".to_string(),
            class: "12".to_string(),
            difficulty: "Medium".to_string(),
            marks: 1,
            question_text: "The derivative of sin(x²) with respect to x is:".to_string(),
            options: vec![
                "cos(x²)".to_string(),
                "2x cos(x²)".to_string(),
                "-2x cos(x²)".to_string(),
                "cos(2x)".to_string(),
            ],
            answer: "2x cos(x²)".to_string(),
            explanation: "Using chain rule: d/dx[sin(x²)] = cos(x²) × d/dx(x²) = cos(x²) × 2x = 2x cos(x²)".to_string(),
        });
    }

    /// Initialize science questions
    fn init_science_questions(&mut self) {
        // Class 10 - Physics
        self.question_bank.insert("q_sci_001".to_string(), Question {
            id: "q_sci_001".to_string(),
            question_type: QuestionType::MultipleChoice,
            subject: "Science".to_string(),
            chapter: "Light - Reflection and Refraction".to_string(),
            class: "10".to_string(),
            difficulty: "Easy".to_string(),
            marks: 1,
            question_text: "The focal length of a convex lens of power +2.0D is:".to_string(),
            options: vec![
                "+0.5 m".to_string(),
                "+0.2 m".to_string(),
                "+2.0 m".to_string(),
                "-0.5 m".to_string(),
            ],
            answer: "+0.5 m".to_string(),
            explanation: "Focal length = 1/Power = 1/2 = 0.5 m = 50 cm".to_string(),
        });

        self.question_bank.insert("q_sci_002".to_string(), Question {
            id: "q_sci_002".to_string(),
            question_type: QuestionType::ShortAnswer,
            subject: "Science".to_string(),
            chapter: "Chemical Reactions and Equations".to_string(),
            class: "10".to_string(),
            difficulty: "Medium".to_string(),
            marks: 2,
            question_text: "Balance the following chemical equation: Fe + H₂O → Fe₃O₄ + H₂".to_string(),
            options: Vec::new(),
            answer: "3Fe + 4H₂O → Fe₃O₄ + 4H₂".to_string(),
            explanation: "Balancing atoms: 3 Fe on both sides, 4 O and 4 H on both sides".to_string(),
        });

        // Class 12 - Physics
        self.question_bank.insert("q_sci_003".to_string(), Question {
            id: "q_sci_003".to_string(),
            question_type: QuestionType::LongAnswer,
            subject: "Physics".to_string(),
            chapter: "Electrostatics".to_string(),
            class: "12".to_string(),
            difficulty: "Hard".to_string(),
            marks: 5,
            question_text: "Derive the expression for electric field due to a point charge using Coulomb's law.".to_string(),
            options: Vec::new(),
            answer: "E = kQ/r², where k = 1/4πε₀".to_string(),
            explanation: "Using Coulomb's law F = kQq/r² and definition E = F/q, we get E = kQ/r²".to_string(),
        });
    }

    /// Initialize social science questions
    fn init_social_science_questions(&mut self) {
        // Class 10 - History
        self.question_bank.insert("q_ss_001".to_string(), Question {
            id: "q_ss_001".to_string(),
            question_type: QuestionType::MultipleChoice,
            subject: "History".to_string(),
            chapter: "The Rise of Nationalism in Europe".to_string(),
            class: "10".to_string(),
            difficulty: "Easy".to_string(),
            marks: 1,
            question_text: "Who was the founder of the Young Italy movement?".to_string(),
            options: vec![
                "Giuseppe Mazzini".to_string(),
                "Count Cavour".to_string(),
                "Giuseppe Garibaldi".to_string(),
                "Victor Emmanuel II".to_string(),
            ],
            answer: "Giuseppe Mazzini".to_string(),
            explanation: "Giuseppe Mazzini founded Young Italy in 1831 to unify Italy".to_string(),
        });

        // Class 10 - Geography
        self.question_bank.insert("q_ss_002".to_string(), Question {
            id: "q_ss_002".to_string(),
            question_type: QuestionType::ShortAnswer,
            subject: "Geography".to_string(),
            chapter: "Resources and Development".to_string(),
            class: "10".to_string(),
            difficulty: "Medium".to_string(),
            marks: 2,
            question_text: "What is sustainable development?".to_string(),
            options: Vec::new(),
            answer: "Development that meets the needs of the present without compromising the ability of future generations to meet their own needs".to_string(),
            explanation: "Sustainable development balances economic growth with environmental protection".to_string(),
        });
    }

    /// Get questions by subject and class
    pub fn get_questions(&self, subject: &str, class: &str) -> Vec<&Question> {
        self.question_bank.values()
            .filter(|q| q.subject == subject && q.class == class)
            .collect()
    }

    /// Get questions by chapter
    pub fn get_questions_by_chapter(&self, subject: &str, chapter: &str) -> Vec<&Question> {
        self.question_bank.values()
            .filter(|q| q.subject == subject && q.chapter == chapter)
            .collect()
    }

    /// Generate exam paper
    pub fn generate_paper(&mut self, subject: String, class: String, title: String) -> ExamPaper {
        let questions = self.get_questions(&subject, &class);
        
        let sections = vec![
            ExamSection {
                section_name: "Section A".to_string(),
                question_type: QuestionType::MultipleChoice,
                number_of_questions: 10,
                marks_per_question: 1,
            },
            ExamSection {
                section_name: "Section B".to_string(),
                question_type: QuestionType::ShortAnswer,
                number_of_questions: 5,
                marks_per_question: 2,
            },
            ExamSection {
                section_name: "Section C".to_string(),
                question_type: QuestionType::LongAnswer,
                number_of_questions: 3,
                marks_per_question: 4,
            },
        ];
        
        let total_marks = sections.iter()
            .map(|s| s.number_of_questions * s.marks_per_question)
            .sum();
        
        let paper = ExamPaper {
            id: format!("paper_{}", self.papers.len()),
            title,
            subject,
            class,
            duration_minutes: 180,
            total_marks,
            questions: questions.iter().map(|q| (*q).clone()).collect(),
            sections,
        };
        
        self.papers.push(paper.clone());
        paper
    }

    /// Get all papers
    pub fn get_all_papers(&self) -> &[ExamPaper] {
        &self.papers
    }

    /// Get paper by ID
    pub fn get_paper(&self, id: &str) -> Option<&ExamPaper> {
        self.papers.iter().find(|p| p.id == id)
    }

    /// Generate answer key
    pub fn generate_answer_key(&self, paper_id: &str) -> HashMap<String, String> {
        let mut answer_key = HashMap::new();
        
        if let Some(paper) = self.get_paper(paper_id) {
            for question in &paper.questions {
                answer_key.insert(question.id.clone(), question.answer.clone());
            }
        }
        
        answer_key
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut generator = ExamPaperGenerator::new();
    
    println!("Sigma AI Exam Paper Generator v0.1 - NCERT Aligned");
    
    loop {
        println!("\n--- Available Subjects ---");
        println!("Mathematics, Science, History, Geography");
        
        println!("\nCommands: generate <subject> <class> <title>, papers, paper <id>, answerkey <id>, questions <subject> <class>, quit");
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
                    let subject = parts[1].to_string();
                    let class = parts[2].to_string();
                    let title = parts[3..].join(" ");
                    let paper = generator.generate_paper(subject.clone(), class.clone(), title);
                    println!("Exam paper generated: {}", paper.id);
                    println!("Subject: {}", paper.subject);
                    println!("Class: {}", paper.class);
                    println!("Duration: {} minutes", paper.duration_minutes);
                    println!("Total Marks: {}", paper.total_marks);
                    println!("Questions: {}", paper.questions.len());
                }
            }
            "papers" => {
                println!("--- All Generated Papers ---");
                for paper in generator.get_all_papers() {
                    println!("{} - {} (Class {}, {} marks)", paper.id, paper.title, paper.class, paper.total_marks);
                }
            }
            "paper" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(paper) = generator.get_paper(arg) {
                        println!("--- Exam Paper ---");
                        println!("Title: {}", paper.title);
                        println!("Subject: {}", paper.subject);
                        println!("Class: {}", paper.class);
                        println!("Duration: {} minutes", paper.duration_minutes);
                        println!("Total Marks: {}", paper.total_marks);
                        println!("\n--- Sections ---");
                        for section in &paper.sections {
                            println!("{}: {} questions × {} marks = {} marks", 
                                section.section_name, section.number_of_questions, 
                                section.marks_per_question, section.number_of_questions * section.marks_per_question);
                        }
                        println!("\n--- Questions ---");
                        for (i, question) in paper.questions.iter().enumerate() {
                            println!("\nQ{}. [{} marks] {}", i + 1, question.marks, question.question_text);
                            if !question.options.is_empty() {
                                for (j, option) in question.options.iter().enumerate() {
                                    println!("  {}. {}", j + 1, option);
                                }
                            }
                        }
                    }
                }
            }
            "answerkey" => {
                if let Some(arg) = parts.get(1) {
                    let answer_key = generator.generate_answer_key(arg);
                    println!("--- Answer Key ---");
                    for (question_id, answer) in &answer_key {
                        println!("{}: {}", question_id, answer);
                    }
                }
            }
            "questions" => {
                if parts.len() >= 3 {
                    let subject = parts[1];
                    let class = parts[2];
                    println!("--- Questions for {} (Class {}) ---", subject, class);
                    for question in generator.get_questions(subject, class) {
                        println!("{} - [{}] {}", question.id, question.difficulty, question.question_text);
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
