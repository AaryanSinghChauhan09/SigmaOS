# Scanner Manager

## Overview

The Scanner Manager provides comprehensive scanner management inspired by Linux Mint's scanner settings and Omarchy's scanner utilities. It supports scanner discovery, scan operations, and image processing with configurable resolution, color modes, and output formats.

## Features

- **Scanner Types**: Flatbed, Sheet-fed, All-in-One, Network
- **Scan Resolutions**: 75, 150, 300, 600, 1200, 2400 DPI
- **Color Modes**: Black & White, Grayscale, Color
- **Output Formats**: PNG, JPEG, TIFF, PDF, BMP
- **Job Status**: Pending, Scanning, Processing, Completed, Failed, Cancelled
- **Device Discovery**: Scan for available scanners (simulated)
- **Job Management**: Submit, cancel, list scan jobs
- **Multi-page Support**: Configure page count for batch scanning
- **Statistics**: Track scanner and job counts

## Components

### ScannerType

```rust
pub enum ScannerType {
    Flatbed,    // Flatbed scanner
    SheetFed,   // Sheet-fed scanner
    AllInOne,   // All-in-one printer/scanner
    Network,    // Network scanner
}
```

### ScanResolution

```rust
pub enum ScanResolution {
    DPI75,      // 75 DPI
    DPI150,     // 150 DPI
    DPI300,     // 300 DPI
    DPI600,     // 600 DPI
    DPI1200,    // 1200 DPI
    DPI2400,    // 2400 DPI
}
```

### ScanColorMode

```rust
pub enum ScanColorMode {
    BlackAndWhite,  // Black and white (1-bit)
    Grayscale,      // Grayscale (8-bit)
    Color,          // Color (24-bit)
}
```

### ScanFormat

```rust
pub enum ScanFormat {
    PNG,    // PNG format
    JPEG,   // JPEG format
    TIFF,   // TIFF format
    PDF,    // PDF format
    BMP,    // BMP format
}
```

### ScanJobStatus

```rust
pub enum ScanJobStatus {
    Pending,     // Job is pending
    Scanning,    // Job is scanning
    Processing,  // Job is processing
    Completed,   // Job is completed
    Failed,      // Job failed
    Cancelled,   // Job was cancelled
}
```

### ScanJob

Represents a scan job with:
- Unique job ID
- Scanner ID
- Output file path
- Resolution
- Color mode
- Output format
- Page count
- Job status
- Creation timestamp

### Scanner

Represents a scanner with:
- Unique scanner ID
- Scanner name
- Scanner type
- URI (SANE device URI)
- Driver
- Availability flag

### ScannerManager

Main management interface with:
- Scanner addition and listing
- Device discovery
- Scan job submission
- Job cancellation
- Job listing and filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::printing::ScannerManager;

let mut manager = ScannerManager::new();

// Scan for devices
let scanners = manager.scan_devices();
for scanner in scanners {
    println!("Scanner: {} ({})", scanner.name, scanner.scanner_type.as_str());
}
```

### Scanner Management

```rust
// Add a scanner
let scanner = Scanner::new(
    "flatbed-001".to_string(),
    "Epson Flatbed".to_string(),
    ScannerType::Flatbed,
    "sane://epson:fw:00:00:00:00:00:00".to_string(),
);
manager.add_scanner(scanner);

// Get a scanner
if let Some(scanner) = manager.get_scanner("flatbed-001") {
    println!("Available: {}", scanner.is_available);
}

// List all scanners
let all = manager.list_scanners();

// List available scanners
let available = manager.list_available();
```

### Scan Job Submission

```rust
// Submit a scan job
let job_id = manager.submit_job(
    "flatbed-001",
    "/tmp/scan.png".to_string(),
    ScanResolution::DPI300,
    ScanColorMode::Color,
    ScanFormat::PNG,
)?;

// Get job status
if let Some(job) = manager.get_job(&job_id) {
    println!("Job status: {}", job.status.as_str());
    println!("Resolution: {} DPI", job.resolution.as_dpi());
    println!("Format: {}", job.format.extension());
}
```

### Job Management

```rust
// List all jobs
let jobs = manager.list_jobs();

// List jobs by status
let pending = manager.list_jobs_by_status(ScanJobStatus::Pending);

// Cancel a job
manager.cancel_job(&job_id)?;
```

### Resolution Selection

```rust
// Get resolution from DPI
let resolution = ScanResolution::from_dpi(600)
    .expect("Invalid DPI");

// Get DPI value
let dpi = resolution.as_dpi();
println!("Resolution: {} DPI", dpi);
```

### Color Mode Selection

```rust
// Get color mode from string
let color_mode = ScanColorMode::from_str("grayscale")
    .expect("Invalid color mode");

// Get display string
let mode_str = color_mode.as_str();
println!("Color mode: {}", mode_str);
```

### Format Selection

```rust
// Get format from string
let format = ScanFormat::from_str("pdf")
    .expect("Invalid format");

// Get file extension
let ext = format.extension();
println!("Output format: {}", ext);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total scanners: {}", stats.total_scanners);
println!("Available: {}", stats.available_count);
println!("Total jobs: {}", stats.total_jobs);
println!("Pending jobs: {}", stats.pending_jobs);
println!("Scanning jobs: {}", stats.scanning_jobs);
```

## Device Discovery

The `scan_devices()` method simulates device discovery by populating the scanner list with common scanner types:

- Epson Flatbed (flatbed-001)
- HP Network Scanner (network-001)
- Canon All-in-One (aio-001)

## AI Agent Maintenance Instructions

When maintaining the Scanner Manager:

1. **URI Validation**: Validate SANE device URIs before adding scanners
2. **Path Validation**: Validate output file paths before submitting jobs
3. **Job Status**: Properly transition job status (pending → scanning → processing → completed/failed)
4. **Resolution Support**: Ensure all supported resolutions are valid
5. **Format Validation**: Validate format compatibility with color mode
6. **Scanner Availability**: Track scanner availability accurately

## Testing

Run the unit tests with:

```bash
cargo test --lib printing::scanner_manager
```

## Future Enhancements

- Integration with SANE (Scanner Access Now Easy)
- Image preview and adjustment
- Multi-page scanning with automatic feeder
- OCR (Optical Character Recognition) integration
- Automatic document feeder (ADF) support
- Scan profiles (text, photo, document)
- Scan job scheduling
- Batch scanning
- Image post-processing (crop, rotate, despeckle)
- Scanner sharing configuration
