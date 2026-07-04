// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/sector/sigma_project.rs — Sigma Project Management (OpenProject)
//
// Implements OpenProject-style project management with work packages,
// task tracking, time logging, Gantt charts, and team collaboration.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Project Management Types ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,  // active, archived, on_hold
    pub start_date: String,
    pub end_date: String,
    pub members: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WorkPackage {
    pub id: String,
    pub project_id: String,
    pub subject: String,
    pub description: String,
    pub status: String,  // new, in_progress, resolved, closed
    pub priority: String,  // low, normal, high, urgent
    pub assignee: Option<String>,
    pub due_date: Option<String>,
    pub estimated_hours: f64,
    pub spent_hours: f64,
}

#[derive(Debug, Clone)]
pub struct TimeEntry {
    pub id: String,
    pub work_package_id: String,
    pub user_id: String,
    pub hours: f64,
    pub date: String,
    pub activity: String,
    pub comments: String,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub work_package_id: String,
    pub title: String,
    pub completed: bool,
    pub due_date: Option<String>,
}

// ─── Project Manager ────────────────────────────────────────────────────

pub struct ProjectManager {
    pub users: HashMap<String, User>,
    pub projects: HashMap<String, Project>,
    pub work_packages: HashMap<String, WorkPackage>,
    pub time_entries: Vec<TimeEntry>,
    pub tasks: Vec<Task>,
    pub current_project: Option<String>,
}

impl ProjectManager {
    pub fn new() -> Self {
        let mut manager = ProjectManager {
            users: HashMap::new(),
            projects: HashMap::new(),
            work_packages: HashMap::new(),
            time_entries: Vec::new(),
            tasks: Vec::new(),
            current_project: None,
        };
        
        manager.init_sample_users();
        manager.init_sample_projects();
        manager.init_sample_work_packages();
        manager
    }

    /// Initialize sample users
    fn init_sample_users(&mut self) {
        self.users.insert("user_001".to_string(), User {
            id: "user_001".to_string(),
            name: "Amit Sharma".to_string(),
            email: "amit.sharma@sigmaos.org".to_string(),
            role: "Project Manager".to_string(),
        });

        self.users.insert("user_002".to_string(), User {
            id: "user_002".to_string(),
            name: "Priya Patel".to_string(),
            email: "priya.patel@sigmaos.org".to_string(),
            role: "Developer".to_string(),
        });
    }

    /// Initialize sample projects
    fn init_sample_projects(&mut self) {
        self.projects.insert("proj_001".to_string(), Project {
            id: "proj_001".to_string(),
            name: "SigmaOS Development".to_string(),
            description: "Core operating system development".to_string(),
            status: "active".to_string(),
            start_date: "2024-01-01".to_string(),
            end_date: "2024-12-31".to_string(),
            members: vec!["user_001".to_string(), "user_002".to_string()],
        });
    }

    /// Initialize sample work packages
    fn init_sample_work_packages(&mut self) {
        self.work_packages.insert("wp_001".to_string(), WorkPackage {
            id: "wp_001".to_string(),
            project_id: "proj_001".to_string(),
            subject: "Implement kernel scheduler".to_string(),
            description: "Design and implement the kernel task scheduler".to_string(),
            status: "in_progress".to_string(),
            priority: "high".to_string(),
            assignee: Some("user_002".to_string()),
            due_date: Some("2024-03-31".to_string()),
            estimated_hours: 40.0,
            spent_hours: 15.5,
        });
    }

    /// Create user
    pub fn create_user(&mut self, name: String, email: String, role: String) -> User {
        let user = User {
            id: format!("user_{}", self.users.len()),
            name,
            email,
            role,
        };
        
        self.users.insert(user.id.clone(), user.clone());
        user
    }

    /// Create project
    pub fn create_project(&mut self, name: String, description: String, start_date: String, end_date: String) -> Project {
        let project = Project {
            id: format!("proj_{}", self.projects.len()),
            name,
            description,
            status: "active".to_string(),
            start_date,
            end_date,
            members: Vec::new(),
        };
        
        self.projects.insert(project.id.clone(), project.clone());
        self.current_project = Some(project.id.clone());
        project
    }

    /// Add member to project
    pub fn add_member(&mut self, project_id: &str, user_id: &str) -> Result<(), String> {
        if let Some(project) = self.projects.get_mut(project_id) {
            if self.users.contains_key(user_id) {
                project.members.push(user_id.to_string());
                Ok(())
            } else {
                Err("User not found".to_string())
            }
        } else {
            Err("Project not found".to_string())
        }
    }

    /// Create work package
    pub fn create_work_package(&mut self, project_id: &str, subject: String, description: String, priority: String, assignee: Option<String>, due_date: Option<String>, estimated_hours: f64) -> WorkPackage {
        let work_package = WorkPackage {
            id: format!("wp_{}", self.work_packages.len()),
            project_id: project_id.to_string(),
            subject,
            description,
            status: "new".to_string(),
            priority,
            assignee,
            due_date,
            estimated_hours,
            spent_hours: 0.0,
        };
        
        self.work_packages.insert(work_package.id.clone(), work_package.clone());
        work_package
    }

    /// Update work package status
    pub fn update_wp_status(&mut self, wp_id: &str, status: String) -> Result<(), String> {
        if let Some(wp) = self.work_packages.get_mut(wp_id) {
            wp.status = status;
            Ok(())
        } else {
            Err("Work package not found".to_string())
        }
    }

    /// Log time
    pub fn log_time(&mut self, wp_id: &str, user_id: &str, hours: f64, activity: String, comments: String) -> Result<TimeEntry, String> {
        if let Some(wp) = self.work_packages.get_mut(wp_id) {
            wp.spent_hours += hours;
            
            let time_entry = TimeEntry {
                id: format!("time_{}", self.time_entries.len()),
                work_package_id: wp_id.to_string(),
                user_id: user_id.to_string(),
                hours,
                date: "now".to_string(),
                activity,
                comments,
            };
            
            self.time_entries.push(time_entry.clone());
            Ok(time_entry)
        } else {
            Err("Work package not found".to_string())
        }
    }

    /// Create task
    pub fn create_task(&mut self, wp_id: &str, title: String, due_date: Option<String>) -> Task {
        let task = Task {
            id: format!("task_{}", self.tasks.len()),
            work_package_id: wp_id.to_string(),
            title,
            completed: false,
            due_date,
        };
        
        self.tasks.push(task.clone());
        task
    }

    /// Complete task
    pub fn complete_task(&mut self, task_id: &str) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.completed = true;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    /// Get work packages for project
    pub fn get_project_wps(&self, project_id: &str) -> Vec<&WorkPackage> {
        self.work_packages.values().filter(|wp| wp.project_id == project_id).collect()
    }

    /// Get work packages for user
    pub fn get_user_wps(&self, user_id: &str) -> Vec<&WorkPackage> {
        self.work_packages.values().filter(|wp| wp.assignee.as_ref() == Some(user_id)).collect()
    }

    /// Get time entries for work package
    pub fn get_wp_time_entries(&self, wp_id: &str) -> Vec<&TimeEntry> {
        self.time_entries.iter().filter(|te| te.work_package_id == wp_id).collect()
    }

    /// Get tasks for work package
    pub fn get_wp_tasks(&self, wp_id: &str) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.work_package_id == wp_id).collect()
    }

    /// Get project by ID
    pub fn get_project(&self, id: &str) -> Option<&Project> {
        self.projects.get(id)
    }

    /// Get all projects
    pub fn get_all_projects(&self) -> Vec<&Project> {
        self.projects.values().collect()
    }

    /// Switch project
    pub fn switch_project(&mut self, id: &str) -> Result<(), String> {
        if self.projects.contains_key(id) {
            self.current_project = Some(id.to_string());
            Ok(())
        } else {
            Err("Project not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = ProjectManager::new();
    
    println!("Sigma Project Management v0.1 - OpenProject Style");
    
    loop {
        println!("\n--- Project Management Status ---");
        if let Some(proj_id) = &manager.current_project {
            if let Some(project) = manager.get_project(proj_id) {
                println!("Current Project: {} ({})", project.name, project.status);
                println!("Members: {}", project.members.len());
                println!("Work Packages: {}", manager.get_project_wps(proj_id).len());
            }
        }
        println!("Total Projects: {}", manager.projects.len());
        println!("Total Users: {}", manager.users.len());
        println!("Total Work Packages: {}", manager.work_packages.len());
        println!("Time Entries: {}", manager.time_entries.len());
        
        println!("\nCommands: create_user <name> <email> <role>, create_project <name> <description> <start> <end>, add_member <proj_id> <user_id>, create_wp <proj_id> <subject> <description> <priority> <assignee> <due_date> <est_hours>, update_wp <wp_id> <status>, log_time <wp_id> <user_id> <hours> <activity> <comments>, create_task <wp_id> <title> <due_date>, complete_task <task_id>, switch_project <id>, projects, wps <proj_id>, wps_user <user_id>, time_entries <wp_id>, tasks <wp_id>, quit");
        println!("WP priorities: low, normal, high, urgent");
        println!("WP statuses: new, in_progress, resolved, closed");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create_user" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let email = parts[2].to_string();
                    let role = parts[3].to_string();
                    let user = manager.create_user(name, email, role);
                    println!("User created: {}", user.name);
                }
            }
            "create_project" => {
                if parts.len() >= 5 {
                    let name = parts[1].to_string();
                    let description = parts[2].to_string();
                    let start_date = parts[3].to_string();
                    let end_date = parts[4].to_string();
                    let project = manager.create_project(name, description, start_date, end_date);
                    println!("Project created: {}", project.name);
                }
            }
            "add_member" => {
                if parts.len() >= 3 {
                    match manager.add_member(parts[1], parts[2]) {
                        Ok(_) => println!("Member added to project"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "create_wp" => {
                if parts.len() >= 8 {
                    let project_id = parts[1].to_string();
                    let subject = parts[2].to_string();
                    let description = parts[3].to_string();
                    let priority = parts[4].to_string();
                    let assignee = Some(parts[5].to_string());
                    let due_date = Some(parts[6].to_string());
                    if let Ok(est_hours) = parts[7].parse::<f64>() {
                        let wp = manager.create_work_package(&project_id, subject, description, priority, assignee, due_date, est_hours);
                        println!("Work package created: {}", wp.subject);
                    }
                }
            }
            "update_wp" => {
                if parts.len() >= 3 {
                    match manager.update_wp_status(parts[1], parts[2].to_string()) {
                        Ok(_) => println!("Work package status updated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "log_time" => {
                if parts.len() >= 6 {
                    if let Ok(hours) = parts[3].parse::<f64>() {
                        let comments = parts[5..].join(" ");
                        match manager.log_time(parts[1], parts[2], hours, parts[4].to_string(), comments) {
                            Ok(_) => println!("Time logged"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "create_task" => {
                if parts.len() >= 4 {
                    let due_date = Some(parts[3].to_string());
                    let task = manager.create_task(parts[1], parts[2].to_string(), due_date);
                    println!("Task created: {}", task.title);
                }
            }
            "complete_task" => {
                if let Some(arg) = parts.get(1) {
                    match manager.complete_task(arg) {
                        Ok(_) => println!("Task completed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "switch_project" => {
                if let Some(arg) = parts.get(1) {
                    match manager.switch_project(arg) {
                        Ok(_) => println!("Switched to project"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "projects" => {
                println!("--- All Projects ---");
                for project in manager.get_all_projects() {
                    println!("{} - {} ({}) [{}]", project.name, project.start_date, project.end_date, project.status);
                }
            }
            "wps" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Work Packages for Project ---");
                    for wp in manager.get_project_wps(arg) {
                        let assignee = wp.assignee.as_ref().map(|s| s.as_str()).unwrap_or("Unassigned");
                        println!("{} - {} ({}) [{}] - Assigned: {} - Est: {:.1}h, Spent: {:.1}h", 
                            wp.subject, wp.status, wp.priority, wp.due_date.as_ref().unwrap_or(&"No due date".to_string()), assignee, wp.estimated_hours, wp.spent_hours);
                    }
                }
            }
            "wps_user" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Work Packages for User ---");
                    for wp in manager.get_user_wps(arg) {
                        println!("{} - {} ({})", wp.subject, wp.status, wp.priority);
                    }
                }
            }
            "time_entries" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Time Entries ---");
                    for te in manager.get_wp_time_entries(arg) {
                        println!("{} - {}h - {} by {}", te.date, te.hours, te.activity, te.user_id);
                    }
                }
            }
            "tasks" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Tasks ---");
                    for task in manager.get_wp_tasks(arg) {
                        println!("{} - {} [{}]", task.title, if task.completed { "[DONE]" } else { "" }, task.due_date.as_ref().unwrap_or(&"No due date".to_string()));
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
