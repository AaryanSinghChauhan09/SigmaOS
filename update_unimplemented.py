import re
with open("src/wiki_unimplemented_ideas.rs", "r") as f:
    content = f.read()

new_methods = """
    pub fn search_notes(&self, keyword: &str) -> Vec<&MarkdownNote> {
        let mut results = Vec::new();
        for notes in self.notebooks.values() {
            for note in notes {
                if note.title.contains(keyword) || note.body_markdown.contains(keyword) {
                    results.push(note);
                }
            }
        }
        results
    }
}"""

content = content.replace("            .push(note);\n    }\n}", "            .push(note);\n    }\n" + new_methods)

with open("src/wiki_unimplemented_ideas.rs", "w") as f:
    f.write(content)
