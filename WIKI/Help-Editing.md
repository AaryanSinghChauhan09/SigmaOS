# Help:Editing

This tutorial explains how to edit SigmaOS Wiki articles and introduces wiki text syntax.

## Getting Started

### Edit Mode
To edit a wiki page:

1. Navigate to the page you want to edit
2. Click the "Edit" button (pencil icon) in the top right
3. The page will open in edit mode
4. Make your changes using Markdown syntax
5. Click "Preview" to see your changes
6. Click "Commit changes" when done

### Edit Button Location
The edit button is located in the top right corner of each wiki page, next to the page title.

## Markdown Syntax

### Headings
Use `#` for headings, with more `#` for smaller headings:

```markdown
# Main Heading
## Section Heading
### Subsection Heading
#### Sub-subsection Heading
```

### Text Formatting

```markdown
**Bold text** using double asterisks
*Italic text* using single asterisks
***Bold and italic*** using triple asterisks
~~Strikethrough~~ using double tildes
`Inline code` using backticks
```

### Links

#### Internal Links
Link to other wiki pages:

```markdown
[Link text](Page-name)
[Link text with anchor](Page-name#section)
```

#### External Links
Link to external websites:

```markdown
[Link text](https://example.com)
[Bare URL](https://example.com)
```

### Images

```markdown
![Alt text](image-url)
![Alt text](image-url "Optional title")
```

### Lists

#### Unordered Lists
```markdown
- Item 1
- Item 2
  - Nested item
  - Another nested item
- Item 3
```

#### Ordered Lists
```markdown
1. First item
2. Second item
   1. Nested numbered item
   2. Another nested item
3. Third item
```

### Code Blocks

#### Fenced Code Blocks
Use triple backticks with language identifier:

```rust
fn main() {
    println!("Hello, SigmaOS!");
}
```

```bash
sudo sigpkg install package-name
```

```toml
[desktop]
theme = "dark"
```

#### Inline Code
Use backticks for inline code: `code here`

### Blockquotes

```markdown
> This is a blockquote
> 
> > Nested blockquote
```

### Tables

```markdown
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
```

### Horizontal Rules

```markdown
---
```

### Task Lists

```markdown
- [x] Completed task
- [ ] Incomplete task
- [ ] Another incomplete task
```

## Best Practices

### Article Structure
Organize your article with a clear structure:

1. **Title**: Clear, descriptive title
2. **Introduction**: Brief overview of the topic
3. **Main Content**: Organized with headings
4. **Examples**: Practical examples where helpful
5. **Conclusion/Summary**: Wrap up key points
6. **See Also**: Links to related articles

### Writing Style

- **Be Clear**: Write clearly and concisely
- **Be Consistent**: Use consistent terminology and formatting
- **Be Complete**: Cover the topic adequately
- **Be Accurate**: Ensure information is correct
- **Be Helpful**: Focus on helping the reader

### Code Examples

When including code:

- **Test First**: Ensure code works as described
- **Explain Context**: Explain what the code does
- **Use Syntax Highlighting**: Specify the language for proper highlighting
- **Keep it Simple**: Avoid unnecessary complexity
- **Handle Errors**: Mention common errors and how to fix them

### Screenshots

When adding screenshots:

- **Relevant**: Screenshots should be directly relevant
- **Clear**: Ensure screenshots are clear and readable
- **Annotated**: Add annotations if helpful
- **Sized Appropriately**: Don't make screenshots too large
- **Alt Text**: Include descriptive alt text

## Advanced Features

### HTML
You can use HTML for complex formatting:

```html
<div class="note">
  <strong>Note:</strong> This is important information.
</div>
```

### Emojis
You can use emojis in your text:

```markdown
🚀 Installation
⚡ Performance
🛡️ Security
```

### Footnotes
Some Markdown processors support footnotes:

```markdown
This is a reference[^1] to a footnote.

[^1]: This is the footnote content.
```

## Common Tasks

### Creating a New Page

1. Go to the wiki main page
2. Click "New Page" or navigate to a non-existent page URL
3. Enter the page name
4. Write your content
5. Save the page

### Renaming a Page

1. Go to the page you want to rename
2. Click "Edit"
3. Copy all content
4. Create a new page with the new name
5. Paste the content
6. Update all links to the old page
7. Delete the old page (or add a redirect)

### Deleting a Page

1. Go to the page you want to delete
2. Click "Edit"
3. Delete all content
4. Add a note about why it was deleted
5. Commit the change
6. Remove links to the deleted page

### Adding Categories

Add categories at the bottom of pages:

```markdown
[[Category:Installation]]
[[Category:Desktop Environment]]
```

## Troubleshooting

### Preview Not Working
If preview doesn't work:
- Refresh the page
- Try a different browser
- Check for syntax errors in your Markdown

### Links Not Working
If links don't work:
- Check the page name spelling
- Ensure the target page exists
- Use proper link syntax

### Formatting Issues
If formatting doesn't look right:
- Check for proper Markdown syntax
- Ensure proper spacing around formatting characters
- Check for conflicting formatting

## Commit Messages

Write clear, descriptive commit messages:

```
Fix typo in Installation Guide

- Corrected command syntax
- Added missing step
- Fixed broken link
```

## Getting Help

If you need help editing:

- **Read This Guide**: Review this documentation
- **Check Examples**: Look at existing well-written pages
- **Ask Questions**: Post in [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **Review Help Pages**: [Help:Reading](Help-Reading), [Help:Browsing](Help-Browsing)

## Additional Resources

- [GitHub Flavored Markdown Spec](https://github.github.com/gfm/)
- [Markdown Guide](https://www.markdownguide.org/)
- [Wiki:Contributing](Wiki-Contributing) — Contributing guidelines
- [Table of Contents](Table-of-contents) — Browse all wiki articles

---

**[Return to Home](Home)** | **[Help:Reading](Help-Reading)** | **[Help:Browsing](Help-Browsing)**
