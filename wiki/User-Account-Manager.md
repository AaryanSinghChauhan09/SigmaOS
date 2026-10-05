# User Account Manager

## Overview

The User Account Manager provides user account management inspired by Linux Mint's user accounts and Omarchy's user utilities. It supports user creation, deletion, modification, and account type management.

## Features

- **User Types**: System, Normal, Administrator accounts
- **User Account Management**: Create, delete, update users
- **Account Information**: Username, UID, GID, full name, home directory, shell
- **Account Status**: Active/inactive status
- **Root Protection**: Prevents deletion and deactivation of root user
- **User Lookup**: Find users by username or UID
- **Statistics**: Track user counts by type and status

## Components

### UserType

```rust
pub enum UserType {
    System,        // System service account
    Normal,         // Regular user account
    Administrator,  // Administrator/root account
}
```

### UserAccount

Represents a user account with:
- Username and UID/GID
- Full name
- Home directory
- Default shell
- User type
- Active status
- Creation timestamp

### AccountManager

Main management interface with:
- User creation with auto UID assignment
- User deletion (with root protection)
- User updates (full name, home, shell)
- Account activation/deactivation
- User lookup by username or UID
- Type-based filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::AccountManager;

let mut manager = AccountManager::new();

// List all users
let users = manager.list_users();

// Get a specific user
if let Some(user) = manager.get_user("root") {
    println!("User: {}", user.username);
}
```

### Creating Users

```rust
// Create a normal user
let user = manager.create_user("newuser".to_string(), UserType::Normal)?;

// Create an administrator
let admin = manager.create_user("admin".to_string(), UserType::Administrator)?;
```

### Managing Users

```rust
// Update user information
manager.update_user(
    "newuser",
    Some("John Doe".to_string()),
    None,
    Some("/bin/zsh".to_string()),
)?;

// Deactivate a user
manager.deactivate_user("newuser")?;

// Reactivate a user
manager.activate_user("newuser")?;
```

### Deleting Users

```rust
// Delete a user
manager.delete_user("olduser")?;

// Cannot delete root
assert!(manager.delete_user("root").is_err());
```

### Filtering Users

```rust
// List normal users
let normal_users = manager.list_by_type(UserType::Normal);

// List administrators
let admins = manager.list_by_type(UserType::Administrator);

// List active users
let active = manager.list_active();
```

### User Lookup

```rust
// Find by username
if let Some(user) = manager.get_user("newuser") {
    println!("Found user: {}", user.full_name);
}

// Find by UID
if let Some(user) = manager.get_user_by_uid(1000) {
    println!("Found user: {}", user.username);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total users: {}", stats.total_users);
println!("Active users: {}", stats.active_users);
println!("Administrators: {}", stats.admin_users);
println!("Next UID: {}", stats.next_uid);
```

## AI Agent Maintenance Instructions

When maintaining the User Account Manager:

1. **Root Protection**: Ensure root user cannot be deleted or deactivated
2. **UID Uniqueness**: Maintain unique UID assignment for new users
3. **Username Validation**: Validate usernames before creation
4. **Home Directory**: Ensure home directories are properly set
5. **Shell Validation**: Validate shell paths before assignment
6. **Account Security**: Maintain proper account status tracking

## Testing

Run the unit tests with:

```bash
cargo test --lib system::user_manager
```

## Future Enhancements

- Integration with actual user database (/etc/passwd, /etc/shadow)
- Password management
- Group membership management
- Password policy enforcement
- Account expiration
- User profile templates
- Bulk user operations
