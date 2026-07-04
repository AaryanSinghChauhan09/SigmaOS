// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/professional/sigma_examprep.rs — Sigma Exam Preparation Modules
//
// Implements AI-generated practice tests for UPSC, SSC, GATE, NET
// and other competitive examinations for Indian professionals.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Exam Types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExamType {
    UPSC,
    SSC,
    GATE,
    NET,
    CAT,
    Banking,
}

#[derive(Debug, Clone)]
pub struct Question {
    pub id: String,
    pub exam_type: ExamType,
    pub subject: String,
    pub difficulty: String,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: String,
}

#[derive(Debug, Clone)]
pub struct TestSession {
    pub exam_type: ExamType,
    pub subject: String,
    pub questions: Vec<String>,
    pub answers: HashMap<String, usize>,
    pub started_at: String,
    pub completed: bool,
}

// ─── Exam Preparation Manager ───────────────────────────────────────────────

pub struct ExamPrepManager {
    pub questions: HashMap<String, Question>,
    pub current_session: Option<TestSession>,
    pub test_history: Vec<TestSession>,
}

impl ExamPrepManager {
    pub fn new() -> Self {
        let mut manager = ExamPrepManager {
            questions: HashMap::new(),
            current_session: None,
            test_history: Vec::new(),
        };
        
        manager.init_questions();
        manager
    }

    /// Initialize question bank for various exams
    fn init_questions(&mut self) {
        // UPSC Questions
        self.add_question(Question {
            id: "upsc_gk_1".to_string(),
            exam_type: ExamType::UPSC,
            subject: "General Knowledge".to_string(),
            difficulty: "Medium".to_string(),
            question: "Which of the following is NOT a fundamental right under the Indian Constitution?".to_string(),
            options: vec![
                "Right to Equality".to_string(),
                "Right to Freedom".to_string(),
                "Right to Property".to_string(),
                "Right to Constitutional Remedies".to_string(),
            ],
            correct_answer: 2,
            explanation: "Right to Property was removed from fundamental rights by the 44th Amendment Act, 1978 and made a legal right under Article 300A.".to_string(),
        });

        self.add_question(Question {
            id: "upsc_polity_1".to_string(),
            exam_type: ExamType::UPSC,
            subject: "Polity".to_string(),
            difficulty: "Hard".to_string(),
            question: "The concept of 'Judicial Review' in the Indian Constitution has been adopted from:".to_string(),
            options: vec![
                "UK".to_string(),
                "USA".to_string(),
                "USSR".to_string(),
                "France".to_string(),
            ],
            correct_answer: 1,
            explanation: "Judicial Review has been adopted from the USA constitution, which allows the judiciary to examine the constitutionality of laws.".to_string(),
        });

        // SSC Questions
        self.add_question(Question {
            id: "ssc_quant_1".to_string(),
            exam_type: ExamType::SSC,
            subject: "Quantitative Aptitude".to_string(),
            difficulty: "Easy".to_string(),
            question: "If a number is increased by 20% and then decreased by 20%, what is the net change?".to_string(),
            options: vec![
                "0%".to_string(),
                "4% decrease".to_string(),
                "4% increase".to_string(),
                "2% decrease".to_string(),
            ],
            correct_answer: 1,
            explanation: "Let original number = 100. After 20% increase = 120. After 20% decrease = 120 × 0.8 = 96. Net change = 4% decrease.".to_string(),
        });

        // GATE Questions
        self.add_question(Question {
            id: "gate_cs_1".to_string(),
            exam_type: ExamType::GATE,
            subject: "Computer Science".to_string(),
            difficulty: "Medium".to_string(),
            question: "What is the time complexity of searching for an element in a balanced Binary Search Tree?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(log n)".to_string(),
                "O(1)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "In a balanced BST, the height is O(log n), so search operation takes O(log n) time.".to_string(),
        });

        self.add_question(Question {
            id: "gate_ds_1".to_string(),
            exam_type: ExamType::GATE,
            subject: "Data Structures".to_string(),
            difficulty: "Hard".to_string(),
            question: "Which data structure is used for implementing recursion?".to_string(),
            options: vec![
                "Queue".to_string(),
                "Stack".to_string(),
                "Linked List".to_string(),
                "Tree".to_string(),
            ],
            correct_answer: 1,
            explanation: "Stack is used for implementing recursion as it follows LIFO (Last In First Out) principle, which matches function call and return mechanism.".to_string(),
        });

        // NET Questions
        self.add_question(Question {
            id: "net_edu_1".to_string(),
            exam_type: ExamType::NET,
            subject: "Education".to_string(),
            difficulty: "Medium".to_string(),
            question: "Which of the following is NOT a level of Bloom's Taxonomy?".to_string(),
            options: vec![
                "Knowledge".to_string(),
                "Comprehension".to_string(),
                "Application".to_string(),
                "Memorization".to_string(),
            ],
            correct_answer: 3,
            explanation: "Bloom's Taxonomy has six levels: Knowledge, Comprehension, Application, Analysis, Synthesis, and Evaluation. Memorization is not a separate level.".to_string(),
        });

        // Banking Questions
        self.add_question(Question {
            id: "bank_apt_1".to_string(),
            exam_type: ExamType::Banking,
            subject: "Banking Awareness".to_string(),
            difficulty: "Easy".to_string(),
            question: "What is the full form of RBI?".to_string(),
            options: vec![
                "Reserve Bank of India".to_string(),
                "Regional Bank of India".to_string(),
                "Rural Bank of India".to_string(),
                "Regular Bank of India".to_string(),
            ],
            correct_answer: 0,
            explanation: "RBI stands for Reserve Bank of India, which is India's central banking institution.".to_string(),
        });
    }

    /// Add question to bank
    fn add_question(&mut self, question: Question) {
        self.questions.insert(question.id.clone(), question);
    }

    /// Start a new test session
    pub fn start_test(&mut self, exam_type: ExamType, subject: String, count: usize) -> Result<(), String> {
        let available_questions: Vec<&Question> = self.questions.values()
            .filter(|q| q.exam_type == exam_type && q.subject == subject)
            .collect();
        
        if available_questions.is_empty() {
            return Err("No questions available for this exam and subject".to_string());
        }
        
        let question_ids: Vec<String> = available_questions.iter()
            .take(count)
            .map(|q| q.id.clone())
            .collect();
        
        self.current_session = Some(TestSession {
            exam_type,
            subject,
            questions: question_ids,
            answers: HashMap::new(),
            started_at: "now".to_string(),
            completed: false,
        });
        
        Ok(())
    }

    /// Submit answer for a question
    pub fn submit_answer(&mut self, question_id: String, answer: usize) -> Result<(), String> {
        if let Some(session) = &mut self.current_session {
            if session.questions.contains(&question_id) {
                session.answers.insert(question_id, answer);
                Ok(())
            } else {
                Err("Question not in current session".to_string())
            }
        } else {
            Err("No active test session".to_string())
        }
    }

    /// Complete test and calculate score
    pub fn complete_test(&mut self) -> Result<(usize, usize, f64), String> {
        if let Some(session) = &mut self.current_session {
            let mut correct = 0;
            let total = session.questions.len();
            
            for question_id in &session.questions {
                if let Some(question) = self.questions.get(question_id) {
                    if let Some(&answer) = session.answers.get(question_id) {
                        if answer == question.correct_answer {
                            correct += 1;
                        }
                    }
                }
            }
            
            let percentage = if total > 0 {
                (correct as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            
            session.completed = true;
            let completed_session = session.clone();
            self.test_history.push(completed_session);
            self.current_session = None;
            
            Ok((correct, total, percentage))
        } else {
            Err("No active test session".to_string())
        }
    }

    /// Get current session
    pub fn get_current_session(&self) -> Option<&TestSession> {
        self.current_session.as_ref()
    }

    /// Get question by ID
    pub fn get_question(&self, id: &str) -> Option<&Question> {
        self.questions.get(id)
    }

    /// Get questions by exam type and subject
    pub fn get_questions(&self, exam_type: ExamType, subject: &str) -> Vec<&Question> {
        self.questions.values()
            .filter(|q| q.exam_type == exam_type && q.subject == subject)
            .collect()
    }

    /// Get available subjects for exam type
    pub fn get_subjects(&self, exam_type: ExamType) -> Vec<String> {
        use std::collections::HashSet;
        
        let mut subjects: HashSet<String> = HashSet::new();
        for question in self.questions.values() {
            if question.exam_type == exam_type {
                subjects.insert(question.subject.clone());
            }
        }
        
        let mut subject_list: Vec<String> = subjects.into_iter().collect();
        subject_list.sort();
        subject_list
    }

    /// Get test history
    pub fn get_test_history(&self) -> &[TestSession] {
        &self.test_history
    }

    /// Generate AI-suggested practice based on weak areas
    pub fn get_suggested_practice(&self, exam_type: ExamType) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        match exam_type {
            ExamType::UPSC => {
                suggestions.push("Focus on Indian Polity and Constitution".to_string());
                suggestions.push("Read daily current affairs from The Hindu".to_string());
                suggestions.push("Practice previous year question papers".to_string());
                suggestions.push("Study NCERT books for basics".to_string());
            }
            ExamType::SSC => {
                suggestions.push("Practice quantitative aptitude daily".to_string());
                suggestions.push("Focus on reasoning and general intelligence".to_string());
                suggestions.push("Improve English vocabulary and grammar".to_string());
                suggestions.push("Solve mock tests regularly".to_string());
            }
            ExamType::GATE => {
                suggestions.push("Focus on core subjects of your branch".to_string());
                suggestions.push("Practice numerical problems".to_string());
                suggestions.push("Revise engineering mathematics".to_string());
                suggestions.push("Solve previous 10 years papers".to_string());
            }
            ExamType::NET => {
                suggestions.push("Study teaching methods and pedagogy".to_string());
                suggestions.push("Focus on research methodology".to_string());
                suggestions.push("Read UGC NET syllabus thoroughly".to_string());
                suggestions.push("Practice paper 1 and paper 2 questions".to_string());
            }
            ExamType::CAT => {
                suggestions.push("Practice reading comprehension daily".to_string());
                suggestions.push("Work on data interpretation speed".to_string());
                suggestions.push("Learn shortcuts for calculations".to_string());
                suggestions.push("Take full-length mock tests".to_string());
            }
            ExamType::Banking => {
                suggestions.push("Focus on banking awareness and current affairs".to_string());
                suggestions.push("Practice quantitative aptitude".to_string());
                suggestions.push("Learn computer basics".to_string());
                suggestions.push("Practice reasoning puzzles".to_string());
            }
        }
        
        suggestions
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = ExamPrepManager::new();
    
    println!("Sigma Exam Prep v0.1 - UPSC, SSC, GATE, NET");
    
    loop {
        println!("\nCommands: start <exam> <subject> <count>, answer <id> <choice>, complete, subjects <exam>, suggest <exam>, history, quit");
        println!("Exams: upsc, ssc, gate, net, cat, banking");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "start" => {
                if parts.len() >= 4 {
                    let exam_type = match parts[1] {
                        "upsc" => ExamType::UPSC,
                        "ssc" => ExamType::SSC,
                        "gate" => ExamType::GATE,
                        "net" => ExamType::NET,
                        "cat" => ExamType::CAT,
                        "banking" => ExamType::Banking,
                        _ => {
                            println!("Unknown exam type");
                            continue;
                        }
                    };
                    let subject = parts[2].to_string();
                    if let Ok(count) = parts[3].parse::<usize>() {
                        match manager.start_test(exam_type, subject, count) {
                            Ok(_) => println!("Test started with {} questions", count),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "answer" => {
                if parts.len() >= 3 {
                    let question_id = parts[1].to_string();
                    if let Ok(answer) = parts[2].parse::<usize>() {
                        match manager.submit_answer(question_id, answer) {
                            Ok(_) => println!("Answer recorded"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "complete" => {
                match manager.complete_test() {
                    Ok((correct, total, percentage)) => {
                        println!("Test Results: {}/{} ({:.1}%)", correct, total, percentage);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "subjects" => {
                if let Some(arg) = parts.get(1) {
                    let exam_type = match *arg {
                        "upsc" => ExamType::UPSC,
                        "ssc" => ExamType::SSC,
                        "gate" => ExamType::GATE,
                        "net" => ExamType::NET,
                        "cat" => ExamType::CAT,
                        "banking" => ExamType::Banking,
                        _ => {
                            println!("Unknown exam type");
                            continue;
                        }
                    };
                    println!("--- Available Subjects ---");
                    for subject in manager.get_subjects(exam_type) {
                        println!("- {}", subject);
                    }
                }
            }
            "suggest" => {
                if let Some(arg) = parts.get(1) {
                    let exam_type = match *arg {
                        "upsc" => ExamType::UPSC,
                        "ssc" => ExamType::SSC,
                        "gate" => ExamType::GATE,
                        "net" => ExamType::NET,
                        "cat" => ExamType::CAT,
                        "banking" => ExamType::Banking,
                        _ => {
                            println!("Unknown exam type");
                            continue;
                        }
                    };
                    println!("--- AI Suggestions ---");
                    for suggestion in manager.get_suggested_practice(exam_type) {
                        println!("- {}", suggestion);
                    }
                }
            }
            "history" => {
                println!("--- Test History ---");
                for (i, session) in manager.get_test_history().iter().enumerate() {
                    let exam_str = format!("{:?}", session.exam_type);
                    println!("{}. {} - {} ({})", i + 1, exam_str, session.subject, if session.completed { "Completed" } else { "Incomplete" });
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
