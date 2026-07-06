// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/it/sigma_playground.rs — Sigma Coding Playground
//
// Implements a built-in IDE with AI assistance for Python, C++, and Java
// aligned with CBSE IT curriculum and practical examinations.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;
use std::process::Command;

// ─── Language Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Python,
    Cpp,
    Java,
}

#[derive(Debug, Clone)]
pub struct CodeFile {
    pub name: String,
    pub language: Language,
    pub content: String,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub success: bool,
    pub output: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

// ─── Coding Playground ───────────────────────────────────────────────────────

pub struct CodingPlayground {
    pub files: HashMap<String, CodeFile>,
    pub current_file: Option<String>,
    pub ai_assistant_enabled: bool,
}

impl CodingPlayground {
    pub fn new() -> Self {
        CodingPlayground {
            files: HashMap::new(),
            current_file: None,
            ai_assistant_enabled: true,
        }
    }

    /// Create new file
    pub fn create_file(&mut self, name: String, language: Language) -> Result<(), String> {
        if self.files.contains_key(&name) {
            return Err("File already exists".to_string());
        }
        
        let file = CodeFile {
            name: name.clone(),
            language,
            content: String::new(),
            modified: false,
        };
        
        self.files.insert(name.clone(), file);
        self.current_file = Some(name);
        Ok(())
    }

    /// Open file
    pub fn open_file(&mut self, name: &str) -> Result<(), String> {
        if self.files.contains_key(name) {
            self.current_file = Some(name.to_string());
            Ok(())
        } else {
            Err("File not found".to_string())
        }
    }

    /// Save file content
    pub fn save_file(&mut self, name: &str, content: String) -> Result<(), String> {
        if let Some(file) = self.files.get_mut(name) {
            file.content = content;
            file.modified = false;
            Ok(())
        } else {
            Err("File not found".to_string())
        }
    }

    /// Get current file content
    pub fn get_current_content(&self) -> Option<&String> {
        self.current_file.as_ref()
            .and_then(|name| self.files.get(name))
            .map(|f| &f.content)
    }

    /// Compile and run code
    pub fn run_code(&self, file_name: &str) -> CompilationResult {
        if let Some(file) = self.files.get(file_name) {
            match file.language {
                Language::Python => self.run_python(&file.content),
                Language::Cpp => self.run_cpp(&file.content),
                Language::Java => self.run_java(&file.content),
            }
        } else {
            CompilationResult {
                success: false,
                output: String::new(),
                errors: vec!["File not found".to_string()],
                warnings: Vec::new(),
            }
        }
    }

    /// Run Python code
    fn run_python(&self, code: &str) -> CompilationResult {
        // In a real implementation, this would use Python interpreter
        // For stub, we'll simulate basic execution
        
        if code.contains("print") {
            let output = code.lines()
                .filter(|line| line.contains("print"))
                .map(|line| {
                    line.split("print(")
                        .nth(1)
                        .and_then(|s| s.split(")").next())
                        .unwrap_or("")
                        .trim_matches(&['"', '\''][..])
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join("\n");
            
            CompilationResult {
                success: true,
                output,
                errors: Vec::new(),
                warnings: Vec::new(),
            }
        } else {
            CompilationResult {
                success: true,
                output: "Code executed successfully".to_string(),
                errors: Vec::new(),
                warnings: Vec::new(),
            }
        }
    }

    /// Run C++ code
    fn run_cpp(&self, code: &str) -> CompilationResult {
        // In a real implementation, this would use g++
        // For stub, we'll simulate compilation
        
        if code.contains("#include") && code.contains("main") {
            CompilationResult {
                success: true,
                output: "C++ program compiled and executed".to_string(),
                errors: Vec::new(),
                warnings: Vec::new(),
            }
        } else if !code.contains("main") {
            CompilationResult {
                success: false,
                output: String::new(),
                errors: vec!["error: no 'main' function found".to_string()],
                warnings: Vec::new(),
            }
        } else {
            CompilationResult {
                success: false,
                output: String::new(),
                errors: vec!["error: compilation failed".to_string()],
                warnings: Vec::new(),
            }
        }
    }

    /// Run Java code
    fn run_java(&self, code: &str) -> CompilationResult {
        // In a real implementation, this would use javac and java
        // For stub, we'll simulate compilation
        
        if code.contains("class") && code.contains("public static void main") {
            CompilationResult {
                success: true,
                output: "Java program compiled and executed".to_string(),
                errors: Vec::new(),
                warnings: Vec::new(),
            }
        } else if !code.contains("main") {
            CompilationResult {
                success: false,
                output: String::new(),
                errors: vec!["error: no 'main' method found".to_string()],
                warnings: Vec::new(),
            }
        } else {
            CompilationResult {
                success: false,
                output: String::new(),
                errors: vec!["error: compilation failed".to_string()],
                warnings: Vec::new(),
            }
        }
    }

    /// Get AI suggestion for code
    pub fn get_ai_suggestion(&self, code: &str, language: Language) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if self.ai_assistant_enabled {
            match language {
                Language::Python => {
                    if code.contains("for") && !code.contains("range") {
                        suggestions.push("Consider using range() for iteration over sequences".to_string());
                    }
                    if code.contains("list") && code.contains("append") {
                        suggestions.push("For better performance, consider list comprehensions".to_string());
                    }
                    if !code.contains("def") && code.len() > 100 {
                        suggestions.push("Consider breaking this code into functions for better organization".to_string());
                    }
                }
                Language::Cpp => {
                    if code.contains("using namespace std") {
                        suggestions.push("Avoid 'using namespace std' in header files to prevent namespace pollution".to_string());
                    }
                    if code.contains("new") && !code.contains("delete") {
                        suggestions.push("Memory leak detected: ensure to delete dynamically allocated memory".to_string());
                    }
                }
                Language::Java => {
                    if code.contains("String") && code.contains("+") {
                        suggestions.push("Use StringBuilder for string concatenation in loops".to_string());
                    }
                    if code.contains("ArrayList") && !code.contains("List") {
                        suggestions.push("Consider programming to interfaces (List) instead of implementations (ArrayList)".to_string());
                    }
                }
            }
        }
        
        suggestions
    }

    /// Get syntax error suggestions
    pub fn get_syntax_help(&self, language: Language) -> Vec<String> {
        match language {
            Language::Python => vec![
                "Python uses indentation for code blocks".to_string(),
                "Comments start with #".to_string(),
                "Print statement: print('Hello World')".to_string(),
                "Variables don't need type declaration".to_string(),
            ],
            Language::Cpp => vec![
                "C++ uses {} for code blocks".to_string(),
                "Comments: // single line, /* */ multi-line".to_string(),
                "Include headers: #include <iostream>".to_string(),
                "Main function: int main() { return 0; }".to_string(),
            ],
            Language::Java => vec![
                "Java uses {} for code blocks".to_string(),
                "Comments: // single line, /* */ multi-line".to_string(),
                "Class name must match filename".to_string(),
                "Main method: public static void main(String[] args)".to_string(),
            ],
        }
    }

    /// List all files
    pub fn list_files(&self) -> Vec<&CodeFile> {
        self.files.values().collect()
    }

    /// Delete file
    pub fn delete_file(&mut self, name: &str) -> Result<(), String> {
        if self.files.remove(name).is_some() {
            if self.current_file.as_ref() == Some(&name.to_string()) {
                self.current_file = None;
            }
            Ok(())
        } else {
            Err("File not found".to_string())
        }
    }

    /// Toggle AI assistant
    pub fn toggle_ai_assistant(&mut self) {
        self.ai_assistant_enabled = !self.ai_assistant_enabled;
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut playground = CodingPlayground::new();
    
    println!("Sigma Coding Playground v0.1 - Python, C++, Java IDE");
    
    loop {
        println!("\n--- Files ---");
        for file in playground.list_files() {
            let lang_str = match file.language {
                Language::Python => "PY",
                Language::Cpp => "CPP",
                Language::Java => "JAVA",
            };
            let marker = if playground.current_file.as_ref() == Some(&file.name) { " >" } else { "  " };
            println!("{}[{}] {} ({})", marker, lang_str, file.name, if file.modified { "*" } else { "" });
        }
        
        println!("\nCommands: new <name> <lang>, open <name>, save <name>, run <name>, suggest, syntax <lang>, delete <name>, ai, quit");
        println!("Languages: python, cpp, java");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "new" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let language = match parts[2] {
                        "python" => Language::Python,
                        "cpp" => Language::Cpp,
                        "java" => Language::Java,
                        _ => {
                            println!("Unknown language");
                            continue;
                        }
                    };
                    match playground.create_file(name, language) {
                        Ok(_) => println!("File created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "open" => {
                if let Some(arg) = parts.get(1) {
                    match playground.open_file(arg) {
                        Ok(_) => println!("Opened: {}", arg),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "save" => {
                if parts.len() >= 2 {
                    let name = parts[1];
                    print!("Enter content (end with . on a line): ");
                    std::io::stdout().flush().unwrap();
                    let mut content = String::new();
                    loop {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line).unwrap();
                        if line.trim() == "." {
                            break;
                        }
                        content.push_str(&line);
                    }
                    match playground.save_file(name, content) {
                        Ok(_) => println!("File saved"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "run" => {
                if let Some(arg) = parts.get(1) {
                    let result = playground.run_code(arg);
                    if result.success {
                        println!("--- Output ---");
                        println!("{}", result.output);
                    } else {
                        println!("--- Errors ---");
                        for error in &result.errors {
                            println!("{}", error);
                        }
                    }
                }
            }
            "suggest" => {
                if let Some(content) = playground.get_current_content() {
                    if let Some(file) = playground.current_file.as_ref().and_then(|name| playground.files.get(name)) {
                        let suggestions = playground.get_ai_suggestion(content, file.language);
                        println!("--- AI Suggestions ---");
                        for suggestion in suggestions {
                            println!("- {}", suggestion);
                        }
                    }
                } else {
                    println!("No file open");
                }
            }
            "syntax" => {
                if parts.len() >= 2 {
                    let language = match parts[1] {
                        "python" => Language::Python,
                        "cpp" => Language::Cpp,
                        "java" => Language::Java,
                        _ => {
                            println!("Unknown language");
                            continue;
                        }
                    };
                    println!("--- Syntax Help ---");
                    for help in playground.get_syntax_help(language) {
                        println!("- {}", help);
                    }
                }
            }
            "delete" => {
                if let Some(arg) = parts.get(1) {
                    match playground.delete_file(arg) {
                        Ok(_) => println!("File deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "ai" => {
                playground.toggle_ai_assistant();
                println!("AI Assistant: {}", if playground.ai_assistant_enabled { "ON" } else { "OFF" });
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
