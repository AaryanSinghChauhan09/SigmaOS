#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    MethodCall,
    MethodReturn,
    Error,
    Signal,
}

#[derive(Debug, Clone)]
pub enum Payload {
    Empty,
    String(String),
    Binary(Vec<u8>),
    Integer(i64),
}

/// A strongly-typed IPC message.
#[derive(Debug, Clone)]
pub struct Message {
    pub msg_type: MessageType,
    pub sender: String,
    pub target: String,
    pub member: String, // The method or signal name
    pub payload: Payload,
}

impl Message {
    pub fn new_call(sender: &str, target: &str, method: &str, payload: Payload) -> Self {
        Self {
            msg_type: MessageType::MethodCall,
            sender: sender.to_string(),
            target: target.to_string(),
            member: method.to_string(),
            payload,
        }
    }

    pub fn new_signal(sender: &str, signal: &str, payload: Payload) -> Self {
        Self {
            msg_type: MessageType::Signal,
            sender: sender.to_string(),
            // Signals are typically broadcasted, so target might be wildcard.
            target: "*".to_string(), 
            member: signal.to_string(),
            payload,
        }
    }
}
