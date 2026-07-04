// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/law/sigma_drafting.rs — Sigma Legal Drafting Assistant
//
// Implements templates for petitions, contracts, and compliance forms
// for law students and legal professionals.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Document Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DocumentType {
    Petition,
    Contract,
    Affidavit,
    LegalNotice,
    ComplianceForm,
    Memorandum,
}

#[derive(Debug, Clone)]
pub struct DocumentTemplate {
    pub id: String,
    pub doc_type: DocumentType,
    pub title: String,
    pub description: String,
    pub template: String,
    pub required_fields: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub doc_type: DocumentType,
    pub title: String,
    pub content: String,
    pub fields: HashMap<String, String>,
    pub created_at: String,
}

// ─── Legal Drafting Assistant ───────────────────────────────────────────────

pub struct LegalDraftingAssistant {
    pub templates: HashMap<String, DocumentTemplate>,
    pub documents: Vec<Document>,
}

impl LegalDraftingAssistant {
    pub fn new() -> Self {
        let mut assistant = LegalDraftingAssistant {
            templates: HashMap::new(),
            documents: Vec::new(),
        };
        
        assistant.init_templates();
        assistant
    }

    /// Initialize document templates
    fn init_templates(&mut self) {
        // Writ Petition Template
        self.templates.insert("tpl_001".to_string(), DocumentTemplate {
            id: "tpl_001".to_string(),
            doc_type: DocumentType::Petition,
            title: "Writ Petition under Article 32/226".to_string(),
            description: "Template for filing writ petition in Supreme Court or High Court".to_string(),
            template: r#"IN THE SUPREME COURT OF INDIA
[CIVIL WRIT PETITION NO. ___ OF 20__]

[Name of Petitioner(s)]
V/s
[Name of Respondent(s)]

PETITION UNDER ARTICLE 32 OF THE CONSTITUTION OF INDIA

TO THE HON'BLE CHIEF JUSTICE OF INDIA AND OTHER COMPANION JUDGES OF THE SUPREME COURT OF INDIA

The Petitioners above named most respectfully submit this petition under Article 32 of the Constitution of India for the issuance of an appropriate Writ, Direction or Order in the nature of:

1. Writ of Habeas Corpus
2. Writ of Mandamus
3. Writ of Certiorari
4. Writ of Prohibition
5. Writ of Quo Warranto

The facts of the case are as follows:

1. That the Petitioner No. 1 is [description of petitioner]
2. That the Respondent No. 1 is [description of respondent]
3. That [state the cause of action]
4. That [state the legal grounds]
5. That [state the relief sought]

PRAYER

The Petitioners therefore pray that this Hon'ble Court may be pleased to:

[State the prayers in detail]

And pass such other orders as this Hon'ble Court may deem fit in the interest of justice.

Date: [Date]
Place: [Place]

Signature of Advocate
Enrollment No.: [Enrollment Number]"#.to_string(),
            required_fields: vec![
                "Petitioner Name".to_string(),
                "Respondent Name".to_string(),
                "Cause of Action".to_string(),
                "Legal Grounds".to_string(),
                "Relief Sought".to_string(),
                "Prayers".to_string(),
                "Advocate Name".to_string(),
                "Enrollment Number".to_string(),
            ],
        });

        // Employment Contract Template
        self.templates.insert("tpl_002".to_string(), DocumentTemplate {
            id: "tpl_002".to_string(),
            doc_type: DocumentType::Contract,
            title: "Employment Contract".to_string(),
            description: "Standard employment contract template".to_string(),
            template: r#"EMPLOYMENT AGREEMENT

This Employment Agreement ("Agreement") is entered into on [Date] by and between:

[Employer Name], a company incorporated under the Companies Act, 2013, having its registered office at [Address] ("Employer")

AND

[Employee Name], residing at [Address] ("Employee")

WHEREAS the Employer wishes to employ the Employee and the Employee wishes to accept employment with the Employer on the terms and conditions set forth herein.

NOW THEREFORE, in consideration of the mutual covenants contained herein, the parties agree as follows:

1. POSITION AND DUTIES
   The Employee shall serve as [Designation] and shall perform such duties as are customarily associated with such position.

2. COMPENSATION
   The Employee shall receive a base salary of [Amount] per annum, payable [monthly/quarterly].

3. TERM OF EMPLOYMENT
   This Agreement shall commence on [Start Date] and continue until terminated by either party.

4. WORKING HOURS
   The Employee shall work [Number] hours per week, from [Start Time] to [End Time].

5. BENEFITS
   The Employee shall be entitled to [list of benefits].

6. CONFIDENTIALITY
   The Employee shall maintain confidentiality of all proprietary information.

7. TERMINATION
   Either party may terminate this Agreement by giving [Notice Period] days written notice.

IN WITNESS WHEREOF, the parties have executed this Agreement as of the date first above written.

_________________________
[Employer Name]

_________________________
[Employee Name]"#.to_string(),
            required_fields: vec![
                "Employer Name".to_string(),
                "Employer Address".to_string(),
                "Employee Name".to_string(),
                "Employee Address".to_string(),
                "Designation".to_string(),
                "Salary".to_string(),
                "Start Date".to_string(),
                "Working Hours".to_string(),
                "Benefits".to_string(),
                "Notice Period".to_string(),
            ],
        });

        // Affidavit Template
        self.templates.insert("tpl_003".to_string(), DocumentTemplate {
            id: "tpl_003".to_string(),
            doc_type: DocumentType::Affidavit,
            title: "General Affidavit".to_string(),
            description: "Template for general affidavit".to_string(),
            template: r#"AFFIDAVIT

I, [Deponent Name], son/daughter/wife of [Father's/Husband's Name], resident of [Address], do hereby solemnly affirm and declare as follows:

1. That I am the [relationship] of the person concerned.
2. That the facts stated herein are true to the best of my knowledge and belief.
3. That [state the facts].
4. That I am filing this affidavit to support [purpose].
5. That I am signing this affidavit voluntarily without any coercion or undue influence.

I further solemnly affirm and declare that the contents of this affidavit are true and correct to the best of my knowledge and belief.

DEPONENT

VERIFICATION
Verified at [Place] on this [Date] that the contents of the affidavit are true and correct to the best of my knowledge and belief.

DEPONENT

Before me

NOTARY PUBLIC / OATH COMMISSIONER
[Name and Address]"#.to_string(),
            required_fields: vec![
                "Deponent Name".to_string(),
                "Father's/Husband's Name".to_string(),
                "Address".to_string(),
                "Relationship".to_string(),
                "Facts".to_string(),
                "Purpose".to_string(),
            ],
        });

        // Legal Notice Template
        self.templates.insert("tpl_004".to_string(), DocumentTemplate {
            id: "tpl_004".to_string(),
            doc_type: DocumentType::LegalNotice,
            title: "Legal Notice".to_string(),
            description: "Template for sending legal notice".to_string(),
            template: r#"LEGAL NOTICE

Date: [Date]

To,
[Recipient Name]
[Recipient Address]

From,
[Sender Name]
[Sender Address]
[Advocate Name]
[Advocate Address]

SUBJECT: LEGAL NOTICE FOR [Subject]

Dear Sir/Madam,

Under instruction from my client, [Client Name], I hereby serve upon you the following legal notice:

1. That my client is the owner of [describe ownership/rights].
2. That you have [describe the violation/breach].
3. Despite repeated requests, you have failed to [describe the failure].
4. This constitutes a violation of [relevant laws/contract terms].

I hereby call upon you to:

1. [First demand]
2. [Second demand]
3. [Third demand]

You are requested to comply with this notice within [Number] days from receipt, failing which my client shall be constrained to initiate appropriate legal proceedings against you at your risk as to costs and consequences.

Copy of this notice has been retained in my office for record and further action.

Yours faithfully,

[Advocate Name]
Advocate, [Court/Bar Association]
Enrollment No.: [Enrollment Number]"#.to_string(),
            required_fields: vec![
                "Recipient Name".to_string(),
                "Recipient Address".to_string(),
                "Sender Name".to_string(),
                "Sender Address".to_string(),
                "Advocate Name".to_string(),
                "Advocate Address".to_string(),
                "Subject".to_string(),
                "Client Name".to_string(),
                "Ownership/Rights".to_string(),
                "Violation/Breach".to_string(),
                "Demands".to_string(),
                "Notice Period".to_string(),
            ],
        });
    }

    /// Get template by ID
    pub fn get_template(&self, id: &str) -> Option<&DocumentTemplate> {
        self.templates.get(id)
    }

    /// Get templates by type
    pub fn get_templates_by_type(&self, doc_type: DocumentType) -> Vec<&DocumentTemplate> {
        self.templates.values()
            .filter(|t| t.doc_type == doc_type)
            .collect()
    }

    /// Get all templates
    pub fn get_all_templates(&self) -> Vec<&DocumentTemplate> {
        self.templates.values().collect()
    }

    /// Create document from template
    pub fn create_document(&mut self, template_id: &str, fields: HashMap<String, String>) -> Result<Document, String> {
        if let Some(template) = self.templates.get(template_id) {
            let mut content = template.template.clone();
            
            // Replace placeholders with field values
            for (key, value) in &fields {
                let placeholder = format!("[{}]", key);
                content = content.replace(&placeholder, value);
            }
            
            let document = Document {
                id: format!("doc_{}", self.documents.len()),
                doc_type: template.doc_type,
                title: template.title.clone(),
                content,
                fields: fields.clone(),
                created_at: "now".to_string(),
            };
            
            self.documents.push(document.clone());
            Ok(document)
        } else {
            Err("Template not found".to_string())
        }
    }

    /// Get document by ID
    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.iter().find(|d| d.id == id)
    }

    /// Get all documents
    pub fn get_all_documents(&self) -> &[Document] {
        &self.documents
    }

    /// Validate required fields
    pub fn validate_fields(&self, template_id: &str, fields: &HashMap<String, String>) -> Result<(), String> {
        if let Some(template) = self.templates.get(template_id) {
            for required_field in &template.required_fields {
                if !fields.contains_key(required_field) {
                    return Err(format!("Missing required field: {}", required_field));
                }
            }
            Ok(())
        } else {
            Err("Template not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut assistant = LegalDraftingAssistant::new();
    
    println!("Sigma Legal Drafting Assistant v0.1 - Petitions, Contracts, Forms");
    
    loop {
        println!("\n--- Available Templates ---");
        for template in assistant.get_all_templates() {
            let type_str = match template.doc_type {
                DocumentType::Petition => "Petition",
                DocumentType::Contract => "Contract",
                DocumentType::Affidavit => "Affidavit",
                DocumentType::LegalNotice => "Legal Notice",
                DocumentType::ComplianceForm => "Compliance Form",
                DocumentType::Memorandum => "Memorandum",
            };
            println!("{} - {} ({})", template.id, template.title, type_str);
        }
        
        println!("\nCommands: template <id>, create <id>, documents, doc <id>, fields <id>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "template" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(template) = assistant.get_template(arg) {
                        println!("--- Template Details ---");
                        println!("Title: {}", template.title);
                        println!("Description: {}", template.description);
                        println!("\nRequired Fields:");
                        for field in &template.required_fields {
                            println!("- {}", field);
                        }
                    }
                }
            }
            "create" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Creating Document from Template {} ---", arg);
                    let mut fields = HashMap::new();
                    
                    if let Some(template) = assistant.get_template(arg) {
                        for field in &template.required_fields {
                            print!("{}: ", field);
                            std::io::stdout().flush().unwrap();
                            let mut value = String::new();
                            std::io::stdin().read_line(&mut value).unwrap();
                            fields.insert(field.clone(), value.trim().to_string());
                        }
                        
                        match assistant.create_document(arg, fields) {
                            Ok(doc) => {
                                println!("Document created: {}", doc.id);
                                println!("\n--- Document Preview ---");
                                println!("{}", doc.content);
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "documents" => {
                println!("--- All Documents ---");
                for doc in assistant.get_all_documents() {
                    println!("{} - {} ({})", doc.id, doc.title, doc.created_at);
                }
            }
            "doc" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(doc) = assistant.get_document(arg) {
                        println!("--- Document ---");
                        println!("Title: {}", doc.title);
                        println!("Created: {}", doc.created_at);
                        println!("\nContent:");
                        println!("{}", doc.content);
                    }
                }
            }
            "fields" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(doc) = assistant.get_document(arg) {
                        println!("--- Document Fields ---");
                        for (key, value) in &doc.fields {
                            println!("{}: {}", key, value);
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
