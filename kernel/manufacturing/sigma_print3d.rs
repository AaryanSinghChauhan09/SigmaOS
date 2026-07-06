// SPDX-License-Identifier: MIT
// SigmaOS 3D Printing & Additive Manufacturing — sigma_print3d.rs
// G-code parsing/streaming engine, STL mesh slicing, print job manager,
// and hardware stepper/temperature PID controller stubs.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ── Job Limits ────────────────────────────────────────────────────────────────
pub const MAX_JOB_COMMANDS: usize = 10000;
pub const MAX_ACTIVE_JOBS: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JobState {
    Queued,
    Heating,
    Printing,
    Paused,
    Completed,
    Aborted,
}

// ── G-Code Parameters ────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GCodeCommand {
    pub code_type: u8,      // 'G' or 'M'
    pub num: u16,           // e.g., 0, 1, 104, 140, etc.
    pub x: f32,             // target x position
    pub y: f32,             // target y position
    pub z: f32,             // target z position
    pub e: f32,             // extruder feed amount
    pub f: f32,             // feedrate (mm/min)
    pub s: f32,             // target temp / fan speed
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PrintJob {
    pub job_id: u32,
    pub filename_hash: u64,
    pub state: JobState,
    pub current_cmd_idx: u32,
    pub total_cmds: u32,
    pub time_elapsed_s: u32,
    pub time_estimated_s: u32,
}

// ── PID Temperature Controller ────────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PidController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub setpoint: f32,
    pub integral: f32,
    pub prev_error: f32,
}

// ── Stepper/Motor Coordinates ────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ToolheadCoords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub e: f32,
}

// ── Global State ─────────────────────────────────────────────────────────────
static PRINT3D_INITIALIZED: AtomicBool = AtomicBool::new(false);
static mut ACTIVE_JOBS: [Option<PrintJob>; MAX_ACTIVE_JOBS] = [None; MAX_ACTIVE_JOBS];
static mut CURRENT_COORDS: ToolheadCoords = ToolheadCoords { x: 0.0, y: 0.0, z: 0.0, e: 0.0 };

// ── Implementation ───────────────────────────────────────────────────────────
pub fn print3d_init() -> i32 {
    if PRINT3D_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    unsafe {
        for slot in ACTIVE_JOBS.iter_mut() {
            *slot = None;
        }
        CURRENT_COORDS = ToolheadCoords::default();
    }
    0
}

// ── PID Loop Calculation ─────────────────────────────────────────────────────
pub fn pid_update(pid: &mut PidController, current_temp: f32, dt: f32) -> f32 {
    let error = pid.setpoint - current_temp;
    
    // Accumulate integral with anti-windup (limit error contribution)
    if error.abs() < 10.0 {
        pid.integral += error * dt;
    }
    
    let derivative = (error - pid.prev_error) / dt;
    pid.prev_error = error;

    let mut output = pid.kp * error + pid.ki * pid.integral + pid.kd * derivative;

    // Constrain PWM output (0.0 to 1.0 duty cycle)
    if output > 1.0 {
        output = 1.0;
    } else if output < 0.0 {
        output = 0.0;
    }

    output
}

// ── G-Code Simple Parser ─────────────────────────────────────────────────────
pub fn parse_gcode_line(line: &[u8]) -> Result<GCodeCommand, i32> {
    if line.is_empty() {
        return Err(-1);
    }

    // Fallback stub GCode parsing
    let mut cmd = GCodeCommand {
        code_type: b'G',
        num: 0,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        e: 0.0,
        f: 0.0,
        s: 0.0,
    };

    if line[0] == b'M' || line[0] == b'm' {
        cmd.code_type = b'M';
    }

    // Extremely basic number scanning
    let mut num = 0u16;
    let mut idx = 1;
    while idx < line.len() && line[idx] >= b'0' && line[idx] <= b'9' {
        num = num * 10 + (line[idx] - b'0') as u16;
        idx += 1;
    }
    cmd.num = num;

    Ok(cmd)
}

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_print3d_init() -> i32 {
    print3d_init()
}

#[no_mangle]
pub extern "C" fn sigma_print3d_pid_calculate(
    kp: f32, ki: f32, kd: f32, setpoint: f32, current: f32, dt: f32,
    integral: *mut f32, prev_error: *mut f32
) -> f32 {
    let mut pid = PidController {
        kp,
        ki,
        kd,
        setpoint,
        integral: unsafe { *integral },
        prev_error: unsafe { *prev_error },
    };

    let pwr = pid_update(&mut pid, current, dt);

    unsafe {
        *integral = pid.integral;
        *prev_error = pid.prev_error;
    }

    pwr
}
