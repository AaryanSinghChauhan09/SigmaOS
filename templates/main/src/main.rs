//! # COMPONENT_NAME — CLI entry point
//!
//! Binary entry point for the `COMPONENT_NAME` SigmaOS component.
//!
//! Exit codes follow the POSIX convention:
//!   0  — success
//!   1  — general / runtime error
//!   2  — usage / argument error (handled automatically by clap)
//!  64  — configuration error (EX_USAGE)
//!  70  — internal software error (EX_SOFTWARE)
//!  78  — configuration error (EX_CONFIG)

// ---------------------------------------------------------------------------
// Compiler directives
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// ---------------------------------------------------------------------------
// Imports
// ---------------------------------------------------------------------------

use std::process;

use clap::{Parser, Subcommand};
use log::LevelFilter;

use COMPONENT_NAME_lib::{ComponentName, Config, Error};

// ---------------------------------------------------------------------------
// Exit code constants
// ---------------------------------------------------------------------------

/// Successful termination.
const EXIT_SUCCESS: i32 = 0;
/// General runtime error.
const EXIT_FAILURE: i32 = 1;
/// Configuration error.
const EXIT_CONFIG_ERROR: i32 = 78;
/// Internal software error (bug).
const EXIT_SOFTWARE_ERROR: i32 = 70;

// ---------------------------------------------------------------------------
// CLI definition (clap derive API)
// ---------------------------------------------------------------------------

/// COMPONENT_NAME — SigmaOS component.
///
/// <!-- TODO: one-line description of what this binary does -->
#[derive(Debug, Parser)]
#[command(
    name = "COMPONENT_NAME",
    version,
    author,
    about,
    long_about = None,
    propagate_version = true,
)]
struct Cli {
    /// Enable verbose output. Repeat for increased verbosity (-vv, -vvv).
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Path to configuration file (JSON).
    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

/// Available sub-commands.
#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the component's primary operation.
    Run {
        /// Dry-run mode: validate and plan without executing side-effects.
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate the configuration file and exit.
    Check,

    /// Print version information and exit.
    Version,
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();

    // Initialise logging before anything else.
    init_logging(cli.verbose);

    log::debug!("COMPONENT_NAME starting; args parsed");

    // Dispatch and handle errors uniformly.
    let exit_code = match run(cli) {
        Ok(()) => EXIT_SUCCESS,
        Err(e) => handle_error(e),
    };

    process::exit(exit_code);
}

// ---------------------------------------------------------------------------
// Top-level dispatcher
// ---------------------------------------------------------------------------

/// Execute the selected sub-command.
///
/// # Errors
///
/// Propagates any [`Error`] returned by sub-command handlers.
fn run(cli: Cli) -> Result<(), Error> {
    // Load configuration: file > defaults.
    let config = load_config(cli.config)?;

    match cli.command {
        Commands::Run { dry_run } => cmd_run(config, dry_run),
        Commands::Check => cmd_check(config),
        Commands::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Sub-command handlers
// ---------------------------------------------------------------------------

/// Execute the primary run operation.
fn cmd_run(config: Config, dry_run: bool) -> Result<(), Error> {
    log::info!("cmd: run (dry_run={})", dry_run);

    let component = ComponentName::new(config)?;

    if dry_run {
        log::info!("Dry-run: configuration is valid, skipping execution.");
        println!("[dry-run] Configuration valid. No side-effects executed.");
        return Ok(());
    }

    component.run()
}

/// Validate configuration and report result.
fn cmd_check(config: Config) -> Result<(), Error> {
    log::info!("cmd: check");
    config.validate()?;
    println!("Configuration OK.");
    Ok(())
}

// ---------------------------------------------------------------------------
// Configuration loading
// ---------------------------------------------------------------------------

/// Load [`Config`] from an optional file path, falling back to defaults.
///
/// # Errors
///
/// Returns [`Error::Io`] if the file cannot be read, or [`Error::Serde`] if
/// the JSON is malformed.
fn load_config(path: Option<std::path::PathBuf>) -> Result<Config, Error> {
    match path {
        Some(p) => {
            log::debug!("Loading config from {}", p.display());
            let contents = std::fs::read_to_string(&p)?;
            let cfg: Config = serde_json::from_str(&contents)?;
            log::debug!("Config loaded successfully");
            Ok(cfg)
        }
        None => {
            log::debug!("No config file specified; using defaults");
            Ok(Config::default())
        }
    }
}

// ---------------------------------------------------------------------------
// Logging initialisation
// ---------------------------------------------------------------------------

/// Initialise `env_logger` with a level derived from the `--verbose` flag.
///
/// Verbosity levels:
///   0 — Warn
///   1 — Info  (-v)
///   2 — Debug (-vv)
///   3+ — Trace (-vvv)
fn init_logging(verbosity: u8) {
    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter_level(level)
        // Respect RUST_LOG env var if set (it takes precedence over --verbose).
        .parse_default_env()
        .format_timestamp_millis()
        .init();

    log::trace!("Logging initialised at level: {}", level);
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

/// Map an [`Error`] to a POSIX exit code and print a human-readable message.
fn handle_error(err: Error) -> i32 {
    match &err {
        Error::InvalidConfig(_) => {
            eprintln!("error: configuration invalid — {err}");
            EXIT_CONFIG_ERROR
        }
        Error::Io(io_err) => {
            eprintln!("error: I/O failure — {io_err}");
            EXIT_FAILURE
        }
        Error::Serde(serde_err) => {
            eprintln!("error: parse failure — {serde_err}");
            EXIT_FAILURE
        }
        Error::Internal(msg) => {
            eprintln!("error: internal bug — {msg}");
            eprintln!("Please report this at https://github.com/sigmaos/COMPONENT_NAME/issues");
            EXIT_SOFTWARE_ERROR
        }
        _ => {
            eprintln!("error: {err}");
            EXIT_FAILURE
        }
    }
}
