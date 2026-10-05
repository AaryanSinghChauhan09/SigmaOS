# Search Manager

## Overview

The Search Manager provides comprehensive search functionality inspired by Linux Mint's search and Omarchy's search utilities. It supports file, application, setting, and command search with configurable result types and relevance ranking.

## Features

- **Search Result Types**: File, Application, Setting, Command, Web
- **Indexing**: File, application, setting, and command indexing
- **Search Query**: Configurable search types and max results
- **Relevance Ranking**: Sort results by relevance score
- **Type Filtering**: Search specific result types
- **Default Indexing**: Pre-indexed applications, commands, and settings
- **Custom Indexing**: Add custom files, applications, settings, commands
- **Statistics**: Track indexed item counts

## Components

### SearchResultType

```rust
pub enum SearchResultType {
    File,        // File search results
    Application, // Application search results
    Setting,     // Setting search results
    Command,     // Command search results
    Web,         // Web search results
}
```

### SearchResult

Represents a search result with:
- Unique result ID
- Result type
- Title
- Description
- Path/URI
- Icon (optional)
- Relevance score

### SearchQuery

Represents a search query with:
- Query string
- Search types to include
- Max results limit

### SearchManager

Main management interface with:
- Indexing operations (files, applications, settings, commands)
- Search execution
- Type-specific search
- Relevance-based sorting
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::SearchManager;

let manager = SearchManager::new();

// Search for applications
let query = SearchQuery::new("terminal".to_string());
let results = manager.search(query);

for result in results {
    println!("{}: {}", result.title, result.description);
}
```

### Indexing Content

```rust
let mut manager = SearchManager::new();

// Index a file
let file_result = SearchResult::new(
    "file1".to_string(),
    SearchResultType::File,
    "Document.txt".to_string(),
    "Text document".to_string(),
    "/home/user/Document.txt".to_string(),
);
manager.index_file(file_result);

// Index an application
let app_result = SearchResult::new(
    "app1".to_string(),
    SearchResultType::Application,
    "My App".to_string(),
    "My application".to_string(),
    "/usr/bin/myapp".to_string(),
);
manager.index_application(app_result);

// Index a setting
let setting_result = SearchResult::new(
    "setting1".to_string(),
    SearchResultType::Setting,
    "Theme".to_string(),
    "Desktop theme".to_string(),
    "settings://theme".to_string(),
);
manager.index_setting(setting_result);

// Index a command
let command_result = SearchResult::new(
    "cmd1".to_string(),
    SearchResultType::Command,
    "mycommand".to_string(),
    "My command".to_string(),
    "/usr/bin/mycommand".to_string(),
);
manager.index_command(command_result);
```

### Search Configuration

```rust
// Search with specific types
let query = SearchQuery::new("term".to_string())
    .with_types(vec![SearchResultType::Application, SearchResultType::Command]);

// Search with max results limit
let query = SearchQuery::new("term".to_string())
    .with_max_results(10);

// Combined configuration
let query = SearchQuery::new("term".to_string())
    .with_types(vec![SearchResultType::File])
    .with_max_results(5);
```

### Search Operations

```rust
// General search (all default types)
let query = SearchQuery::new("terminal".to_string());
let results = manager.search(query);

// Search only applications
let query = SearchQuery::new("terminal".to_string())
    .with_types(vec![SearchResultType::Application]);
let results = manager.search(query);

// Search only commands
let query = SearchQuery::new("ls".to_string())
    .with_types(vec![SearchResultType::Command]);
let results = manager.search(query);
```

### Result Customization

```rust
let mut result = SearchResult::new(
    "id".to_string(),
    SearchResultType::File,
    "Title".to_string(),
    "Description".to_string(),
    "/path/to/file".to_string(),
);

// Set icon
result.set_icon("folder".to_string());

// Set relevance score
result.set_relevance(0.95);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Indexed files: {}", stats.indexed_files);
println!("Indexed apps: {}", stats.indexed_apps);
println!("Indexed settings: {}", stats.indexed_settings);
println!("Indexed commands: {}", stats.indexed_commands);
```

## Default Indexed Content

The Search Manager includes pre-indexed content:

**Applications:**
- Terminal
- File Manager
- Web Browser
- Settings
- Calculator

**Commands:**
- ls (List directory contents)
- cd (Change directory)
- cp (Copy files)
- mv (Move files)
- rm (Remove files)
- grep (Search text)

**Settings:**
- Theme
- Display
- Sound
- Network
- Power

## AI Agent Maintenance Instructions

When maintaining the Search Manager:

1. **Query Validation**: Validate search queries before execution
2. **Relevance Scoring**: Implement accurate relevance scoring algorithms
3. **Index Updates**: Update indexes when content changes
4. **Type Safety**: Ensure result types match their content
5. **Path Validation**: Validate file paths before indexing
6. **Search Performance**: Optimize search for large indexes

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::search_manager
```

## Future Enhancements

- Real-time file system indexing
- Fuzzy search support
- Search history
- Search suggestions
- Web search integration
- Search result actions (open, delete, etc.)
- Per-user search preferences
- Search result caching
- Advanced search filters (date, size, type)
- Search result preview
