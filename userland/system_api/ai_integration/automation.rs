// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Automation Engine - Natural language to script automation

use serde::{Deserialize, Serialize};

/// Automation Engine for natural language to script conversion
pub struct AutomationEngine {
    workflows: Vec<Workflow>,
    templates: Vec<WorkflowTemplate>,
}

impl AutomationEngine {
    /// Create a new Automation Engine
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            workflows: Vec::new(),
            templates: Self::load_templates(),
        })
    }

    /// Load workflow templates
    fn load_templates() -> Vec<WorkflowTemplate> {
        vec![
            WorkflowTemplate {
                id: "python_project".to_string(),
                name: "Python Project Setup".to_string(),
                description: "Create a new Python project with standard structure".to_string(),
                natural_language_patterns: vec!["create python project".to_string(), "setup python".to_string()],
                steps: vec![
                    WorkflowStep {
                        action: "create_directory".to_string(),
                        parameters: vec!["project_name".to_string()],
                    },
                    WorkflowStep {
                        action: "create_file".to_string(),
                        parameters: vec!["README.md".to_string()],
                    },
                    WorkflowStep {
                        action: "create_file".to_string(),
                        parameters: vec!["requirements.txt".to_string()],
                    },
                    WorkflowStep {
                        action: "initialize_git".to_string(),
                        parameters: vec![],
                    },
                    WorkflowStep {
                        action: "create_virtualenv".to_string(),
                        parameters: vec![],
                    },
                ],
            },
            WorkflowTemplate {
                id: "kubernetes_setup".to_string(),
                name: "Kubernetes Development Setup".to_string(),
                description: "Set up Kubernetes development environment".to_string(),
                natural_language_patterns: vec!["setup kubernetes".to_string(), "install k8s".to_string()],
                steps: vec![
                    WorkflowStep {
                        action: "install_docker".to_string(),
                        parameters: vec![],
                    },
                    WorkflowStep {
                        action: "install_kubectl".to_string(),
                        parameters: vec![],
                    },
                    WorkflowStep {
                        action: "install_minikube".to_string(),
                        parameters: vec![],
                    },
                    WorkflowStep {
                        action: "start_cluster".to_string(),
                        parameters: vec![],
                    },
                ],
            },
        ]
    }

    /// Create a workflow from natural language
    pub fn create_workflow(&mut self, parameters: &[String]) -> Result<super::AIResponse, Box<dyn std::error::Error>> {
        if let Some(command) = parameters.first() {
            let template = self.find_template(command);
            
            if let Some(tmpl) = template {
                let workflow = Workflow {
                    id: format!("workflow-{:?}", uuid::Uuid::new_v4()),
                    name: tmpl.name.clone(),
                    template_id: tmpl.id.clone(),
                    status: WorkflowStatus::Pending,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    steps: tmpl.steps.clone(),
                };
                
                self.workflows.push(workflow.clone());
                
                Ok(super::AIResponse {
                    message: format!("Created workflow: {}. This will execute {} steps.", workflow.name, workflow.steps.len()),
                    confidence: 0.9,
                    action: Some(format!("execute_workflow_{}", workflow.id)),
                })
            } else {
                // Create custom workflow
                let workflow = self.create_custom_workflow(command)?;
                self.workflows.push(workflow.clone());
                
                Ok(super::AIResponse {
                    message: format!("Created custom workflow: {}", workflow.name),
                    confidence: 0.7,
                    action: Some(format!("execute_workflow_{}", workflow.id)),
                })
            }
        } else {
            Ok(super::AIResponse {
                message: "Please specify what workflow to create".to_string(),
                confidence: 0.0,
                action: None,
            })
        }
    }

    /// Find a matching template
    fn find_template(&self, command: &str) -> Option<&WorkflowTemplate> {
        let command_lower = command.to_lowercase();
        
        self.templates
            .iter()
            .find(|t| t.natural_language_patterns.iter().any(|p| command_lower.contains(p)))
    }

    /// Create a custom workflow
    fn create_custom_workflow(&self, command: &str) -> Result<Workflow, Box<dyn std::error::Error>> {
        Ok(Workflow {
            id: format!("workflow-{:?}", uuid::Uuid::new_v4()),
            name: format!("Custom: {}", command),
            template_id: "custom".to_string(),
            status: WorkflowStatus::Pending,
            created_at: chrono::Utc::now().to_rfc3339(),
            steps: vec![
                WorkflowStep {
                    action: "analyze_request".to_string(),
                    parameters: vec![command.to_string()],
                },
                WorkflowStep {
                    action: "execute_custom".to_string(),
                    parameters: vec![command.to_string()],
                },
            ],
        })
    }

    /// Execute a workflow
    pub fn execute_workflow(&mut self, workflow_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(workflow) = self.workflows.iter_mut().find(|w| w.id == workflow_id) {
            workflow.status = WorkflowStatus::Running;
            
            // Execute steps (placeholder)
            for step in &workflow.steps {
                println!("Executing step: {} with parameters: {:?}", step.action, step.parameters);
            }
            
            workflow.status = WorkflowStatus::Completed;
            Ok(())
        } else {
            Err(format!("Workflow {} not found", workflow_id).into())
        }
    }

    /// Get all workflows
    pub fn get_workflows(&self) -> Vec<Workflow> {
        self.workflows.clone()
    }

    /// Get workflow templates
    pub fn get_templates(&self) -> Vec<WorkflowTemplate> {
        self.templates.clone()
    }

    /// Add a custom template
    pub fn add_template(&mut self, template: WorkflowTemplate) {
        self.templates.push(template);
    }
}

/// Workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub template_id: String,
    pub status: WorkflowStatus,
    pub created_at: String,
    pub steps: Vec<WorkflowStep>,
}

/// Workflow status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub action: String,
    pub parameters: Vec<String>,
}

/// Workflow template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub natural_language_patterns: Vec<String>,
    pub steps: Vec<WorkflowStep>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_engine_creation() {
        let engine = AutomationEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_create_workflow() {
        let mut engine = AutomationEngine::new().unwrap();
        let response = engine.create_workflow(&["create python project".to_string()]);
        assert!(response.is_ok());
        assert!(response.unwrap().confidence > 0.5);
    }
}
