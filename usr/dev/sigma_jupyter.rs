// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/dev/sigma_jupyter.rs — Sigma JupyterLab Notebooks
//
// Implements JupyterLab-style notebook environment with cell execution,
// kernel management, file browsing, and notebook operations.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Jupyter Types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Code,
    Markdown,
    Raw,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellExecutionState {
    Idle,
    Running,
    Queued,
}

#[derive(Debug, Clone)]
pub struct NotebookCell {
    pub id: String,
    pub cell_type: CellType,
    pub content: String,
    pub execution_count: Option<u32>,
    pub output: Vec<CellOutput>,
    pub execution_state: CellExecutionState,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum CellOutput {
    Stream { name: String, text: String },
    DisplayData { data: HashMap<String, String> },
    ExecuteResult { data: HashMap<String, String>, execution_count: u32 },
    Error { ename: String, evalue: String, traceback: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct Notebook {
    pub id: String,
    pub name: String,
    pub path: String,
    pub kernel_name: String,
    pub cells: Vec<NotebookCell>,
    pub metadata: HashMap<String, String>,
    pub created: String,
    pub modified: String,
}

#[derive(Debug, Clone)]
pub struct Kernel {
    pub name: String,
    pub language: String,
    pub version: String,
    pub state: String,
    pub connections: u32,
}

#[derive(Debug, Clone)]
pub struct KernelSession {
    pub id: String,
    pub kernel_name: String,
    pub notebook_id: String,
    pub started: String,
    pub last_activity: String,
}

// ─── Jupyter Manager ─────────────────────────────────────────────────────

pub struct JupyterManager {
    pub notebooks: HashMap<String, Notebook>,
    pub kernels: HashMap<String, Kernel>,
    pub sessions: HashMap<String, KernelSession>,
    pub current_notebook: Option<String>,
    pub server_running: bool,
    pub server_port: u16,
}

impl JupyterManager {
    pub fn new() -> Self {
        let mut manager = JupyterManager {
            notebooks: HashMap::new(),
            kernels: HashMap::new(),
            sessions: HashMap::new(),
            current_notebook: None,
            server_running: true,
            server_port: 8888,
        };

        manager.init_default_kernels();
        manager
    }

    /// Initialize default kernels
    fn init_default_kernels(&mut self) {
        self.kernels.insert("python3".to_string(), Kernel {
            name: "python3".to_string(),
            language: "python".to_string(),
            version: "3.11.0".to_string(),
            state: "ready".to_string(),
            connections: 0,
        });

        self.kernels.insert("ir".to_string(), Kernel {
            name: "ir".to_string(),
            language: "R".to_string(),
            version: "4.3.0".to_string(),
            state: "ready".to_string(),
            connections: 0,
        });

        self.kernels.insert("julia".to_string(), Kernel {
            name: "julia".to_string(),
            language: "julia".to_string(),
            version: "1.9.0".to_string(),
            state: "ready".to_string(),
            connections: 0,
        });
    }

    /// Create a new notebook
    pub fn create_notebook(&mut self, name: String, path: String, kernel_name: String) -> Result<Notebook, String> {
        let notebook_id = format!("nb_{}", self.notebooks.len());
        
        if !self.kernels.contains_key(&kernel_name) {
            return Err("Kernel not found".to_string());
        }

        let notebook = Notebook {
            id: notebook_id.clone(),
            name: name.clone(),
            path,
            kernel_name: kernel_name.clone(),
            cells: vec![],
            metadata: HashMap::new(),
            created: "now".to_string(),
            modified: "now".to_string(),
        };

        self.notebooks.insert(notebook_id.clone(), notebook.clone());
        self.current_notebook = Some(notebook_id.clone());
        Ok(notebook)
    }

    /// Open a notebook
    pub fn open_notebook(&mut self, notebook_id: &str) -> Result<(), String> {
        if self.notebooks.contains_key(notebook_id) {
            self.current_notebook = Some(notebook_id.to_string());
            Ok(())
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Close a notebook
    pub fn close_notebook(&mut self, notebook_id: &str) -> Result<(), String> {
        if self.notebooks.contains_key(notebook_id) {
            if self.current_notebook.as_ref() == Some(&notebook_id.to_string()) {
                self.current_notebook = None;
            }
            Ok(())
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Delete a notebook
    pub fn delete_notebook(&mut self, notebook_id: &str) -> Result<(), String> {
        if self.notebooks.remove(notebook_id).is_some() {
            if self.current_notebook.as_ref() == Some(&notebook_id.to_string()) {
                self.current_notebook = None;
            }
            Ok(())
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Add a cell to notebook
    pub fn add_cell(&mut self, notebook_id: &str, cell_type: CellType, content: String) -> Result<NotebookCell, String> {
        if let Some(notebook) = self.notebooks.get_mut(notebook_id) {
            let cell = NotebookCell {
                id: format!("cell_{}", notebook.cells.len()),
                cell_type,
                content,
                execution_count: None,
                output: vec![],
                execution_state: CellExecutionState::Idle,
                metadata: HashMap::new(),
            };
            notebook.cells.push(cell.clone());
            notebook.modified = "now".to_string();
            Ok(cell)
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Execute a cell
    pub fn execute_cell(&mut self, notebook_id: &str, cell_id: &str) -> Result<(), String> {
        if let Some(notebook) = self.notebooks.get_mut(notebook_id) {
            if let Some(cell) = notebook.cells.iter_mut().find(|c| c.id == cell_id) {
                cell.execution_state = CellExecutionState::Running;
                
                // Simulate execution
                let execution_count = cell.execution_count.unwrap_or(0) + 1;
                cell.execution_count = Some(execution_count);
                
                // Generate output based on content
                if !cell.content.is_empty() {
                    cell.output.push(CellOutput::Stream {
                        name: "stdout".to_string(),
                        text: format!("Output for: {}", cell.content),
                    });
                }
                
                cell.execution_state = CellExecutionState::Idle;
                notebook.modified = "now".to_string();
                Ok(())
            } else {
                Err("Cell not found".to_string())
            }
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Delete a cell
    pub fn delete_cell(&mut self, notebook_id: &str, cell_id: &str) -> Result<(), String> {
        if let Some(notebook) = self.notebooks.get_mut(notebook_id) {
            if notebook.cells.iter().any(|c| c.id == cell_id) {
                notebook.cells.retain(|c| c.id != cell_id);
                notebook.modified = "now".to_string();
                Ok(())
            } else {
                Err("Cell not found".to_string())
            }
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Clear cell output
    pub fn clear_output(&mut self, notebook_id: &str, cell_id: &str) -> Result<(), String> {
        if let Some(notebook) = self.notebooks.get_mut(notebook_id) {
            if let Some(cell) = notebook.cells.iter_mut().find(|c| c.id == cell_id) {
                cell.output.clear();
                notebook.modified = "now".to_string();
                Ok(())
            } else {
                Err("Cell not found".to_string())
            }
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Start a kernel session
    pub fn start_session(&mut self, notebook_id: &str, kernel_name: &str) -> Result<KernelSession, String> {
        if !self.kernels.contains_key(kernel_name) {
            return Err("Kernel not found".to_string());
        }

        if !self.notebooks.contains_key(notebook_id) {
            return Err("Notebook not found".to_string());
        }

        let session_id = format!("session_{}", self.sessions.len());
        let session = KernelSession {
            id: session_id.clone(),
            kernel_name: kernel_name.to_string(),
            notebook_id: notebook_id.to_string(),
            started: "now".to_string(),
            last_activity: "now".to_string(),
        };

        if let Some(kernel) = self.kernels.get_mut(kernel_name) {
            kernel.connections += 1;
        }

        self.sessions.insert(session_id.clone(), session.clone());
        Ok(session)
    }

    /// Stop a kernel session
    pub fn stop_session(&mut self, session_id: &str) -> Result<(), String> {
        if let Some(session) = self.sessions.remove(session_id) {
            if let Some(kernel) = self.kernels.get_mut(&session.kernel_name) {
                kernel.connections = kernel.connections.saturating_sub(1);
            }
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    /// Get notebook by ID
    pub fn get_notebook(&self, notebook_id: &str) -> Option<&Notebook> {
        self.notebooks.get(notebook_id)
    }

    /// List all notebooks
    pub fn list_notebooks(&self) -> Vec<&Notebook> {
        self.notebooks.values().collect()
    }

    /// Get all kernels
    pub fn get_kernels(&self) -> Vec<&Kernel> {
        self.kernels.values().collect()
    }

    /// Get all sessions
    pub fn get_sessions(&self) -> Vec<&KernelSession> {
        self.sessions.values().collect()
    }

    /// Save notebook
    pub fn save_notebook(&mut self, notebook_id: &str) -> Result<(), String> {
        if let Some(notebook) = self.notebooks.get_mut(notebook_id) {
            notebook.modified = "now".to_string();
            Ok(())
        } else {
            Err("Notebook not found".to_string())
        }
    }

    /// Start server
    pub fn start_server(&mut self) {
        self.server_running = true;
    }

    /// Stop server
    pub fn stop_server(&mut self) {
        self.server_running = false;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("notebooks".to_string(), self.notebooks.len() as u32);
        stats.insert("kernels".to_string(), self.kernels.len() as u32);
        stats.insert("sessions".to_string(), self.sessions.len() as u32);
        stats.insert("total_cells".to_string(), self.notebooks.values().map(|n| n.cells.len() as u32).sum());
        stats.insert("server_running".to_string(), if self.server_running { 1 } else { 0 });
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut jupyter = JupyterManager::new();
    
    println!("Sigma JupyterLab v0.1 - Notebook Environment");
    
    loop {
        println!("\n--- Jupyter Commands ---");
        println!("notebooks         - List all notebooks");
        println!("create <name> <path> <kernel> - Create notebook");
        println!("open <id>         - Open notebook");
        println!("close <id>        - Close notebook");
        println!("delete <id>       - Delete notebook");
        println!("info <id>         - Show notebook details");
        println!("add_cell <type> <content> - Add cell (code, markdown, raw)");
        println!("execute <cell_id> - Execute cell");
        println!("delete_cell <cell_id> - Delete cell");
        println!("clear_output <cell_id> - Clear cell output");
        println!("kernels           - List available kernels");
        println!("start_session <nb_id> <kernel> - Start kernel session");
        println!("stop_session <session_id> - Stop kernel session");
        println!("sessions          - List active sessions");
        println!("save <id>         - Save notebook");
        println!("stats             - Show statistics");
        println!("start_server      - Start Jupyter server");
        println!("stop_server       - Stop Jupyter server");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "notebooks" => {
                println!("--- Notebooks ---");
                for notebook in jupyter.list_notebooks() {
                    println!("{} - {} - {} - {} cells", 
                        notebook.id, notebook.name, notebook.kernel_name, notebook.cells.len());
                }
            }
            "create" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let path = parts[2].to_string();
                    let kernel = parts[3].to_string();
                    match jupyter.create_notebook(name, path, kernel) {
                        Ok(nb) => println!("Notebook created: {}", nb.id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "open" => {
                if let Some(id) = parts.get(1) {
                    match jupyter.open_notebook(id) {
                        Ok(_) => println!("Notebook opened"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "close" => {
                if let Some(id) = parts.get(1) {
                    match jupyter.close_notebook(id) {
                        Ok(_) => println!("Notebook closed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete" => {
                if let Some(id) = parts.get(1) {
                    match jupyter.delete_notebook(id) {
                        Ok(_) => println!("Notebook deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "info" => {
                if let Some(id) = parts.get(1) {
                    if let Some(notebook) = jupyter.get_notebook(id) {
                        println!("--- Notebook Info ---");
                        println!("ID: {}", notebook.id);
                        println!("Name: {}", notebook.name);
                        println!("Path: {}", notebook.path);
                        println!("Kernel: {}", notebook.kernel_name);
                        println!("Cells: {}", notebook.cells.len());
                        println!("Created: {}", notebook.created);
                        println!("Modified: {}", notebook.modified);
                    }
                }
            }
            "add_cell" => {
                if parts.len() >= 3 {
                    let cell_type = match parts[1] {
                        "code" => CellType::Code,
                        "markdown" => CellType::Markdown,
                        "raw" => CellType::Raw,
                        _ => CellType::Code,
                    };
                    let content = parts[2..].join(" ");
                    if let Some(nb_id) = &jupyter.current_notebook {
                        match jupyter.add_cell(nb_id, cell_type, content) {
                            Ok(_) => println!("Cell added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        eprintln!("No notebook open");
                    }
                }
            }
            "execute" => {
                if let Some(cell_id) = parts.get(1) {
                    if let Some(nb_id) = &jupyter.current_notebook {
                        match jupyter.execute_cell(nb_id, cell_id) {
                            Ok(_) => println!("Cell executed"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        eprintln!("No notebook open");
                    }
                }
            }
            "delete_cell" => {
                if let Some(cell_id) = parts.get(1) {
                    if let Some(nb_id) = &jupyter.current_notebook {
                        match jupyter.delete_cell(nb_id, cell_id) {
                            Ok(_) => println!("Cell deleted"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        eprintln!("No notebook open");
                    }
                }
            }
            "clear_output" => {
                if let Some(cell_id) = parts.get(1) {
                    if let Some(nb_id) = &jupyter.current_notebook {
                        match jupyter.clear_output(nb_id, cell_id) {
                            Ok(_) => println!("Output cleared"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    } else {
                        eprintln!("No notebook open");
                    }
                }
            }
            "kernels" => {
                println!("--- Kernels ---");
                for kernel in jupyter.get_kernels() {
                    println!("{} - {} - {} - {} connections", 
                        kernel.name, kernel.language, kernel.version, kernel.connections);
                }
            }
            "start_session" => {
                if parts.len() >= 3 {
                    let nb_id = parts[1];
                    let kernel = parts[2];
                    match jupyter.start_session(nb_id, kernel) {
                        Ok(session) => println!("Session started: {}", session.id),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stop_session" => {
                if let Some(session_id) = parts.get(1) {
                    match jupyter.stop_session(session_id) {
                        Ok(_) => println!("Session stopped"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "sessions" => {
                println!("--- Sessions ---");
                for session in jupyter.get_sessions() {
                    println!("{} - {} - {}", session.id, session.kernel_name, session.notebook_id);
                }
            }
            "save" => {
                if let Some(id) = parts.get(1) {
                    match jupyter.save_notebook(id) {
                        Ok(_) => println!("Notebook saved"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in jupyter.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "start_server" => {
                jupyter.start_server();
                println!("Jupyter server started on port {}", jupyter.server_port);
            }
            "stop_server" => {
                jupyter.stop_server();
                println!("Jupyter server stopped");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
