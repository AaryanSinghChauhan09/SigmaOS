// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_moodle.rs — Sigma E-Learning Platform (Moodle)
//
// Implements Moodle-style e-learning platform with course management,
// student enrollment, assignments, quizzes, and grading.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── E-Learning Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,  // student, teacher, admin
    pub enrolled_courses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub description: String,
    pub instructor: String,
    pub students: Vec<String>,
    pub modules: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub id: String,
    pub course_id: String,
    pub name: String,
    pub module_type: String,  // assignment, quiz, resource, forum
    pub content: String,
    pub due_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub id: String,
    pub module_id: String,
    pub student_id: String,
    pub content: String,
    pub submitted_at: String,
    pub grade: Option<f64>,
    pub feedback: String,
}

#[derive(Debug, Clone)]
pub struct QuizAttempt {
    pub id: String,
    pub quiz_id: String,
    pub student_id: String,
    pub score: f64,
    pub max_score: f64,
    pub completed_at: String,
}

// ─── E-Learning Manager ────────────────────────────────────────────────────

pub struct ELearningManager {
    pub users: HashMap<String, User>,
    pub courses: HashMap<String, Course>,
    pub modules: HashMap<String, Module>,
    pub assignments: HashMap<String, Assignment>,
    pub quiz_attempts: HashMap<String, QuizAttempt>,
    pub current_user: Option<String>,
}

impl ELearningManager {
    pub fn new() -> Self {
        let mut manager = ELearningManager {
            users: HashMap::new(),
            courses: HashMap::new(),
            modules: HashMap::new(),
            assignments: HashMap::new(),
            quiz_attempts: HashMap::new(),
            current_user: None,
        };
        
        manager.init_sample_users();
        manager.init_sample_courses();
        manager.init_sample_modules();
        manager
    }

    /// Initialize sample users
    fn init_sample_users(&mut self) {
        self.users.insert("user_001".to_string(), User {
            id: "user_001".to_string(),
            username: "student1".to_string(),
            email: "student1@sigmaos.edu".to_string(),
            role: "student".to_string(),
            enrolled_courses: vec!["course_001".to_string()],
        });

        self.users.insert("user_002".to_string(), User {
            id: "user_002".to_string(),
            username: "teacher1".to_string(),
            email: "teacher1@sigmaos.edu".to_string(),
            role: "teacher".to_string(),
            enrolled_courses: vec!["course_001".to_string()],
        });
    }

    /// Initialize sample courses
    fn init_sample_courses(&mut self) {
        self.courses.insert("course_001".to_string(), Course {
            id: "course_001".to_string(),
            name: "Introduction to Computer Science".to_string(),
            description: "Learn the fundamentals of programming and computer science".to_string(),
            instructor: "user_002".to_string(),
            students: vec!["user_001".to_string()],
            modules: vec!["module_001".to_string(), "module_002".to_string()],
            active: true,
        });
    }

    /// Initialize sample modules
    fn init_sample_modules(&mut self) {
        self.modules.insert("module_001".to_string(), Module {
            id: "module_001".to_string(),
            course_id: "course_001".to_string(),
            name: "Week 1: Introduction".to_string(),
            module_type: "resource".to_string(),
            content: "Welcome to the course! This week covers basic concepts.".to_string(),
            due_date: None,
        });

        self.modules.insert("module_002".to_string(), Module {
            id: "module_002".to_string(),
            course_id: "course_001".to_string(),
            name: "Assignment 1: Hello World".to_string(),
            module_type: "assignment".to_string(),
            content: "Write your first program in Rust.".to_string(),
            due_date: Some("2024-02-01".to_string()),
        });
    }

    /// Create user
    pub fn create_user(&mut self, username: String, email: String, role: String) -> User {
        let user = User {
            id: format!("user_{}", self.users.len()),
            username,
            email,
            role,
            enrolled_courses: Vec::new(),
        };
        
        self.users.insert(user.id.clone(), user.clone());
        user
    }

    /// Create course
    pub fn create_course(&mut self, name: String, description: String, instructor: String) -> Course {
        let course = Course {
            id: format!("course_{}", self.courses.len()),
            name,
            description,
            instructor,
            students: Vec::new(),
            modules: Vec::new(),
            active: true,
        };
        
        self.courses.insert(course.id.clone(), course.clone());
        course
    }

    /// Enroll student in course
    pub fn enroll_student(&mut self, course_id: &str, student_id: &str) -> Result<(), String> {
        if let Some(course) = self.courses.get_mut(course_id) {
            if let Some(student) = self.users.get_mut(student_id) {
                if student.role == "student" {
                    course.students.push(student_id.to_string());
                    student.enrolled_courses.push(course_id.to_string());
                    Ok(())
                } else {
                    Err("User is not a student".to_string())
                }
            } else {
                Err("Student not found".to_string())
            }
        } else {
            Err("Course not found".to_string())
        }
    }

    /// Add module to course
    pub fn add_module(&mut self, course_id: &str, name: String, module_type: String, content: String, due_date: Option<String>) -> Module {
        let module = Module {
            id: format!("module_{}", self.modules.len()),
            course_id: course_id.to_string(),
            name,
            module_type,
            content,
            due_date,
        };
        
        self.modules.insert(module.id.clone(), module.clone());
        
        if let Some(course) = self.courses.get_mut(course_id) {
            course.modules.push(module.id.clone());
        }
        
        module
    }

    /// Submit assignment
    pub fn submit_assignment(&mut self, module_id: &str, student_id: &str, content: String) -> Assignment {
        let assignment = Assignment {
            id: format!("assignment_{}", self.assignments.len()),
            module_id: module_id.to_string(),
            student_id: student_id.to_string(),
            content,
            submitted_at: "now".to_string(),
            grade: None,
            feedback: String::new(),
        };
        
        self.assignments.insert(assignment.id.clone(), assignment.clone());
        assignment
    }

    /// Grade assignment
    pub fn grade_assignment(&mut self, assignment_id: &str, grade: f64, feedback: String) -> Result<(), String> {
        if let Some(assignment) = self.assignments.get_mut(assignment_id) {
            assignment.grade = Some(grade);
            assignment.feedback = feedback;
            Ok(())
        } else {
            Err("Assignment not found".to_string())
        }
    }

    /// Record quiz attempt
    pub fn record_quiz_attempt(&mut self, quiz_id: &str, student_id: &str, score: f64, max_score: f64) -> QuizAttempt {
        let attempt = QuizAttempt {
            id: format!("attempt_{}", self.quiz_attempts.len()),
            quiz_id: quiz_id.to_string(),
            student_id: student_id.to_string(),
            score,
            max_score,
            completed_at: "now".to_string(),
        };
        
        self.quiz_attempts.insert(attempt.id.clone(), attempt.clone());
        attempt
    }

    /// Get user by ID
    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    /// Get course by ID
    pub fn get_course(&self, id: &str) -> Option<&Course> {
        self.courses.get(id)
    }

    /// Get all courses
    pub fn get_all_courses(&self) -> Vec<&Course> {
        self.courses.values().collect()
    }

    /// Get courses for student
    pub fn get_student_courses(&self, student_id: &str) -> Vec<&Course> {
        if let Some(student) = self.users.get(student_id) {
            student.enrolled_courses.iter()
                .filter_map(|id| self.courses.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get modules for course
    pub fn get_course_modules(&self, course_id: &str) -> Vec<&Module> {
        if let Some(course) = self.courses.get(course_id) {
            course.modules.iter()
                .filter_map(|id| self.modules.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Login
    pub fn login(&mut self, username: &str) -> Result<(), String> {
        if self.users.values().any(|u| u.username == username) {
            self.current_user = self.users.values()
                .find(|u| u.username == username)
                .map(|u| u.id.clone());
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Logout
    pub fn logout(&mut self) {
        self.current_user = None;
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = ELearningManager::new();
    
    println!("Sigma E-Learning Platform v0.1 - Moodle Style");
    
    loop {
        println!("\n--- E-Learning Status ---");
        if let Some(user_id) = &manager.current_user {
            if let Some(user) = manager.get_user(user_id) {
                println!("Logged in as: {} ({})", user.username, user.role);
            }
        } else {
            println!("Not logged in");
        }
        println!("Users: {}", manager.users.len());
        println!("Courses: {}", manager.courses.len());
        println!("Modules: {}", manager.modules.len());
        
        println!("\nCommands: login <username>, logout, create_user <username> <email> <role>, create_course <name> <description> <instructor>, enroll <course_id> <student_id>, add_module <course_id> <name> <type> <content> [due_date], submit <module_id> <student_id> <content>, grade <assignment_id> <grade> <feedback>, courses, my_courses, course_modules <id>, quit");
        println!("Roles: student, teacher, admin");
        println!("Module types: assignment, quiz, resource, forum");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "login" => {
                if let Some(arg) = parts.get(1) {
                    match manager.login(arg) {
                        Ok(_) => println!("Logged in"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "logout" => {
                manager.logout();
                println!("Logged out");
            }
            "create_user" => {
                if parts.len() >= 4 {
                    let username = parts[1].to_string();
                    let email = parts[2].to_string();
                    let role = parts[3].to_string();
                    let user = manager.create_user(username, email, role);
                    println!("User created: {}", user.username);
                }
            }
            "create_course" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let description = parts[2].to_string();
                    let instructor = parts[3].to_string();
                    let course = manager.create_course(name, description, instructor);
                    println!("Course created: {}", course.name);
                }
            }
            "enroll" => {
                if parts.len() >= 3 {
                    match manager.enroll_student(parts[1], parts[2]) {
                        Ok(_) => println!("Student enrolled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_module" => {
                if parts.len() >= 5 {
                    let course_id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let module_type = parts[3].to_string();
                    let content = parts[4..].join(" ");
                    let due_date = None;
                    let module = manager.add_module(&course_id, name, module_type, content, due_date);
                    println!("Module added: {}", module.name);
                }
            }
            "submit" => {
                if parts.len() >= 4 {
                    let content = parts[3..].join(" ");
                    let assignment = manager.submit_assignment(parts[1], parts[2], content);
                    println!("Assignment submitted: {}", assignment.id);
                }
            }
            "grade" => {
                if parts.len() >= 4 {
                    if let Ok(grade) = parts[2].parse::<f64>() {
                        let feedback = parts[3..].join(" ");
                        match manager.grade_assignment(parts[1], grade, feedback) {
                            Ok(_) => println!("Assignment graded"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "courses" => {
                println!("--- All Courses ---");
                for course in manager.get_all_courses() {
                    println!("{} - {} ({})", course.id, course.name, if course.active { "[ACTIVE]" } else { "" });
                    println!("  Instructor: {}", course.instructor);
                    println!("  Students: {}", course.students.len());
                }
            }
            "my_courses" => {
                if let Some(user_id) = &manager.current_user {
                    println!("--- My Courses ---");
                    for course in manager.get_student_courses(user_id) {
                        println!("{} - {}", course.id, course.name);
                    }
                } else {
                    println!("Not logged in");
                }
            }
            "course_modules" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Course Modules ---");
                    for module in manager.get_course_modules(arg) {
                        println!("{} - {} ({})", module.id, module.name, module.module_type);
                        if let Some(due) = &module.due_date {
                            println!("  Due: {}", due);
                        }
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
