// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Metabase BI and natural language analytics
// Principles: DISCOVERABILITY, USER FREEDOM AND CONTROL, UNIFORM ACCESS
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone)]
pub struct FilterSpec {
    pub field: String,
    pub op: String,
    pub value: String,
}
#[derive(Debug, Clone)]
pub struct AggSpec {
    pub func: String,
    pub field: String,
    pub alias: String,
}
#[derive(Debug, Clone)]
pub enum MetabaseQuestion {
    Native {
        sql: String,
    },
    Structured {
        table: String,
        filters: Vec<FilterSpec>,
        aggregations: Vec<AggSpec>,
        breakouts: Vec<String>,
        limit: Option<u32>,
    },
}
#[derive(Debug, Clone)]
pub struct MetabaseCard {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub question: MetabaseQuestion,
    pub display_type: String,
}
#[derive(Debug, Clone)]
pub struct DashboardCard {
    pub card_id: u32,
    pub position: (u32, u32),
    pub size: (u32, u32),
}
#[derive(Debug, Clone)]
pub struct MetabaseDashboard {
    pub id: u32,
    pub name: String,
    pub cards: Vec<DashboardCard>,
}
pub struct NlQueryParser;
impl NlQueryParser {
    pub fn parse_question(nl: &str) -> MetabaseQuestion {
        let lower = nl.to_ascii_lowercase();
        if lower.contains("count") {
            MetabaseQuestion::Structured {
                table: String::from("events"),
                filters: Vec::new(),
                aggregations: alloc::vec![AggSpec {
                    func: "count".into(),
                    field: "*".into(),
                    alias: "count".into()
                }],
                breakouts: Vec::new(),
                limit: None,
            }
        } else if lower.contains("average") || lower.contains("avg") {
            MetabaseQuestion::Structured {
                table: String::from("metrics"),
                filters: Vec::new(),
                aggregations: alloc::vec![AggSpec {
                    func: "avg".into(),
                    field: "value".into(),
                    alias: "average".into()
                }],
                breakouts: Vec::new(),
                limit: None,
            }
        } else {
            MetabaseQuestion::Native {
                sql: String::from("SELECT 1"),
            }
        }
    }
}
pub struct MetabaseEngine {
    pub cards: BTreeMap<u32, MetabaseCard>,
    pub dashboards: BTreeMap<u32, MetabaseDashboard>,
    pub next_id: u32,
}
impl MetabaseEngine {
    pub fn new() -> Self {
        Self {
            cards: BTreeMap::new(),
            dashboards: BTreeMap::new(),
            next_id: 1,
        }
    }
    pub fn create_card(&mut self, name: &str, q: MetabaseQuestion, display: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.cards.insert(
            id,
            MetabaseCard {
                id,
                name: name.into(),
                description: String::new(),
                question: q,
                display_type: display.into(),
            },
        );
        id
    }
    pub fn run_card(&self, card_id: u32) -> Vec<BTreeMap<String, String>> {
        let _card = self.cards.get(&card_id);
        Vec::new()
    }
    pub fn build_dashboard(&mut self, name: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.dashboards.insert(
            id,
            MetabaseDashboard {
                id,
                name: name.into(),
                cards: Vec::new(),
            },
        );
        id
    }
    pub fn add_card_to_dashboard(
        &mut self,
        dash_id: u32,
        card_id: u32,
        pos: (u32, u32),
        size: (u32, u32),
    ) -> Result<(), &'static str> {
        self.dashboards
            .get_mut(&dash_id)
            .ok_or("dashboard not found")
            .map(|d| {
                d.cards.push(DashboardCard {
                    card_id,
                    position: pos,
                    size,
                })
            })
    }
    pub fn export_results(&self, _card_id: u32) -> Vec<u8> {
        b"[]".to_vec()
    }
    pub fn nl_to_card(&mut self, name: &str, nl: &str) -> u32 {
        let q = NlQueryParser::parse_question(nl);
        self.create_card(name, q, "table")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_card() {
        let mut e = MetabaseEngine::new();
        let q = MetabaseQuestion::Native {
            sql: String::from("SELECT count(*) FROM orders"),
        };
        let id = e.create_card("Order Count", q, "big_number");
        assert!(e.cards.contains_key(&id));
    }
    #[test]
    fn test_nl_parser_count() {
        let q = NlQueryParser::parse_question("How many users?");
        assert!(matches!(q, MetabaseQuestion::Structured { .. }));
    }
    #[test]
    fn test_nl_parser_native() {
        let q = NlQueryParser::parse_question("show me something");
        assert!(matches!(q, MetabaseQuestion::Native { .. }));
    }
    #[test]
    fn test_dashboard() {
        let mut e = MetabaseEngine::new();
        let card = e.nl_to_card("Users", "count of users");
        let dash = e.build_dashboard("Main");
        e.add_card_to_dashboard(dash, card, (0, 0), (6, 4)).unwrap();
        assert_eq!(e.dashboards[&dash].cards.len(), 1);
    }
}
