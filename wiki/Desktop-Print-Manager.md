# Desktop Print Manager

## Overview

The Desktop Print Manager provides comprehensive print management inspired by Linux Mint's print settings and Omarchy's print utilities. It supports printer management, print job tracking, printer status monitoring, and advanced print options.

## Features

- **Printer Status**: Idle, Printing, Stopped, Error, Offline
- **Print Job Status**: Pending, Processing, Completed, Canceled, Failed
- **Printer Management**: Add, remove, and manage printers
- **Print Job Management**: Add, remove, and track print jobs
- **Default Printer**: Set and track default printer
- **Printer Sharing**: Enable/disable printer sharing
- **Printer Status Updates**: Update printer status in real-time
- **Print Job Options**: Pages, copies, color mode, duplex
- **Job Filtering**: Filter jobs by printer or status
- **Job Cancellation**: Cancel pending or processing jobs
- **Statistics**: Track printer counts, job counts, and status distribution

## Components

### PrinterStatus

```rust
pub enum PrinterStatus {
    Idle,      // Printer is idle
    Printing,  // Printer is printing
    Stopped,   // Printer is stopped
    Error,     // Printer has an error
    Offline,   // Printer is offline
}
```

### PrintJobStatus

```rust
pub enum PrintJobStatus {
    Pending,    // Job is pending
    Processing, // Job is processing
    Completed,  // Job is completed
    Canceled,   // Job is canceled
    Failed,     // Job failed
}
```

### Printer

Printer with:
- Printer ID and name
- Location
- Make and model
- URI (IPP, USB, etc.)
- Status
- Default flag
- Shared flag

### PrintJob

Print job with:
- Job ID and title
- Printer ID
- File path
- Status
- Pages
- Copies
- Color mode
- Duplex

### DesktopPrintManager

Main management interface with:
- Printer management (add, remove, retrieve)
- Print job management
- Default printer configuration
- Printer sharing configuration
- Printer status updates
- Print job status updates
- Job cancellation
- Job filtering by printer or status
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopPrintManager;

let mut manager = DesktopPrintManager::new();

// Get configuration
println!("Total printers: {}", manager.get_printers().len());
println!("Total jobs: {}", manager.get_print_jobs().len());
println!("Default printer: {:?}", manager.get_default_printer());
```

### Printer Management

```rust
// Add printer
let printer = Printer::new(
    "custom".to_string(),
    "Custom Printer".to_string(),
    "ipp://localhost:631/printers/custom".to_string(),
)
.with_location("Office".to_string())
.with_make_and_model("HP LaserJet Pro".to_string());

let id = manager.add_printer(printer);

// Remove printer
manager.remove_printer(&id);
```

### Printer Status

```rust
// Set printer status
manager.set_printer_status("printer_0", PrinterStatus::Printing);
manager.set_printer_status("printer_0", PrinterStatus::Idle);
```

### Default Printer

```rust
// Set default printer
manager.set_default_printer(&printer_id);

// Get default printer
if let Some(default) = manager.get_default_printer() {
    println!("Default printer: {}", default.name);
}
```

### Printer Sharing

```rust
// Enable printer sharing
manager.set_printer_shared("printer_0", true);

// Disable printer sharing
manager.set_printer_shared("printer_0", false);
```

### Print Job Management

```rust
// Add print job
let job = PrintJob::new(
    "job".to_string(),
    "printer_0".to_string(),
    "Document".to_string(),
    "/tmp/document.pdf".to_string(),
)
.with_pages(5)
.with_copies(2)
.with_color_mode(true)
.with_duplex(true);

let id = manager.add_print_job(job);

// Remove print job
manager.remove_print_job(&id);
```

### Print Job Status

```rust
// Set print job status
manager.set_print_job_status(&job_id, PrintJobStatus::Processing);
manager.set_print_job_status(&job_id, PrintJobStatus::Completed);
```

### Job Cancellation

```rust
// Cancel print job
manager.cancel_print_job(&job_id);
```

### Job Filtering

```rust
// Get jobs by printer
let printer_jobs = manager.get_print_jobs_by_printer("printer_0");

// Get jobs by status
let pending_jobs = manager.get_print_jobs_by_status(PrintJobStatus::Pending);
let completed_jobs = manager.get_print_jobs_by_status(PrintJobStatus::Completed);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total printers: {}", stats.total_printers);
println!("Idle printers: {}", stats.idle_printers);
println!("Printing printers: {}", stats.printing_printers);
println!("Error printers: {}", stats.error_printers);
println!("Total jobs: {}", stats.total_jobs);
println!("Pending jobs: {}", stats.pending_jobs);
println!("Processing jobs: {}", stats.processing_jobs);
println!("Default printer set: {}", stats.default_printer_set);
```

## Default Configuration

The Print Manager includes default configuration:

- **Default Printer**: 1 default printer (idle status)
- **Default Printer Set**: true

## AI Agent Maintenance Instructions

When maintaining the Print Manager:

1. **CUPS Integration**: Integrate with CUPS (Common Unix Printing System)
2. **Printer Discovery**: Implement automatic printer discovery
3. **Driver Management**: Add printer driver management
4. **Print Preview**: Add print preview functionality
5. **Print Queue UI**: Integrate with print queue UI
6. **IPP Support**: Add full IPP (Internet Printing Protocol) support
7. **Network Printers**: Add network printer configuration
8. **PDF Printing**: Add direct PDF printing
9. **Print to File**: Add print-to-file functionality
10. **Job Prioritization**: Add job priority and reordering

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::print_manager
```

## Future Enhancements

- CUPS integration for actual print management
- Automatic printer discovery
- Printer driver management
- Print preview functionality
- Print queue UI integration
- Full IPP support
- Network printer configuration
- Direct PDF printing
- Print-to-file functionality
- Job priority and reordering
