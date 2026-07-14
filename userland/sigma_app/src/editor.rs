#[derive(Debug, Clone)]
pub struct TextBuffer {
    pub content: String,
    pub cursor_offset: usize,
}

pub struct SigmaEditor {
    pub active_buffer: TextBuffer,
}

impl Default for SigmaEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaEditor {
    pub fn new() -> Self {
        Self {
            active_buffer: TextBuffer {
                content: String::new(),
                cursor_offset: 0,
            },
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        let offset = self.active_buffer.cursor_offset;
        self.active_buffer.content.insert_str(offset, text);
        self.active_buffer.cursor_offset += text.len();
    }

    pub fn move_cursor(&mut self, to: usize) -> Result<(), String> {
        if to <= self.active_buffer.content.len() {
            self.active_buffer.cursor_offset = to;
            Ok(())
        } else {
            Err("Cursor out of bounds".to_string())
        }
    }
}
