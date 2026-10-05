# Backup Job Manager

## Overview

The Backup Job Manager provides comprehensive backup management inspired by Linux Mint's backup tools and Omarchy's backup utilities. It supports full, incremental, and differential backups with configurable sources, destinations, and job scheduling.

## Features

- **Backup Types**: Full, Incremental, Differential
- **Backup Status**: Pending, Running, Completed, Failed, Cancelled
- **Backup Sources**: Configurable backup sources with enable/disable
- **Backup Destinations**: Multiple backup destinations with capacity tracking
- **Job Management**: Create, cancel, list backup jobs
- **Job Tracking**: Track job status, size, completion time, errors
- **Default Configuration**: Pre-configured home and documents sources, local backup destination
- **Statistics**: Track source, destination, and job counts

## Components

### BackupType

```rust
pub enum BackupType {
    Full,         // Full backup of all data
    Incremental,  // Incremental backup since last backup
    Differential, // Differential backup since last full backup
}
```

### BackupStatus

```rust
pub enum BackupStatus {
    Pending,    // Backup is pending
    Running,    // Backup is running
    Completed,  // Backup completed successfully
    Failed,     // Backup failed
    Cancelled,  // Backup was cancelled
}
```

### BackupSource

Represents a backup source with:
- Unique source ID
- Source path
- Source name
- Enabled flag

### BackupDestination

Represents a backup destination with:
- Unique destination ID
- Destination path
- Destination name
- Capacity (bytes)

### BackupJob

Represents a backup job with:
- Unique job ID
- Backup type
- Source ID
- Destination ID
- Status
- Creation timestamp
- Completion timestamp
- Size
- Error message

### BackupJobManager

Main management interface with:
- Source management (add, list, get)
- Destination management (add, list, get)
- Backup job creation
- Job management (cancel, list, get)
- Job status filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::backup::BackupJobManager;

let manager = BackupJobManager::new();

// List sources
let sources = manager.list_sources();
for source in sources {
    println!("{}: {}", source.name, source.path);
}
```

### Source Management

```rust
// Add a source
let source = BackupSource::new(
    "pictures".to_string(),
    "/home/user/Pictures".to_string(),
    "Pictures".to_string(),
);
manager.add_source(source);

// Get a source
if let Some(source) = manager.get_source("pictures") {
    println!("Source: {}", source.name);
}

// List all sources
let sources = manager.list_sources();
```

### Destination Management

```rust
// Add a destination
let destination = BackupDestination::new(
    "remote-backup".to_string(),
    "/mnt/backup".to_string(),
    "Remote Backup".to_string(),
    2 * 1024 * 1024 * 1024 * 1024, // 2 TB
);
manager.add_destination(destination);

// Get a destination
if let Some(dest) = manager.get_destination("remote-backup")) {
    println!("Destination: {}", dest.name);
}

// List all destinations
let destinations = manager.list_destinations();
```

### Creating Backups

```rust
// Create a full backup
let job_id = manager.create_backup(
    BackupType::Full,
    "home",
    "local-backup",
)?;

// Create an incremental backup
let job_id = manager.create_backup(
    BackupType::Incremental,
    "documents",
    "local-backup",
)?;

// Create a differential backup
let job_id = manager.create_backup(
    BackupType::Differential,
    "home",
    "remote-backup",
)?;
```

### Job Management

```rust
// Get a job
if let Some(job) = manager.get_job(&job_id) {
    println!("Status: {}", job.status.as_str());
    println!("Type: {}", job.backup_type.as_str());
    println!("Size: {} bytes", job.size);
}

// List all jobs
let jobs = manager.list_jobs();

// List jobs by status
let completed = manager.list_jobs_by_status(BackupStatus::Completed);
let failed = manager.list_jobs_by_status(BackupStatus::Failed);

// Cancel a job
manager.cancel_job(&job_id)?;
```

### Source Enable/Disable

```rust
// Disable a source
if let Some(source) = manager.get_source_mut("documents") {
    source.set_enabled(false);
}

// Attempting to backup a disabled source will fail
let result = manager.create_backup(
    BackupType::Full,
    "documents",
    "local-backup",
);
assert!(result.is_err());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total sources: {}", stats.total_sources);
println!("Enabled sources: {}", stats.enabled_sources);
println!("Total destinations: {}", stats.total_destinations);
println!("Total jobs: {}", stats.total_jobs);
println!("Completed jobs: {}", stats.completed_jobs);
println!("Failed jobs: {}", stats.failed_jobs);
```

## Default Configuration

The Backup Job Manager includes pre-configured sources and destinations:

**Sources:**
- **home**: /home (Home Directory)
- **documents**: /home/user/Documents (Documents)

**Destinations:**
- **local-backup**: /backup (Local Backup, 1 TB capacity)

## AI Agent Maintenance Instructions

When maintaining the Backup Job Manager:

1. **Path Validation**: Validate source and destination paths before adding
2. **Capacity Checking**: Ensure destination has sufficient capacity before backup
3. **Job Tracking**: Accurately track job status, size, and completion time
4. **Error Handling**: Capture and store error messages for failed jobs
5. **Source Availability**: Check source availability before creating jobs
6. **Destination Availability**: Check destination availability before creating jobs

## Testing

Run the unit tests with:

```bash
cargo test --lib backup::backup_manager
```

## Future Enhancements

- Scheduled backup support (cron-like scheduling)
- Compression and encryption
- Backup retention policies
- Backup verification
- Differential/incremental delta calculation
- Remote backup destinations (SSH, S3, etc.)
- Backup deduplication
- Real-time backup progress tracking
- Backup history and logs
- Restore functionality
- Backup notifications
