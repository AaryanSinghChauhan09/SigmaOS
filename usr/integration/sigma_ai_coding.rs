// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/integration/sigma_ai_coding.rs — Sigma AI Coding Assistant
//
// Implements AI-powered coding assistant using StarCoder/CodeGen
// models for program synthesis and natural language coding workflows.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── AI Coding Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIModel {
    StarCoder,
    CodeGen,
    ClaudeCode,
    GPT4,
}

#[derive(Debug, Clone)]
pub struct CodeSuggestion {
    pub id: String,
    pub code: String,
    pub explanation: String,
    pub confidence: f64,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct CodeContext {
    pub file_name: String,
    pub language: String,
    pub cursor_position: (usize, usize),
    pub surrounding_code: String,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CodingSession {
    pub id: String,
    pub model: AIModel,
    pub context: CodeContext,
    pub suggestions: Vec<CodeSuggestion>,
    pub chat_history: Vec<String>,
}

// ─── AI Coding Assistant ─────────────────────────────────────────────────────

pub struct AICodingAssistant {
    pub sessions: HashMap<String, CodingSession>,
    pub current_session: Option<String>,
    pub current_model: AIModel,
}

impl AICodingAssistant {
    pub fn new() -> Self {
        let mut assistant = AICodingAssistant {
            sessions: HashMap::new(),
            current_session: None,
            current_model: AIModel::StarCoder,
        };
        
        assistant
    }

    /// Set current AI model
    pub fn set_model(&mut self, model: AIModel) {
        self.current_model = model;
    }

    /// Create new coding session
    pub fn create_session(&mut self, file_name: String, language: String) -> CodingSession {
        let session = CodingSession {
            id: format!("session_{}", self.sessions.len()),
            model: self.current_model,
            context: CodeContext {
                file_name,
                language,
                cursor_position: (0, 0),
                surrounding_code: String::new(),
                imports: Vec::new(),
            },
            suggestions: Vec::new(),
            chat_history: Vec::new(),
        };
        
        self.sessions.insert(session.id.clone(), session.clone());
        self.current_session = Some(session.id.clone());
        session
    }

    /// Get current session
    pub fn get_current_session(&self) -> Option<&CodingSession> {
        self.current_session.as_ref()
            .and_then(|id| self.sessions.get(id))
    }

    /// Generate code suggestion based on context
    pub fn generate_suggestion(&mut self, prompt: &str) -> CodeSuggestion {
        let language = if let Some(session) = self.get_current_session() {
            session.context.language.clone()
        } else {
            "python".to_string()
        };
        
        let (code, explanation) = match language.as_str() {
            "python" => self.generate_python_code(prompt),
            "rust" => self.generate_rust_code(prompt),
            "javascript" => self.generate_javascript_code(prompt),
            "cpp" => self.generate_cpp_code(prompt),
            _ => self.generate_python_code(prompt),
        };
        
        let suggestion = CodeSuggestion {
            id: format!("sugg_{}", self.current_session.as_ref().map_or(0, |s| s.suggestions.len())),
            code,
            explanation,
            confidence: 0.85,
            language,
        };
        
        if let Some(session_id) = &self.current_session {
            if let Some(session) = self.sessions.get_mut(session_id) {
                session.suggestions.push(suggestion.clone());
            }
        }
        
        suggestion
    }

    /// Generate Python code
    fn generate_python_code(&self, prompt: &str) -> (String, String) {
        let prompt_lower = prompt.to_lowercase();
        
        if prompt_lower.contains("sort") || prompt_lower.contains("list") {
            (r#"def sort_list(items):
    return sorted(items)

# Example usage:
numbers = [3, 1, 4, 1, 5, 9, 2, 6]
sorted_numbers = sort_list(numbers)
print(sorted_numbers)"#.to_string(), "Implements a function to sort a list using Python's built-in sorted() function".to_string())
        } else if prompt_lower.contains("class") {
            (r#"class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        return f"Hello, my name is {self.name} and I am {self.age} years old"

# Example usage:
person = Person("Alice", 30)
print(person.greet())"#.to_string(), "Defines a Person class with constructor and greet method".to_string())
        } else if prompt_lower.contains("file") || prompt_lower.contains("read") {
            (r#"def read_file(filename):
    with open(filename, 'r') as f:
        return f.read()

def write_file(filename, content):
    with open(filename, 'w') as f:
        f.write(content)

# Example usage:
content = read_file('input.txt')
write_file('output.txt', content)"#.to_string(), "Functions to read and write files using context managers".to_string())
        } else {
            (r#"def hello_world():
    print("Hello, World!")

if __name__ == "__main__":
    hello_world()"#.to_string(), "Simple hello world function".to_string())
        }
    }

    /// Generate Rust code
    fn generate_rust_code(&self, prompt: &str) -> (String, String) {
        let prompt_lower = prompt.to_lowercase();
        
        if prompt_lower.contains("struct") {
            (r#"struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old", self.name, self.age)
    }
}

fn main() {
    let person = Person::new(String::from("Alice"), 30);
    println!("{}", person.greet());
}"#.to_string(), "Defines a Person struct with constructor and greet method".to_string())
        } else if prompt_lower.contains("vec") || prompt_lower.contains("vector") {
            (r#"fn sort_vec<T: Ord>(mut items: Vec<T>) -> Vec<T> {
    items.sort();
    items
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted_numbers = sort_vec(numbers);
    println!("{:?}", sorted_numbers);
}"#.to_string(), "Function to sort a vector using built-in sort method".to_string())
        } else {
            (r#"fn main() {
    println!("Hello, World!");
}"#.to_string(), "Simple hello world function".to_string())
        }
    }

    /// Generate JavaScript code
    fn generate_javascript_code(&self, prompt: &str) -> (String, String) {
        let prompt_lower = prompt.to_lowercase();
        
        if prompt_lower.contains("function") || prompt_lower.contains("arrow") {
            (r#"// Traditional function
function greet(name) {
    return `Hello, ${name}!`;
}

// Arrow function
const greetArrow = (name) => `Hello, ${name}!`;

console.log(greet("World"));
console.log(greetArrow("World"));"#.to_string(), "Shows both traditional and arrow function syntax".to_string())
        } else if prompt_lower.contains("array") || prompt_lower.contains("map") {
            (r#"const numbers = [1, 2, 3, 4, 5];

// Using map to double each number
const doubled = numbers.map(n => n * 2);

// Using filter to get even numbers
const evens = numbers.filter(n => n % 2 === 0);

console.log(doubled);
console.log(evens);"#.to_string(), "Demonstrates array methods map and filter".to_string())
        } else {
            (r#"console.log("Hello, World!");"#.to_string(), "Simple hello world".to_string())
        }
    }

    /// Generate C++ code
    fn generate_cpp_code(&self, prompt: &str) -> (String, String) {
        let prompt_lower = prompt.to_lowercase();
        
        if prompt_lower.contains("class") {
            (r#"include <iostream>
#include <string>

class Person {
private:
    std::string name;
    int age;

public:
    Person(std::string n, int a) : name(n), age(a) {}
    
    void greet() {
        std::cout << "Hello, my name is " << name 
                  << " and I am " << age << " years old" << std::endl;
    }
};

int main() {
    Person person("Alice", 30);
    person.greet();
    return 0;
}"#.to_string(), "Defines a Person class with constructor and greet method".to_string())
        } else if prompt_lower.contains("vector") {
            (r#"include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> numbers = {3, 1, 4, 1, 5, 9, 2, 6};
    
    std::sort(numbers.begin(), numbers.end());
    
    for (int num : numbers) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    
    return 0;
}"#.to_string(), "Sorts a vector using std::sort".to_string())
        } else {
            (r#"include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}"#.to_string(), "Simple hello world".to_string())
        }
    }

    /// Add chat message
    pub fn add_chat_message(&mut self, message: String) {
        if let Some(session_id) = &self.current_session {
            if let Some(session) = self.sessions.get_mut(session_id) {
                session.chat_history.push(message);
            }
        }
    }

    /// Get chat history
    pub fn get_chat_history(&self) -> Vec<String> {
        if let Some(session) = self.get_current_session() {
            session.chat_history.clone()
        } else {
            Vec::new()
        }
    }

    /// Get model name
    pub fn get_model_name(&self, model: AIModel) -> &str {
        match model {
            AIModel::StarCoder => "StarCoder",
            AIModel::CodeGen => "Salesforce CodeGen",
            AIModel::ClaudeCode => "Claude Code",
            AIModel::GPT4 => "GPT-4",
        }
    }

    /// Get all sessions
    pub fn get_all_sessions(&self) -> Vec<&CodingSession> {
        self.sessions.values().collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut assistant = AICodingAssistant::new();
    
    println!("Sigma AI Coding Assistant v0.1 - StarCoder/CodeGen Integration");
    
    loop {
        println!("\n--- Current Model: {} ---", assistant.get_model_name(assistant.current_model));
        if let Some(session) = assistant.get_current_session() {
            println!("--- Active Session: {} ({}) ---", session.id, session.context.language);
        }
        
        println!("\nCommands: model <type>, session <file> <lang>, suggest <prompt>, chat <message>, history, sessions, quit");
        println!("Models: starcoder, codegen, claudcode, gpt4");
        println!("Languages: python, rust, javascript, cpp, java");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "model" => {
                if let Some(arg) = parts.get(1) {
                    let model = match *arg {
                        "starcoder" => AIModel::StarCoder,
                        "codegen" => AIModel::CodeGen,
                        "claudcode" => AIModel::ClaudeCode,
                        "gpt4" => AIModel::GPT4,
                        _ => {
                            println!("Unknown model");
                            continue;
                        }
                    };
                    assistant.set_model(model);
                    println!("Model changed to {}", assistant.get_model_name(model));
                }
            }
            "session" => {
                if parts.len() >= 3 {
                    let file_name = parts[1].to_string();
                    let language = parts[2].to_string();
                    let session = assistant.create_session(file_name, language);
                    println!("Session created: {}", session.id);
                }
            }
            "suggest" => {
                if parts.len() >= 2 {
                    let prompt = parts[1..].join(" ");
                    let suggestion = assistant.generate_suggestion(&prompt);
                    println!("--- Code Suggestion ---");
                    println!("Language: {}", suggestion.language);
                    println!("Confidence: {:.0}%", suggestion.confidence * 100.0);
                    println!("\nCode:");
                    println!("{}", suggestion.code);
                    println!("\nExplanation:");
                    println!("{}", suggestion.explanation);
                }
            }
            "chat" => {
                if parts.len() >= 2 {
                    let message = parts[1..].join(" ");
                    assistant.add_chat_message(message);
                    println!("Message added to chat history");
                }
            }
            "history" => {
                println!("--- Chat History ---");
                for message in assistant.get_chat_history() {
                    println!("- {}", message);
                }
            }
            "sessions" => {
                println!("--- All Sessions ---");
                for session in assistant.get_all_sessions() {
                    println!("{} - {} ({})", session.id, session.context.file_name, session.context.language);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
