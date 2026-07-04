// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/law/sigma_caselaw.rs — Sigma Case Law Database
//
// Implements indexed Indian judgments with AI summaries for law
// students and legal professionals.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Case Law Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Court {
    SupremeCourt,
    HighCourt,
    DistrictCourt,
    Tribunal,
}

#[derive(Debug, Clone)]
pub struct Case {
    pub id: String,
    pub case_number: String,
    pub title: String,
    pub court: Court,
    pub year: u32,
    pub citation: String,
    pub summary: String,
    pub key_points: Vec<String>,
    pub judges: Vec<String>,
    pub outcome: String,
    pub related_cases: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub case_id: String,
    pub relevance_score: f32,
    pub matched_terms: Vec<String>,
}

// ─── Case Law Database ───────────────────────────────────────────────────────

pub struct CaseLawDatabase {
    pub cases: HashMap<String, Case>,
    pub index: HashMap<String, Vec<String>>, // keyword -> case_ids
}

impl CaseLawDatabase {
    pub fn new() -> Self {
        let mut database = CaseLawDatabase {
            cases: HashMap::new(),
            index: HashMap::new(),
        };
        
        database.init_landmark_cases();
        database.build_index();
        database
    }

    /// Initialize landmark Indian cases
    fn init_landmark_cases(&mut self) {
        // Kesavananda Bharati v. State of Kerala (1973)
        self.cases.insert("case_001".to_string(), Case {
            id: "case_001".to_string(),
            case_number: "Writ Petition (Civil) 135 of 1970".to_string(),
            title: "Kesavananda Bharati v. State of Kerala".to_string(),
            court: Court::SupremeCourt,
            year: 1973,
            citation: "(1973) 4 SCC 225".to_string(),
            summary: "This landmark case established the 'Basic Structure Doctrine' of the Indian Constitution, holding that Parliament cannot alter the basic features of the Constitution through amendments.".to_string(),
            key_points: vec![
                "Basic Structure Doctrine established".to_string(),
                "Parliament's amending power is not unlimited".to_string(),
                "Fundamental rights are part of basic structure".to_string(),
                "Judicial review is a basic feature".to_string(),
            ],
            judges: vec![
                "S.M. Sikri, C.J.".to_string(),
                "H.R. Khanna, J.".to_string(),
                "A.N. Ray, J.".to_string(),
            ],
            outcome: "Supreme Court struck down the 24th Amendment but upheld the 25th Amendment partially".to_string(),
            related_cases: vec!["Minerva Mills v. Union of India (1980)".to_string(), "Indira Nehru Gandhi v. Raj Narain (1975)".to_string()],
        });

        // Vishaka v. State of Rajasthan (1997)
        self.cases.insert("case_002".to_string(), Case {
            id: "case_002".to_string(),
            case_number: "Writ Petition (Civil) 666-70 of 1993".to_string(),
            title: "Vishaka v. State of Rajasthan".to_string(),
            court: Court::SupremeCourt,
            year: 1997,
            citation: "(1997) 6 SCC 241".to_string(),
            summary: "This case established guidelines to prevent sexual harassment of women at workplace, filling legislative vacuum until proper law was enacted.".to_string(),
            key_points: vec![
                "Sexual harassment violates fundamental rights".to_string(),
                "Guidelines for prevention of sexual harassment".to_string(),
                "Employer responsibility for safe workplace".to_string(),
                "Reference to CEDAW and international conventions".to_string(),
            ],
            judges: vec![
                "Sujata V. Manohar, J.".to_string(),
                "B.N. Kirpal, J.".to_string(),
            ],
            outcome: "Supreme Court laid down Vishaka Guidelines for workplace sexual harassment".to_string(),
            related_cases: vec![" Apparel Export Promotion Council v. A.K. Chopra (1999)".to_string()],
        });

        // Shreya Singhal v. Union of India (2015)
        self.cases.insert("case_003".to_string(), Case {
            id: "case_003".to_string(),
            case_number: "Writ Petition (Criminal) 167 of 2013".to_string(),
            title: "Shreya Singhal v. Union of India".to_string(),
            court: Court::SupremeCourt,
            year: 2015,
            citation: "(2015) 5 SCC 1".to_string(),
            summary: "This case struck down Section 66A of IT Act as unconstitutional, protecting freedom of speech on social media.".to_string(),
            key_points: vec![
                "Section 66A violated Article 19(1)(a)".to_string(),
                "Distinction between discussion and incitement".to_string(),
                "Overbreadth doctrine applied".to_string(),
                "Freedom of speech on internet protected".to_string(),
            ],
            judges: vec![
                "J. Chelameswar, J.".to_string(),
                "R.F. Nariman, J.".to_string(),
            ],
            outcome: "Section 66A of IT Act declared unconstitutional".to_string(),
            related_cases: vec!["Navtej Singh Johar v. Union of India (2018)".to_string()],
        });

        // Puttaswamy v. Union of India (2017)
        self.cases.insert("case_004".to_string(), Case {
            id: "case_004".to_string(),
            case_number: "Writ Petition (Civil) 494 of 2012".to_string(),
            title: "Justice K.S. Puttaswamy (Retd.) v. Union of India".to_string(),
            court: Court::SupremeCourt,
            year: 2017,
            citation: "(2017) 10 SCC 1".to_string(),
            summary: "This landmark case recognized 'Right to Privacy' as a fundamental right under Article 21 of the Constitution.".to_string(),
            key_points: vec![
                "Right to Privacy is a fundamental right".to_string(),
                "Privacy includes informational privacy".to_string(),
                "Overruled previous judgments (MP Sharma, Kharak Singh)".to_string(),
                "Aadhaar scheme requires privacy safeguards".to_string(),
            ],
            judges: vec![
                "J.S. Khehar, C.J.".to_string(),
                "J. Chelameswar, J.".to_string(),
                "S.A. Bobde, J.".to_string(),
            ],
            outcome: "Right to Privacy recognized as fundamental right under Article 21".to_string(),
            related_cases: vec!["M.P. Sharma v. Satish Chandra (1954)".to_string(), "Kharak Singh v. State of UP (1963)".to_string()],
        });
    }

    /// Build search index
    fn build_index(&mut self) {
        for (case_id, case) in &self.cases {
            // Index title words
            for word in case.title.split_whitespace() {
                let word_lower = word.to_lowercase();
                self.index.entry(word_lower).or_insert_with(Vec::new).push(case_id.clone());
            }
            
            // Index key points
            for point in &case.key_points {
                for word in point.split_whitespace() {
                    let word_lower = word.to_lowercase();
                    self.index.entry(word_lower).or_insert_with(Vec::new).push(case_id.clone());
                }
            }
        }
    }

    /// Search cases by keyword
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for word in query_lower.split_whitespace() {
            if let Some(case_ids) = self.index.get(word) {
                for case_id in case_ids {
                    if let Some(case) = self.cases.get(case_id) {
                        let relevance = self.calculate_relevance(&case, &query_lower);
                        if let Some(existing) = results.iter_mut().find(|r| r.case_id == case_id) {
                            existing.relevance_score = existing.relevance_score.max(relevance);
                            existing.matched_terms.push(word.to_string());
                        } else {
                            results.push(SearchResult {
                                case_id: case_id.clone(),
                                relevance_score: relevance,
                                matched_terms: vec![word.to_string()],
                            });
                        }
                    }
                }
            }
        }
        
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        results
    }

    /// Calculate relevance score
    fn calculate_relevance(&self, case: &Case, query: &str) -> f32 {
        let mut score = 0.0;
        
        if case.title.to_lowercase().contains(query) {
            score += 10.0;
        }
        
        for point in &case.key_points {
            if point.to_lowercase().contains(query) {
                score += 5.0;
            }
        }
        
        if case.summary.to_lowercase().contains(query) {
            score += 2.0;
        }
        
        score
    }

    /// Get case by ID
    pub fn get_case(&self, id: &str) -> Option<&Case> {
        self.cases.get(id)
    }

    /// Get cases by court
    pub fn get_cases_by_court(&self, court: Court) -> Vec<&Case> {
        self.cases.values()
            .filter(|c| c.court == court)
            .collect()
    }

    /// Get cases by year
    pub fn get_cases_by_year(&self, year: u32) -> Vec<&Case> {
        self.cases.values()
            .filter(|c| c.year == year)
            .collect()
    }

    /// Get all cases
    pub fn get_all_cases(&self) -> Vec<&Case> {
        self.cases.values().collect()
    }

    /// Get AI summary of case
    pub fn get_ai_summary(&self, case_id: &str) -> Option<String> {
        if let Some(case) = self.cases.get(case_id) {
            let summary = format!(
                "Case: {} ({})\nCourt: {}\nYear: {}\n\nSummary: {}\n\nKey Points:\n{}\n\nOutcome: {}",
                case.title,
                case.citation,
                format!("{:?}", case.court),
                case.year,
                case.summary,
                case.key_points.iter().enumerate().map(|(i, p)| format!("{}. {}", i + 1, p)).collect::<Vec<_>>().join("\n"),
                case.outcome
            );
            Some(summary)
        } else {
            None
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let database = CaseLawDatabase::new();
    
    println!("Sigma Case Law Database v0.1 - Indian Judgments");
    
    loop {
        println!("\nCommands: search <query>, case <id>, court <type>, year <year>, list, summary <id>, quit");
        println!("Courts: supremecourt, highcourt, districtcourt, tribunal");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "search" => {
                if parts.len() >= 2 {
                    let query = parts[1..].join(" ");
                    let results = database.search(&query);
                    println!("--- Search Results for '{}' ---", query);
                    for result in results {
                        if let Some(case) = database.get_case(&result.case_id) {
                            println!("{} ({}) - Relevance: {:.1}", case.title, case.year, result.relevance_score);
                        }
                    }
                }
            }
            "case" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(case) = database.get_case(arg) {
                        println!("--- Case Details ---");
                        println!("Title: {}", case.title);
                        println!("Case No: {}", case.case_number);
                        println!("Court: {:?}", case.court);
                        println!("Year: {}", case.year);
                        println!("Citation: {}", case.citation);
                        println!("\nSummary: {}", case.summary);
                        println!("\nKey Points:");
                        for (i, point) in case.key_points.iter().enumerate() {
                            println!("{}. {}", i + 1, point);
                        }
                        println!("\nJudges: {}", case.judges.join(", "));
                        println!("Outcome: {}", case.outcome);
                        println!("\nRelated Cases: {}", case.related_cases.join(", "));
                    }
                }
            }
            "court" => {
                if let Some(arg) = parts.get(1) {
                    let court = match *arg {
                        "supremecourt" => Court::SupremeCourt,
                        "highcourt" => Court::HighCourt,
                        "districtcourt" => Court::DistrictCourt,
                        "tribunal" => Court::Tribunal,
                        _ => {
                            println!("Unknown court type");
                            continue;
                        }
                    };
                    println!("--- Cases from {:?} ---", court);
                    for case in database.get_cases_by_court(court) {
                        println!("{} ({}) - {}", case.title, case.year, case.citation);
                    }
                }
            }
            "year" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(year) = arg.parse::<u32>() {
                        println!("--- Cases from {} ---", year);
                        for case in database.get_cases_by_year(year) {
                            println!("{} - {}", case.title, case.citation);
                        }
                    }
                }
            }
            "list" => {
                println!("--- All Cases ---");
                for case in database.get_all_cases() {
                    println!("{} ({}) - {}", case.title, case.year, case.citation);
                }
            }
            "summary" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(summary) = database.get_ai_summary(arg) {
                        println!("--- AI Summary ---");
                        println!("{}", summary);
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
