// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_civilservices.rs — Sigma Civil Services Prep
//
// Implements UPSC/SSC/GATE practice modules with adaptive difficulty
// for Indian civil services examination preparation.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Civil Services Types ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExamType {
    UPSC,
    SSC,
    GATE,
    NET,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subject {
    GeneralStudies,
    History,
    Geography,
    Polity,
    Economy,
    Science,
    Mathematics,
    Reasoning,
    English,
    Aptitude,
}

#[derive(Debug, Clone)]
pub struct Question {
    pub id: String,
    pub exam_type: ExamType,
    pub subject: Subject,
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: String,
    pub difficulty: String,
}

#[derive(Debug, Clone)]
pub struct TestSession {
    pub id: String,
    pub exam_type: ExamType,
    pub subject: Subject,
    pub questions: Vec<Question>,
    pub answers: Vec<Option<usize>>,
    pub score: f64,
    pub time_taken_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub exam_type: ExamType,
    pub subjects: Vec<Subject>,
    pub test_scores: HashMap<String, f64>,
    pub adaptive_level: f64,
}

// ─── Civil Services Prep Engine ─────────────────────────────────────────────

pub struct CivilServicesEngine {
    pub questions: HashMap<String, Question>,
    pub test_sessions: HashMap<String, TestSession>,
    pub user_profiles: HashMap<String, UserProfile>,
}

impl CivilServicesEngine {
    pub fn new() -> Self {
        let mut engine = CivilServicesEngine {
            questions: HashMap::new(),
            test_sessions: HashMap::new(),
            user_profiles: HashMap::new(),
        };
        
        engine.init_question_bank();
        engine
    }

    /// Initialize question bank
    fn init_question_bank(&mut self) {
        // UPSC General Studies
        self.questions.insert("q_001".to_string(), Question {
            id: "q_001".to_string(),
            exam_type: ExamType::UPSC,
            subject: Subject::Polity,
            question: "Which Article of the Indian Constitution deals with the Right to Equality?".to_string(),
            options: vec![
                "Article 14".to_string(),
                "Article 15".to_string(),
                "Article 16".to_string(),
                "Article 17".to_string(),
            ],
            correct_answer: 0,
            explanation: "Article 14 guarantees equality before law and equal protection of laws within the territory of India.".to_string(),
            difficulty: "Medium".to_string(),
        });

        self.questions.insert("q_002".to_string(), Question {
            id: "q_002".to_string(),
            exam_type: ExamType::UPSC,
            subject: Subject::History,
            question: "Who was the first Governor-General of independent India?".to_string(),
            options: vec![
                "Lord Mountbatten".to_string(),
                "C. Rajagopalachari".to_string(),
                "Dr. Rajendra Prasad".to_string(),
                "Jawaharlal Nehru".to_string(),
            ],
            correct_answer: 0,
            explanation: "Lord Mountbatten served as the last Viceroy and first Governor-General of independent India from 1947 to 1948.".to_string(),
            difficulty: "Easy".to_string(),
        });

        // SSC Reasoning
        self.questions.insert("q_003".to_string(), Question {
            id: "q_003".to_string(),
            exam_type: ExamType::SSC,
            subject: Subject::Reasoning,
            question: "If A is the brother of B, B is the sister of C, and C is the father of D, how is A related to D?".to_string(),
            options: vec![
                "Uncle".to_string(),
                "Nephew".to_string(),
                "Cousin".to_string(),
                "Brother".to_string(),
            ],
            correct_answer: 0,
            explanation: "A is brother of B, B is sister of C, so A is brother of C. C is father of D, so A is uncle of D.".to_string(),
            difficulty: "Medium".to_string(),
        });

        // GATE Mathematics
        self.questions.insert("q_004".to_string(), Question {
            id: "q_004".to_string(),
            exam_type: ExamType::GATE,
            subject: Subject::Mathematics,
            question: "What is the value of ∫(x² + 1)dx from 0 to 2?".to_string(),
            options: vec![
                "14/3".to_string(),
                "10/3".to_string(),
                "8/3".to_string(),
                "4/3".to_string(),
            ],
            correct_answer: 0,
            explanation: "∫(x² + 1)dx = x³/3 + x. Evaluating from 0 to 2: (8/3 + 2) - (0 + 0) = 8/3 + 2 = 14/3.".to_string(),
            difficulty: "Medium".to_string(),
        });

        // Economy
        self.questions.insert("q_005".to_string(), Question {
            id: "q_005".to_string(),
            exam_type: ExamType::UPSC,
            subject: Subject::Economy,
            question: "Which Five-Year Plan introduced the concept of 'Mixed Economy' in India?".to_string(),
            options: vec![
                "First Five-Year Plan".to_string(),
                "Second Five-Year Plan".to_string(),
                "Third Five-Year Plan".to_string(),
                "Fourth Five-Year Plan".to_string(),
            ],
            correct_answer: 1,
            explanation: "The Second Five-Year Plan (1956-61) under Mahalanobis model emphasized industrialization and introduced the concept of mixed economy.".to_string(),
            difficulty: "Hard".to_string(),
        });

        // Geography
        self.questions.insert("q_006".to_string(), Question {
            id: "q_006".to_string(),
            exam_type: ExamType::UPSC,
            subject: Subject::Geography,
            question: "Which is the longest river in India?".to_string(),
            options: vec![
                "Yamuna".to_string(),
                "Godavari".to_string(),
                "Ganga".to_string(),
                "Brahmaputra".to_string(),
            ],
            correct_answer: 2,
            explanation: "The Ganga is the longest river in India, flowing approximately 2,525 km from its source in Gangotri Glacier to the Bay of Bengal.".to_string(),
            difficulty: "Easy".to_string(),
        });
    }

    /// Create user profile
    pub fn create_profile(&mut self, name: String, exam_type: ExamType, subjects: Vec<Subject>) -> UserProfile {
        let profile = UserProfile {
            id: format!("user_{}", self.user_profiles.len()),
            name,
            exam_type,
            subjects,
            test_scores: HashMap::new(),
            adaptive_level: 1.0,
        };
        
        self.user_profiles.insert(profile.id.clone(), profile.clone());
        profile
    }

    /// Get questions by exam and subject
    pub fn get_questions(&self, exam_type: ExamType, subject: Subject) -> Vec<&Question> {
        self.questions.values()
            .filter(|q| q.exam_type == exam_type && q.subject == subject)
            .collect()
    }

    /// Get questions by difficulty
    pub fn get_questions_by_difficulty(&self, difficulty: &str) -> Vec<&Question> {
        self.questions.values()
            .filter(|q| q.difficulty == difficulty)
            .collect()
    }

    /// Create adaptive test session
    pub fn create_test_session(&mut self, profile_id: &str, subject: Subject, num_questions: usize) -> Result<TestSession, String> {
        if let Some(profile) = self.user_profiles.get(profile_id) {
            let mut available_questions: Vec<&Question> = self.questions.values()
                .filter(|q| q.exam_type == profile.exam_type && q.subject == subject)
                .collect();
            
            // Adaptive selection based on user level
            available_questions.sort_by(|a, b| {
                let a_diff = match a.difficulty.as_str() {
                    "Easy" => 1,
                    "Medium" => 2,
                    "Hard" => 3,
                    _ => 2,
                };
                let b_diff = match b.difficulty.as_str() {
                    "Easy" => 1,
                    "Medium" => 2,
                    "Hard" => 3,
                    _ => 2,
                };
                (a_diff as f64 - profile.adaptive_level).abs()
                    .partial_cmp(&(b_diff as f64 - profile.adaptive_level).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            
            let selected_questions: Vec<Question> = available_questions
                .into_iter()
                .take(num_questions)
                .cloned()
                .collect();
            
            let session = TestSession {
                id: format!("session_{}", self.test_sessions.len()),
                exam_type: profile.exam_type,
                subject,
                questions: selected_questions,
                answers: vec![None; num_questions],
                score: 0.0,
                time_taken_minutes: 0,
            };
            
            self.test_sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        } else {
            Err("Profile not found".to_string())
        }
    }

    /// Submit answer for a question
    pub fn submit_answer(&mut self, session_id: &str, question_index: usize, answer: usize) -> Result<(), String> {
        if let Some(session) = self.test_sessions.get_mut(session_id) {
            if question_index < session.questions.len() {
                session.answers[question_index] = Some(answer);
                Ok(())
            } else {
                Err("Question index out of range".to_string())
            }
        } else {
            Err("Session not found".to_string())
        }
    }

    /// Calculate test score
    pub fn calculate_score(&mut self, session_id: &str) -> f64 {
        if let Some(session) = self.test_sessions.get_mut(session_id) {
            let mut correct = 0;
            for (i, question) in session.questions.iter().enumerate() {
                if let Some(answer) = session.answers[i] {
                    if answer == question.correct_answer {
                        correct += 1;
                    }
                }
            }
            session.score = (correct as f64 / session.questions.len() as f64) * 100.0;
            session.score
        } else {
            0.0
        }
    }

    /// Update adaptive level based on performance
    pub fn update_adaptive_level(&mut self, profile_id: &str, score: f64) {
        if let Some(profile) = self.user_profiles.get_mut(profile_id) {
            if score > 80.0 {
                profile.adaptive_level = (profile.adaptive_level + 0.5).min(3.0);
            } else if score < 50.0 {
                profile.adaptive_level = (profile.adaptive_level - 0.5).max(1.0);
            }
        }
    }

    /// Get exam type name
    pub fn get_exam_name(&self, exam_type: ExamType) -> &str {
        match exam_type {
            ExamType::UPSC => "UPSC Civil Services",
            ExamType::SSC => "SSC CGL",
            ExamType::GATE => "GATE",
            ExamType::NET => "UGC NET",
        }
    }

    /// Get subject name
    pub fn get_subject_name(&self, subject: Subject) -> &str {
        match subject {
            Subject::GeneralStudies => "General Studies",
            Subject::History => "History",
            Subject::Geography => "Geography",
            Subject::Polity => "Indian Polity",
            Subject::Economy => "Economy",
            Subject::Science => "General Science",
            Subject::Mathematics => "Mathematics",
            Subject::Reasoning => "Reasoning",
            Subject::English => "English",
            Subject::Aptitude => "Aptitude",
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut engine = CivilServicesEngine::new();
    
    println!("Sigma Civil Services Prep v0.1 - UPSC/SSC/GATE Practice");
    
    loop {
        println!("\nCommands: profile <name> <exam> <subject>, test <profile_id> <subject> <count>, answer <session> <q_num> <answer>, score <session>, profile <id>, quit");
        println!("Exams: upsc, ssc, gate, net");
        println!("Subjects: gs, history, geography, polity, economy, science, math, reasoning, english, aptitude");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "profile" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let exam_type = match parts[2] {
                        "upsc" => ExamType::UPSC,
                        "ssc" => ExamType::SSC,
                        "gate" => ExamType::GATE,
                        "net" => ExamType::NET,
                        _ => {
                            println!("Unknown exam type");
                            continue;
                        }
                    };
                    let subject = match parts[3] {
                        "gs" => Subject::GeneralStudies,
                        "history" => Subject::History,
                        "geography" => Subject::Geography,
                        "polity" => Subject::Polity,
                        "economy" => Subject::Economy,
                        "science" => Subject::Science,
                        "math" => Subject::Mathematics,
                        "reasoning" => Subject::Reasoning,
                        "english" => Subject::English,
                        "aptitude" => Subject::Aptitude,
                        _ => {
                            println!("Unknown subject");
                            continue;
                        }
                    };
                    let profile = engine.create_profile(name, exam_type, vec![subject]);
                    println!("Profile created: {}", profile.id);
                }
            }
            "test" => {
                if parts.len() >= 4 {
                    let profile_id = parts[1];
                    let subject = match parts[2] {
                        "gs" => Subject::GeneralStudies,
                        "history" => Subject::History,
                        "geography" => Subject::Geography,
                        "polity" => Subject::Polity,
                        "economy" => Subject::Economy,
                        "science" => Subject::Science,
                        "math" => Subject::Mathematics,
                        "reasoning" => Subject::Reasoning,
                        "english" => Subject::English,
                        "aptitude" => Subject::Aptitude,
                        _ => {
                            println!("Unknown subject");
                            continue;
                        }
                    };
                    if let Ok(count) = parts[3].parse::<usize>() {
                        match engine.create_test_session(profile_id, subject, count) {
                            Ok(session) => {
                                println!("Test session created: {}", session.id);
                                println!("Questions: {}", session.questions.len());
                                for (i, q) in session.questions.iter().enumerate() {
                                    println!("\nQ{}: {}", i + 1, q.question);
                                    for (j, opt) in q.options.iter().enumerate() {
                                        println!("  {}. {}", j + 1, opt);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "answer" => {
                if parts.len() >= 4 {
                    let session_id = parts[1];
                    if let (Ok(q_num), Ok(answer)) = (parts[2].parse::<usize>(), parts[3].parse::<usize>()) {
                        match engine.submit_answer(session_id, q_num - 1, answer - 1) {
                            Ok(_) => println!("Answer submitted"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "score" => {
                if let Some(arg) = parts.get(1) {
                    let score = engine.calculate_score(arg);
                    println!("Score: {:.1}%", score);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
