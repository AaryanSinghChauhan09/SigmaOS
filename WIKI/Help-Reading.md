# Help:Reading

This page provides guidance on how to read and understand SigmaOS Wiki articles effectively.

## Article Structure

SigmaOS Wiki articles generally follow a consistent structure:

### Headings
- **Main Heading** (`#`): Article title
- **Section Headings** (`##`): Major sections
- **Subsection Headings** (`###`): Subsections within major sections

### Code Blocks
Code is displayed in fenced code blocks with syntax highlighting:

```bash
# Example command
sudo sigpkg install package-name
```

```rust
// Example Rust code
fn main() {
    println!("Hello, SigmaOS!");
}
```

### Notes and Warnings
Important information is highlighted:

> **Note**: This is important information that you should pay attention to.

> **Warning**: This is a warning about potential issues or risks.

## Common Conventions

### Commands
Commands that should be run in a terminal are displayed in code blocks with the shell type indicated:

```bash
# For bash/sh commands
command arguments
```

### File Paths
File paths are displayed in code inline: `/etc/sigma-config/config.toml`

### Configuration Examples
Configuration examples show the exact format needed:

```toml
[desktop]
theme = "dark"
workspace = "developer"
```

### Menu Navigation
Menu navigation is shown with arrows: `System → Settings → Display`

## Understanding Instructions

### Step-by-Step Guides
Step-by-step guides break complex tasks into numbered steps:

1. First step description
2. Second step description
3. Third step description

### Conditional Instructions
Some instructions depend on your configuration:

> **If you are using x86_64 architecture**: Follow these instructions
> 
> **If you are using AArch64 architecture**: Follow these instructions instead

### Optional Steps
Optional steps are marked as such:

> **Optional**: This step is not required but recommended for better experience

## Troubleshooting Common Issues

### Commands Not Found
If a command is not found, ensure:
- The package is installed via sigpkg
- The package is in your PATH
- You have the necessary permissions

### Permission Denied
If you get permission denied errors:
- Use `sudo` for system-level commands
- Check file permissions with `ls -l`
- Ensure you're in the correct user group

### Configuration Not Applied
If configuration changes don't take effect:
- Restart the affected service
- Reload the configuration file
- Check for syntax errors in the configuration

## Getting Help

If you're having trouble understanding an article:

1. **Read the Article Again**: Sometimes a second reading helps clarify details
2. **Check Related Articles**: Look for links to related documentation
3. **Search the Wiki**: Use the search function to find related information
4. **Ask the Community**: Post a question in [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
5. **Report Issues**: If the article is unclear or incorrect, report it as a documentation issue

## Contributing to Documentation

If you find documentation that is unclear, incomplete, or incorrect:

1. **Edit the Article**: You can edit wiki articles directly
2. **Suggest Improvements**: Post suggestions in GitHub Discussions
3. **Report Issues**: Create an issue for documentation problems
4. **Help Others**: Answer questions from other users

## Additional Resources

- [Table of Contents](Table-of-contents) — Browse all wiki articles
- [Help:Browsing](Help-Browsing) — Learn how to search and navigate the wiki
- [Help:Editing](Help-Editing) — Learn how to edit wiki articles
- [Wiki:Contributing](Wiki-Contributing) — Contribute to the wiki

---

**[Return to Home](Home)** | **[Help:Browsing](Help-Browsing)** | **[Help:Editing](Help-Editing)**
