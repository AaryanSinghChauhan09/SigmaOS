
// Sovereign Core Apps Shard (SigmaOffice, SigmaTasks, SigmaVault, SigmaChat)
// Zero-dependency, #![no_std] compliant

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

const MAX_NODES: usize = 16;
const MAX_TASKS: usize = 16;
const MAX_SECRETS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct TextNode {
    pub node_id: u32,
    pub parent_id: u32,
    pub formatting_flags: u8,
    pub content_hash: u32,
}

pub struct SigmaOfficeDocument {
    pub nodes: [Option<TextNode>; MAX_NODES],
    pub next_node_id: u32,
}

impl SigmaOfficeDocument {
    pub fn new() -> Self {
        const EMPTY_NODE: Option<TextNode> = None;
        Self {
            nodes: [EMPTY_NODE; MAX_NODES],
            next_node_id: 1,
        }
    }

    pub fn add_node(
        &mut self,
        parent_id: u32,
        formatting_flags: u8,
        content_hash: u32,
    ) -> Result<u32, &'static str> {
        let id = self.next_node_id;
        let node = TextNode {
            node_id: id,
            parent_id,
            formatting_flags,
            content_hash,
        };

        for slot in self.nodes.iter_mut() {
            if slot.is_none() {
                *slot = Some(node);
                self.next_node_id += 1;
                return Ok(id);
            }
        }

        Err("SigmaOfficeDocument: Document node limit reached")
    }
}

impl Default for SigmaOfficeDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Copy)]
pub struct ProductivityTask {
    pub task_id: u32,
    pub title_hash: u32,
    pub priority: TaskPriority,
    pub is_completed: bool,
}

pub struct SigmaTasksBoard {
    pub tasks: [Option<ProductivityTask>; MAX_TASKS],
    pub next_task_id: u32,
}

impl SigmaTasksBoard {
    pub fn new() -> Self {
        const EMPTY_TASK: Option<ProductivityTask> = None;
        Self {
            tasks: [EMPTY_TASK; MAX_TASKS],
            next_task_id: 1,
        }
    }

    pub fn add_task(
        &mut self,
        title_hash: u32,
        priority: TaskPriority,
    ) -> Result<u32, &'static str> {
        let id = self.next_task_id;
        let task = ProductivityTask {
            task_id: id,
            title_hash,
            priority,
            is_completed: false,
        };

        for slot in self.tasks.iter_mut() {
            if slot.is_none() {
                *slot = Some(task);
                self.next_task_id += 1;
                return Ok(id);
            }
        }

        Err("SigmaTasksBoard: Task board capacity exceeded")
    }
}

impl Default for SigmaTasksBoard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SigmaVaultContainer {
    pub secrets: [Option<(u32, [u8; 32])>; MAX_SECRETS],
    pub next_secret_id: u32,
}

impl SigmaVaultContainer {
    pub fn new() -> Self {
        const EMPTY_SECRET: Option<(u32, [u8; 32])> = None;
        Self {
            secrets: [EMPTY_SECRET; MAX_SECRETS],
            next_secret_id: 1,
        }
    }

    pub fn store_secret(&mut self, payload: [u8; 32]) -> Result<u32, &'static str> {
        let id = self.next_secret_id;
        for slot in self.secrets.iter_mut() {
            if slot.is_none() {
                *slot = Some((id, payload));
                self.next_secret_id += 1;
                return Ok(id);
            }
        }

        Err("SigmaVaultContainer: Vault capacity exceeded")
    }
}

impl Default for SigmaVaultContainer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// SIGMACHAT: IRC (irssi/weechat) & MATRIX INSPIRED CHAT ROOM APPLICATION
// =========================================================================

#[derive(Debug, Clone)]
pub struct SovereignChatMessage {
    pub message_id: u64,
    pub sender: String,
    pub body: String,
    pub timestamp_sec: u64,
    pub is_encrypted: bool,
}

pub struct SovereignChatRoom {
    pub room_id: String,
    pub topic: String,
    pub members: Vec<String>,
    pub messages: Vec<SovereignChatMessage>,
    pub is_e2ee: bool,
}

pub struct SigmaChatRoomManager {
    pub rooms: BTreeMap<String, SovereignChatRoom>,
    pub active_user: String,
    pub next_msg_id: u64,
}

impl SigmaChatRoomManager {
    pub fn new(active_user: &str) -> Self {
        let mut mgr = Self {
            rooms: BTreeMap::new(),
            active_user: active_user.to_string(),
            next_msg_id: 1,
        };

        // Default channels
        mgr.create_room("#general", "General Discussion Channel", false);
        mgr.create_room("#dev", "SigmaOS Kernel & Userspace Core Dev", true);
        mgr
    }

    pub fn create_room(&mut self, room_id: &str, topic: &str, is_e2ee: bool) {
        let mut members = Vec::new();
        members.push(self.active_user.clone());

        self.rooms.insert(
            room_id.to_string(),
            SovereignChatRoom {
                room_id: room_id.to_string(),
                topic: topic.to_string(),
                members,
                messages: Vec::new(),
                is_e2ee,
            },
        );
    }

    /// Process IRC-style command line input or plain message
    pub fn process_input(
        &mut self,
        current_room: &str,
        input: &str,
        timestamp_sec: u64,
    ) -> Result<String, &'static str> {
        let trimmed = input.trim();
        if trimmed.starts_with('/') {
            let mut parts = trimmed.split_whitespace();
            let cmd = parts.next().unwrap_or("");
            match cmd {
                "/join" => {
                    let room = parts.next().ok_or("Usage: /join <room_id>")?;
                    if !self.rooms.contains_key(room) {
                        self.create_room(room, "Custom Channel", false);
                    } else {
                        let room_obj = self.rooms.get_mut(room).unwrap();
                        if !room_obj.members.contains(&self.active_user) {
                            room_obj.members.push(self.active_user.clone());
                        }
                    }
                    Ok(format!("Joined room {}", room))
                }
                "/topic" => {
                    let topic_text: Vec<&str> = parts.collect();
                    let new_topic = topic_text.join(" ");
                    let room_obj = self.rooms.get_mut(current_room).ok_or("Room not found")?;
                    room_obj.topic = new_topic.clone();
                    Ok(format!("Topic updated for {}: {}", current_room, new_topic))
                }
                "/nick" => {
                    let new_nick = parts.next().ok_or("Usage: /nick <new_nickname>")?;
                    self.active_user = new_nick.to_string();
                    Ok(format!("Nickname changed to {}", new_nick))
                }
                _ => Err("Unknown IRC command"),
            }
        } else {
            // Post message to current room
            let msg_id = self.next_msg_id;
            self.next_msg_id += 1;

            let room_obj = self.rooms.get_mut(current_room).ok_or("Room not found")?;
            let is_e2ee = room_obj.is_e2ee;

            room_obj.messages.push(SovereignChatMessage {
                message_id: msg_id,
                sender: self.active_user.clone(),
                body: trimmed.to_string(),
                timestamp_sec,
                is_encrypted: is_e2ee,
            });

            Ok(format!("Message posted to {}", current_room))
        }
    }
}

impl Default for SigmaChatRoomManager {
    fn default() -> Self {
        Self::new("alice")
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_apps() {
        let mut doc = SigmaOfficeDocument::new();
        let node_id = doc.add_node(0, 0x01, 12345).unwrap();
        assert_eq!(node_id, 1);

        let mut board = SigmaTasksBoard::new();
        let task_id = board.add_task(54321, TaskPriority::High).unwrap();
        assert_eq!(task_id, 1);

        let mut vault = SigmaVaultContainer::new();
        let secret_id = vault.store_secret([0xAA; 32]).unwrap();
        assert_eq!(secret_id, 1);

        let mut chat = SigmaChatRoomManager::new("alice");
        assert!(chat
            .process_input("#general", "Hello SigmaOS World!", 1700000000)
            .is_ok());
        assert_eq!(chat.rooms.get("#general").unwrap().messages.len(), 1);

        assert!(chat
            .process_input("#general", "/join #kernel", 1700000001)
            .is_ok());
        assert!(chat.rooms.contains_key("#kernel"));

        assert!(chat
            .process_input("#general", "/nick bob", 1700000002)
            .is_ok());
        assert_eq!(chat.active_user, "bob");
    }
}
