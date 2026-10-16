//! # POSIX Job Control
//!
//! Implements POSIX job control semantics for SigmaOS shells and terminals.
//!
//! ## Features
//! - Process groups and sessions (`setsid`, `setpgrp`, `tcsetpgrp`, `tcgetpgrp`)
//! - Foreground / background job management (`fg`, `bg`)
//! - Stopped-job notification via `SIGTTOU` / `SIGTTIN`
//! - `waitpid` with `WUNTRACED` / `WCONTINUED` flags
//! - Job table with status tracking (Running / Stopped / Done)

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;

extern crate alloc;

// ---------------------------------------------------------------------------
// Signal numbers (subset needed for job control)
// ---------------------------------------------------------------------------

/// Signal sent to a process group when a background process tries to read from
/// the controlling terminal.
pub const SIGTTIN: u32 = 21;

/// Signal sent to a process group when a background process tries to write to
/// the controlling terminal (and `TOSTOP` is set).
pub const SIGTTOU: u32 = 22;

/// Sent when the terminal's foreground process group changes.
pub const SIGTSTP: u32 = 20;

/// Sent to resume a stopped process.
pub const SIGCONT: u32 = 18;

/// Sent when a child process changes state.
pub const SIGCHLD: u32 = 17;

// ---------------------------------------------------------------------------
// waitpid flags
// ---------------------------------------------------------------------------

/// Report status of stopped (not terminated) children.
pub const WUNTRACED: u32 = 2;

/// Report status of continued children.
pub const WCONTINUED: u32 = 8;

/// Do not block; return immediately if no child has changed state.
pub const WNOHANG: u32 = 1;

// ---------------------------------------------------------------------------
// Process / session identifiers
// ---------------------------------------------------------------------------

/// A process identifier.
pub type Pid = i32;

/// A process-group identifier.
pub type Pgid = i32;

/// A session identifier.
pub type Sid = i32;

// ---------------------------------------------------------------------------
// Job status
// ---------------------------------------------------------------------------

/// The lifecycle state of a managed job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    /// The job is currently running in the foreground.
    ForegroundRunning,
    /// The job is running in the background.
    BackgroundRunning,
    /// The job has been suspended (received `SIGTSTP`).
    Stopped,
    /// The job has completed; holds the exit code.
    Done(i32),
}

// ---------------------------------------------------------------------------
// Job descriptor
// ---------------------------------------------------------------------------

/// Describes a single shell job, which may consist of one or more processes
/// connected by a pipeline.
#[derive(Debug, Clone)]
pub struct Job {
    /// Shell-assigned job number (shown in `jobs` output, e.g. `[1]`).
    pub job_id: usize,
    /// The process-group ID of the job's process group.
    pub pgid: Pgid,
    /// PIDs of all processes in the pipeline.
    pub pids: Vec<Pid>,
    /// Human-readable command string.
    pub command: String,
    /// Current status of the job.
    pub status: JobStatus,
    /// Whether this job was started as a background job (`cmd &`).
    pub background: bool,
}

impl Job {
    /// Creates a new `Job` with the given parameters.
    pub fn new(
        job_id: usize,
        pgid: Pgid,
        pids: Vec<Pid>,
        command: String,
        background: bool,
    ) -> Self {
        let status = if background {
            JobStatus::BackgroundRunning
        } else {
            JobStatus::ForegroundRunning
        };
        Self {
            job_id,
            pgid,
            pids,
            command,
            status,
            background,
        }
    }

    /// Returns `true` if the job has finished executing.
    pub fn is_done(&self) -> bool {
        matches!(self.status, JobStatus::Done(_))
    }
}

// ---------------------------------------------------------------------------
// Job table
// ---------------------------------------------------------------------------

/// Maintains the list of active jobs for a shell session.
pub struct JobTable {
    jobs: Vec<Job>,
    next_id: usize,
}

impl JobTable {
    /// Creates an empty `JobTable`.
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_id: 1,
        }
    }

    /// Inserts a new job and returns its assigned job ID.
    pub fn add(&mut self, pgid: Pgid, pids: Vec<Pid>, command: String, background: bool) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.jobs
            .push(Job::new(id, pgid, pids, command, background));
        id
    }

    /// Looks up a job by job ID.
    pub fn get(&self, job_id: usize) -> Option<&Job> {
        self.jobs.iter().find(|j| j.job_id == job_id)
    }

    /// Mutable lookup by job ID.
    pub fn get_mut(&mut self, job_id: usize) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.job_id == job_id)
    }

    /// Looks up a job by its process-group ID.
    pub fn get_by_pgid(&mut self, pgid: Pgid) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.pgid == pgid)
    }

    /// Removes finished jobs and returns them (for the shell to report).
    pub fn reap_done(&mut self) -> Vec<Job> {
        let done: Vec<Job> = self.jobs.iter().filter(|j| j.is_done()).cloned().collect();
        self.jobs.retain(|j| !j.is_done());
        done
    }

    /// Returns an iterator over all jobs.
    pub fn list(&self) -> &[Job] {
        &self.jobs
    }
}

// ---------------------------------------------------------------------------
// Terminal control
// ---------------------------------------------------------------------------

/// Represents a controlling terminal for a session.
pub struct ControllingTerminal {
    /// File descriptor of the terminal device (e.g. `/dev/tty`).
    pub fd: i32,
    /// The session ID this terminal belongs to.
    pub session: Sid,
    /// The process group currently in the foreground.
    pub foreground_pgid: Pgid,
}

impl ControllingTerminal {
    /// Opens the given terminal file descriptor as a controlling terminal.
    pub fn new(fd: i32, session: Sid, foreground_pgid: Pgid) -> Self {
        Self {
            fd,
            session,
            foreground_pgid,
        }
    }

    /// Returns the current foreground process group.
    ///
    /// Analogous to `tcgetpgrp(3)`.
    pub fn tcgetpgrp(&self) -> Pgid {
        self.foreground_pgid
    }

    /// Sets the foreground process group of the terminal.
    ///
    /// Analogous to `tcsetpgrp(3)`. The calling process must be a member of
    /// the session that owns this terminal.
    ///
    /// # Errors
    /// Returns `Err(JobControlError::PermissionDenied)` if the caller is not
    /// in the same session as the terminal.
    pub fn tcsetpgrp(&mut self, pgid: Pgid, caller_sid: Sid) -> Result<(), JobControlError> {
        if caller_sid != self.session {
            return Err(JobControlError::PermissionDenied);
        }
        self.foreground_pgid = pgid;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Session management
// ---------------------------------------------------------------------------

/// A POSIX session groups related process groups under one controlling terminal.
pub struct Session {
    pub sid: Sid,
    pub leader_pid: Pid,
    pub terminal: Option<ControllingTerminal>,
    pub process_groups: Vec<Pgid>,
}

impl Session {
    /// Creates a new session with the given leader PID (which becomes the SID).
    pub fn new(leader_pid: Pid) -> Self {
        Self {
            sid: leader_pid,
            leader_pid,
            terminal: None,
            process_groups: Vec::new(),
        }
    }

    /// Attaches a controlling terminal to this session.
    pub fn set_terminal(&mut self, fd: i32, initial_pgid: Pgid) {
        self.terminal = Some(ControllingTerminal::new(fd, self.sid, initial_pgid));
    }
}

// ---------------------------------------------------------------------------
// fg / bg command implementations
// ---------------------------------------------------------------------------

/// Brings a stopped or background job into the foreground.
///
/// Steps:
/// 1. Send `SIGCONT` to the job's process group.
/// 2. Call `tcsetpgrp` to give the job the terminal.
/// 3. Wait for the job to finish or stop (`waitpid` with `WUNTRACED`).
pub fn fg(
    table: &mut JobTable,
    terminal: &mut ControllingTerminal,
    job_id: usize,
    caller_sid: Sid,
) -> Result<(), JobControlError> {
    let job = table
        .get_mut(job_id)
        .ok_or(JobControlError::NoSuchJob(job_id))?;
    let pgid = job.pgid;
    job.status = JobStatus::ForegroundRunning;
    job.background = false;
    // TODO: send SIGCONT via kill(-pgid, SIGCONT) syscall
    terminal.tcsetpgrp(pgid, caller_sid)?;
    // TODO: waitpid(-pgid, WUNTRACED) and update job status
    Ok(())
}

/// Resumes a stopped job in the background.
///
/// Sends `SIGCONT` to the job's process group without transferring terminal
/// ownership.
pub fn bg(table: &mut JobTable, job_id: usize) -> Result<(), JobControlError> {
    let job = table
        .get_mut(job_id)
        .ok_or(JobControlError::NoSuchJob(job_id))?;
    if job.status != JobStatus::Stopped {
        return Err(JobControlError::NotStopped);
    }
    job.status = JobStatus::BackgroundRunning;
    job.background = true;
    // TODO: send SIGCONT via kill(-pgid, SIGCONT) syscall
    let _ = job.pgid;
    Ok(())
}

// ---------------------------------------------------------------------------
// waitpid wrapper
// ---------------------------------------------------------------------------

/// Exit / stop status returned by [`waitpid`].
#[derive(Debug, Clone)]
pub enum WaitStatus {
    /// Process exited normally with the given exit code.
    Exited(Pid, i32),
    /// Process was terminated by a signal.
    Signaled(Pid, u32),
    /// Process was stopped by a signal (only reported when `WUNTRACED` is set).
    Stopped(Pid, u32),
    /// Process was resumed by `SIGCONT` (only reported when `WCONTINUED` is set).
    Continued(Pid),
    /// No child changed state (only possible with `WNOHANG`).
    NoChange,
}

/// Waits for a child process to change state.
///
/// - `pid == -1`: wait for any child.
/// - `pid < -1`: wait for any child in process group `|pid|`.
/// - `pid == 0`: wait for any child in the caller's process group.
/// - `pid > 0`: wait for the specific child.
///
/// # Flags
/// Use bitwise OR of `WUNTRACED`, `WCONTINUED`, and `WNOHANG`.
pub fn waitpid(pid: Pid, flags: u32) -> Result<WaitStatus, JobControlError> {
    // TODO: invoke the real waitpid(2) syscall and decode the status word.
    let _ = (pid, flags);
    Err(JobControlError::Unimplemented)
}

// ---------------------------------------------------------------------------
// SIGCHLD handler integration
// ---------------------------------------------------------------------------

/// Called from the shell's `SIGCHLD` handler to reap children and update the
/// job table.
pub fn handle_sigchld(table: &mut JobTable) {
    loop {
        match waitpid(-1, WNOHANG | WUNTRACED | WCONTINUED) {
            Ok(WaitStatus::NoChange) | Err(_) => break,
            Ok(WaitStatus::Exited(pid, code)) => {
                update_job_for_pid(table, pid, JobStatus::Done(code));
            }
            Ok(WaitStatus::Stopped(pid, _sig)) => {
                update_job_for_pid(table, pid, JobStatus::Stopped);
            }
            Ok(WaitStatus::Continued(pid)) => {
                update_job_for_pid(table, pid, JobStatus::BackgroundRunning);
            }
            Ok(WaitStatus::Signaled(pid, _sig)) => {
                update_job_for_pid(table, pid, JobStatus::Done(128));
            }
        }
    }
}

fn update_job_for_pid(table: &mut JobTable, pid: Pid, status: JobStatus) {
    if let Some(job) = table.jobs.iter_mut().find(|j| j.pids.contains(&pid)) {
        job.status = status;
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can arise from job-control operations.
#[derive(Debug)]
pub enum JobControlError {
    /// No job with the given ID exists in the table.
    NoSuchJob(usize),
    /// The operation requires the job to be stopped.
    NotStopped,
    /// The caller does not have permission (e.g., different session).
    PermissionDenied,
    /// The underlying syscall is not yet implemented.
    Unimplemented,
}
