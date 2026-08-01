// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell for SigmaOS

use std::io::{self, BufRead, Write};

/// Shell command type
#[derive(Debug, Clone)]
pub enum ShellCommand {
    Help,
    ListProcesses,
    ListFiles,
    Exit,
    Echo {
        message: String,
    },
    Set {
        variable: String,
        value: String,
    },
    Get {
        variable: String,
    },
    Alias {
        name: String,
        value: String,
    },
    Unalias {
        name: String,
    },
    Run {
        variable: String,
    },
    AgentList,
    AgentRegister {
        description: String,
        commands: String,
    },
    AgentRun {
        task_id: usize,
    },
    Unknown(String),
}

/// Represents an automated action task executed by an AI agent
#[derive(Debug, Clone)]
pub struct AgentTask {
    pub task_id: usize,
    pub description: String,
    pub commands: Vec<String>,
}

/// AI Agent Automation Engine inside SigmaOS REPL
#[derive(Debug, Clone)]
pub struct AgentAutomationEngine {
    pub registered_tasks: std::collections::HashMap<usize, AgentTask>,
    pub next_task_id: usize,
}

impl AgentAutomationEngine {
    pub fn new() -> Self {
        AgentAutomationEngine {
            registered_tasks: std::collections::HashMap::new(),
            next_task_id: 1,
        }
    }

    pub fn register_task(&mut self, description: String, commands: Vec<String>) -> usize {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.registered_tasks.insert(
            id,
            AgentTask {
                task_id: id,
                description,
                commands,
            },
        );
        id
    }
}

impl Default for AgentAutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Shell REPL
pub struct ShellRepl {
    running: bool,
    variables: std::collections::HashMap<String, String>,
    aliases: std::collections::HashMap<String, String>,
    prompt: String,
    agent_engine: AgentAutomationEngine,
}

impl ShellRepl {
    pub fn new() -> Self {
        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            prompt: "sigma-sh> ".to_string(),
            agent_engine: AgentAutomationEngine::new(),
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            prompt,
            agent_engine: AgentAutomationEngine::new(),
        }
    }

    pub fn run(&mut self) {
        println!("SigmaOS Shell v0.1.0");
        println!("Type 'help' for available commands\n");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        while self.running {
            print!("{}", self.prompt);
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();

            let input = input.trim();
            if !input.is_empty() {
                self.execute_line(input);
            }
        }

        println!("Goodbye!");
    }

    pub fn execute_line(&mut self, line: &str) {
        if line.contains(';') {
            let subcommands: Vec<&str> = line.split(';').collect();
            for sub in subcommands {
                let trimmed = sub.trim();
                if !trimmed.is_empty() {
                    self.execute_single_line(trimmed);
                }
            }
        } else {
            self.execute_single_line(line);
        }
    }

    fn execute_single_line(&mut self, line: &str) {
        let command = self.parse_command(line);
        let result = self.execute_command(command);

        match result {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }

    pub fn parse_command(&self, input: &str) -> ShellCommand {
        let mut expanded_input = input.to_string();
        let first_word = input.split_whitespace().next().unwrap_or("");
        if let Some(alias_value) = self.aliases.get(first_word) {
            let rest = if input.len() > first_word.len() {
                &input[first_word.len()..]
            } else {
                ""
            };
            expanded_input = format!("{}{}", alias_value, rest);
        }

        let parts: Vec<&str> = expanded_input.split_whitespace().collect();

        if parts.is_empty() {
            return ShellCommand::Unknown(input.to_string());
        }

        match parts[0] {
            "help" => ShellCommand::Help,
            "ps" => ShellCommand::ListProcesses,
            "ls" => ShellCommand::ListFiles,
            "exit" | "quit" => ShellCommand::Exit,
            "echo" => {
                let message = parts[1..].join(" ");
                ShellCommand::Echo { message }
            }
            "set" => {
                if parts.len() >= 3 {
                    ShellCommand::Set {
                        variable: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "get" => {
                if parts.len() >= 2 {
                    ShellCommand::Get {
                        variable: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "alias" => {
                if parts.len() >= 3 {
                    ShellCommand::Alias {
                        name: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "unalias" => {
                if parts.len() >= 2 {
                    ShellCommand::Unalias {
                        name: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "run" | "exec" => {
                if parts.len() >= 2 {
                    ShellCommand::Run {
                        variable: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "agent" => {
                if parts.len() >= 2 {
                    match parts[1] {
                        "list" => ShellCommand::AgentList,
                        "run" => {
                            if parts.len() >= 3 {
                                if let Ok(id) = parts[2].parse::<usize>() {
                                    ShellCommand::AgentRun { task_id: id }
                                } else {
                                    ShellCommand::Unknown(input.to_string())
                                }
                            } else {
                                ShellCommand::Unknown(input.to_string())
                            }
                        }
                        "register" => {
                            if parts.len() >= 4 {
                                let desc = parts[2].to_string();
                                let cmds = parts[3..].join(" ");
                                ShellCommand::AgentRegister {
                                    description: desc,
                                    commands: cmds,
                                }
                            } else {
                                ShellCommand::Unknown(input.to_string())
                            }
                        }
                        _ => ShellCommand::Unknown(input.to_string()),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            _ => ShellCommand::Unknown(input.to_string()),
        }
    }

    pub fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
        match command {
            ShellCommand::Help => Ok("Available commands:\n\
                   help    - Show this help message\n\
                   ps      - List running processes\n\
                   ls      - List files\n\
                   echo    - Print a message\n\
                   set     - Set a variable\n\
                   get     - Get a variable\n\
                   alias   - Create a command shortcut/alias\n\
                   unalias - Remove an alias\n\
                   run     - Execute an automated macro/script variable\n\
                   agent   - Interface for AI Agent Automation tasks (register, list, run)\n\
                   exit    - Exit the shell"
                .to_string()),
            ShellCommand::ListProcesses => Ok("PID  NAME        STATE\n\
                   1    sigma-sh    Running\n\
                   2    kernel      Running"
                .to_string()),
            ShellCommand::ListFiles => Ok("README.md\n\
                   Cargo.toml\n\
                   src/\n\
                   tests/"
                .to_string()),
            ShellCommand::Exit => {
                self.running = false;
                Ok(String::new())
            }
            ShellCommand::Echo { message } => Ok(message),
            ShellCommand::Set { variable, value } => {
                self.variables.insert(variable.clone(), value.clone());
                Ok(format!("{} = {}", variable, value))
            }
            ShellCommand::Get { variable } => match self.variables.get(&variable) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Variable '{}' not found", variable)),
            },
            ShellCommand::Alias { name, value } => {
                self.aliases.insert(name.clone(), value.clone());
                Ok(format!("alias {} = {}", name, value))
            }
            ShellCommand::Unalias { name } => {
                if self.aliases.remove(&name).is_some() {
                    Ok(format!("Removed alias {}", name))
                } else {
                    Err(format!("Alias '{}' not found", name))
                }
            }
            ShellCommand::Run { variable } => {
                if let Some(val) = self.variables.get(&variable).cloned() {
                    self.execute_line(&val);
                    Ok(format!("Executed macro '{}'", variable))
                } else {
                    Err(format!("Variable/Macro '{}' not found", variable))
                }
            }
            ShellCommand::AgentRegister {
                description,
                commands,
            } => {
                let cmd_list: Vec<String> =
                    commands.split(';').map(|s| s.trim().to_string()).collect();
                let id = self
                    .agent_engine
                    .register_task(description.clone(), cmd_list);
                Ok(format!(
                    "Agent task #{} registered successfully: {}",
                    id, description
                ))
            }
            ShellCommand::AgentList => {
                if self.agent_engine.registered_tasks.is_empty() {
                    Ok("No agent automation tasks registered.".to_string())
                } else {
                    let mut list_str = "Registered Agent Automation Tasks:\n".to_string();
                    for (id, task) in &self.agent_engine.registered_tasks {
                        list_str.push_str(&format!(
                            "  [#{}] {} (Commands: {})\n",
                            id,
                            task.description,
                            task.commands.join("; ")
                        ));
                    }
                    Ok(list_str)
                }
            }
            ShellCommand::AgentRun { task_id } => {
                if let Some(task) = self.agent_engine.registered_tasks.get(&task_id).cloned() {
                    let mut result_str = format!("[Agent Automation Run #{}]\n", task_id);
                    result_str.push_str(&format!("Task Description: {}\n", task.description));
                    result_str.push_str("-----------------------------\n");
                    for (idx, cmd) in task.commands.iter().enumerate() {
                        result_str.push_str(&format!("Step {}: Executing '{}'...\n", idx + 1, cmd));
                        self.execute_line(cmd);
                    }
                    result_str.push_str("-----------------------------\n");
                    result_str.push_str("[Agent Automation Complete: Success]");
                    Ok(result_str)
                } else {
                    Err(format!("Agent task #{} not found", task_id))
                }
            }
            ShellCommand::Unknown(cmd) => Err(format!("Unknown command: {}", cmd)),
        }
    }
}

impl Default for ShellRepl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = ShellRepl::new();
        assert!(repl.running);
        assert_eq!(repl.prompt, "sigma-sh> ");
    }

    #[test]
    fn test_parse_help() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("help");
        assert!(matches!(command, ShellCommand::Help));
    }

    #[test]
    fn test_parse_echo() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("echo hello world");
        assert!(matches!(command, ShellCommand::Echo { .. }));
    }

    #[test]
    fn test_execute_echo() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Echo {
            message: "test".to_string(),
        };
        let result = repl.execute_command(command);
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_set_get_variable() {
        let mut repl = ShellRepl::new();
        let set_cmd = ShellCommand::Set {
            variable: "test".to_string(),
            value: "value".to_string(),
        };
        repl.execute_command(set_cmd).unwrap();

        let get_cmd = ShellCommand::Get {
            variable: "test".to_string(),
        };
        let result = repl.execute_command(get_cmd);
        assert_eq!(result.unwrap(), "value");
    }

    #[test]
    fn test_exit() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Exit;
        repl.execute_command(command).unwrap();
        assert!(!repl.running);
    }

    #[test]
    fn test_alias_unalias() {
        let mut repl = ShellRepl::new();
        let alias_cmd = ShellCommand::Alias {
            name: "l".to_string(),
            value: "ls".to_string(),
        };
        repl.execute_command(alias_cmd).unwrap();

        let parsed = repl.parse_command("l");
        assert!(matches!(parsed, ShellCommand::ListFiles));

        let unalias_cmd = ShellCommand::Unalias {
            name: "l".to_string(),
        };
        repl.execute_command(unalias_cmd).unwrap();

        let parsed_after = repl.parse_command("l");
        assert!(matches!(parsed_after, ShellCommand::Unknown(..)));
    }

    #[test]
    fn test_macro_automation() {
        let mut repl = ShellRepl::new();
        let set_cmd = ShellCommand::Set {
            variable: "test_macro".to_string(),
            value: "echo running; ls".to_string(),
        };
        repl.execute_command(set_cmd).unwrap();

        let run_cmd = ShellCommand::Run {
            variable: "test_macro".to_string(),
        };
        let result = repl.execute_command(run_cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_agent_automation() {
        let mut repl = ShellRepl::new();

        // 1. Register an Agent Task
        let reg_cmd = ShellCommand::AgentRegister {
            description: "SysAudit".to_string(),
            commands: "echo audit_start; ps; echo audit_end".to_string(),
        };
        let reg_res = repl.execute_command(reg_cmd).unwrap();
        assert!(reg_res.contains("Agent task #1 registered successfully"));

        // 2. List registered tasks
        let list_cmd = ShellCommand::AgentList;
        let list_res = repl.execute_command(list_cmd).unwrap();
        assert!(list_res.contains("SysAudit"));

        // 3. Run the Agent Task
        let run_cmd = ShellCommand::AgentRun { task_id: 1 };
        let run_res = repl.execute_command(run_cmd).unwrap();
        assert!(run_res.contains("[Agent Automation Run #1]"));
        assert!(run_res.contains("[Agent Automation Complete: Success]"));
    }
}
