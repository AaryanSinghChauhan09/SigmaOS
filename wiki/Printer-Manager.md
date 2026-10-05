# Printer Manager

## Overview

The Printer Manager provides comprehensive printer management inspired by Linux Mint's printer settings and Omarchy's printer utilities. It supports printer discovery, queue management, and job control for local, network, and PDF printers.

## Features

- **Printer Types**: Local, Network, IPP, PDF
- **Printer Status**: Idle, Printing, Paused, Error, Offline
- **Print Job Status**: Pending, Processing, Completed, Failed, Cancelled
- **Job Management**: Submit, cancel, list jobs
- **Printer Management**: Add, remove, enable, disable printers
- **Default Printer**: Set and retrieve default printer
- **Job Configuration**: Pages, copies, color, duplex settings
- **Statistics**: Track printer and job counts

## Components

### PrinterType

```rust
pub enum PrinterType {
    Local,    // Local USB/parallel printer
    Network,  // Network printer
    IPP,      // IPP/CUPS printer
    PDF,      // Print to PDF virtual printer
}
```

### PrinterStatus

```rust
pub enum PrinterStatus {
    Idle,      // Printer is idle
    Printing,  // Printer is printing
    Paused,    // Printer is paused
    Error,     // Printer has an error
    Offline,   // Printer is offline
}
```

### PrintJobStatus

```rust
pub enum PrintJobStatus {
    Pending,     // Job is pending
    Processing,  // Job is processing
    Completed,   // Job is completed
    Failed,      // Job failed
    Cancelled,   // Job was cancelled
}
```

### PrintJob

Represents a print job with:
- Unique job ID
- Document title
- File path
- Page count
- Copy count
- Color/duplex settings
- Job status
- Creation timestamp

### Printer

Represents a printer with:
- Unique printer ID
- Printer name
- Location
- Printer type
- Status
- URI
- Driver
- Default flag
- Enabled flag

### PrinterManager

Main management interface with:
- Printer addition and removal
- Default printer management
- Enable/disable printers
- Print job submission
- Job cancellation
- Job listing and filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::printing::PrinterManager;

let mut manager = PrinterManager::new();

// List all printers
let printers = manager.list_printers();
for printer in printers {
    println!("Printer: {} ({})", printer.name, printer.printer_type.as_str());
}
```

### Printer Management

```rust
// Add a printer
let printer = Printer::new(
    "local-printer".to_string(),
    "My Printer".to_string(),
    PrinterType::Local,
    "usb://001/002".to_string(),
);
manager.add_printer(printer);

// Get a printer
if let Some(printer) = manager.get_printer("local-printer") {
    println!("Status: {}", printer.status.as_str());
}

// Set default printer
manager.set_default("local-printer")?;

// Enable/disable printer
manager.enable("local-printer")?;
manager.disable("local-printer")?;

// Remove printer
manager.remove("local-printer")?;
```

### Print Job Submission

```rust
// Submit a print job
let job_id = manager.submit_job(
    "pdf-printer",
    "Document".to_string(),
    "/home/user/document.pdf".to_string(),
    5,
)?;

// Get job status
if let Some(job) = manager.get_job(&job_id) {
    println!("Job status: {}", job.status.as_str());
    println!("Pages: {}", job.pages);
    println!("Copies: {}", job.copies);
}
```

### Job Management

```rust
// List all jobs
let jobs = manager.list_jobs();

// List jobs by status
let pending = manager.list_jobs_by_status(PrintJobStatus::Pending);

// Cancel a job
manager.cancel_job(&job_id)?;
```

### Job Configuration

```rust
let mut job = PrintJob::new(
    "job-1".to_string(),
    "Document".to_string(),
    "/tmp/doc.pdf".to_string(),
    10,
);

// Configure job
job.set_copies(2);
job.set_color(true);
job.set_duplex(true);
```

### Default Printer

```rust
// Get default printer
if let Some(printer) = manager.get_default() {
    println!("Default printer: {}", printer.name);
}

// Set default printer
manager.set_default("local-printer")?;
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total printers: {}", stats.total_printers);
println!("Enabled: {}", stats.enabled_count);
println!("Offline: {}", stats.offline_count);
println!("Total jobs: {}", stats.total_jobs);
println!("Pending jobs: {}", stats.pending_jobs);
println!("Processing jobs: {}", stats.processing_jobs);
```

## Default Printers

The Printer Manager includes a default PDF printer for printing documents to PDF files:

- **ID**: `pdf-printer`
- **Name**: Print to PDF
- **Type**: PDF
- **URI**: `pdf://`
- **Default**: Yes

## AI Agent Maintenance Instructions

When maintaining the Printer Manager:

1. **Job Validation**: Validate file paths before submitting jobs
2. **Printer State**: Track printer status accurately (idle, printing, offline)
3. **Job Status**: Properly transition job status (pending → processing → completed/failed)
4. **Default Protection**: Prevent removal of default printer
5. **Job Limits**: Implement job queue limits to prevent resource exhaustion
6. **Printer Drivers**: Validate driver compatibility before assignment

## Testing

Run the unit tests with:

```bash
cargo test --lib printing::printer_manager
```

## Future Enhancements

- Integration with CUPS (Common Unix Printing System)
- IPP (Internet Printing Protocol) client
- Printer auto-discovery via mDNS/Bonjour
- Job prioritization
- Job scheduling and hold/release
- Printer driver management
- Print job preview
- Job history and logging
- Per-user printer permissions
- Printer sharing configuration
