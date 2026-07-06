// SPDX-License-Identifier: MIT
// SigmaOS ROS 2 Robotics Integration — sigma_robotics.rs
// DDS middleware on SigmaOS, URDF model loader, trajectory planner,
// sigma-twin digital twin synchronization, and sensor fusion pipeline.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ── DDS (Data Distribution Service) Domain ───────────────────────────────────
pub const DDS_DOMAIN_DEFAULT: u32 = 0;
pub const MAX_DDS_TOPICS:  usize = 128;
pub const MAX_DDS_PARTICIPANTS: usize = 32;
pub const TOPIC_NAME_LEN:  usize = 64;
pub const MAX_PAYLOAD:     usize = 65536;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DdsQos {
    BestEffort,
    Reliable,
    Transient,       // Keep last N samples
    Volatile,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DdsReliability {
    BestEffort,
    Reliable,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DdsTopic {
    pub name:          [u8; TOPIC_NAME_LEN],
    pub type_hash:     u64,       // hash of the message type
    pub qos:           DdsQos,
    pub reliability:   DdsReliability,
    pub history_depth: u16,
    pub publishers:    u8,
    pub subscribers:   u8,
    pub active:        AtomicBool,
    pub msg_count:     AtomicU64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DdsParticipant {
    pub participant_id: u32,
    pub domain_id:      u32,
    pub node_name:      [u8; 32],
    pub namespace:      [u8; 32],
    pub pub_count:      u8,
    pub sub_count:      u8,
    pub active:         AtomicBool,
}

// ── URDF (Unified Robot Description Format) ──────────────────────────────────
pub const MAX_JOINTS:   usize = 64;
pub const MAX_LINKS:    usize = 64;
pub const LINK_NAME_LEN: usize = 32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JointType {
    Revolute,      // rotation around axis (bounded)
    Continuous,    // rotation around axis (unbounded)
    Prismatic,     // linear sliding along axis
    Fixed,         // no motion
    Floating,      // 6-DOF
    Planar,        // 2D translation + rotation
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct JointDescriptor {
    pub name:         [u8; LINK_NAME_LEN],
    pub joint_type:   JointType,
    pub parent_link:  u8,   // index into link table
    pub child_link:   u8,
    pub axis:         Vec3f,
    pub lower_limit:  f32,
    pub upper_limit:  f32,
    pub max_velocity: f32,  // rad/s or m/s
    pub max_effort:   f32,  // N·m or N
    pub damping:      f32,
    pub friction:     f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LinkDescriptor {
    pub name:     [u8; LINK_NAME_LEN],
    pub mass_kg:  f32,
    pub inertia:  [f32; 6],  // Ixx, Ixy, Ixz, Iyy, Iyz, Izz
    pub origin:   Vec3f,
    pub visual:   MeshRef,
    pub collision: MeshRef,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct MeshRef {
    pub mesh_hash: u64,     // hash reference to mesh data
    pub scale:     Vec3f,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RobotModel {
    pub name:        [u8; 32],
    pub joint_count: u8,
    pub link_count:  u8,
    pub dof:         u8,     // degrees of freedom
    pub joints:      [JointDescriptor; MAX_JOINTS],
    pub links:       [LinkDescriptor; MAX_LINKS],
}

// ── Trajectory Planning ──────────────────────────────────────────────────────
pub const MAX_WAYPOINTS: usize = 256;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct JointState {
    pub positions:    [f32; MAX_JOINTS],
    pub velocities:   [f32; MAX_JOINTS],
    pub efforts:      [f32; MAX_JOINTS],
    pub joint_count:  u8,
    pub timestamp_ns: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TrajectoryPoint {
    pub positions:    [f32; MAX_JOINTS],
    pub velocities:   [f32; MAX_JOINTS],
    pub accelerations:[f32; MAX_JOINTS],
    pub joint_count:  u8,
    pub time_from_start_ms: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlannerAlgorithm {
    RRTStar,         // Rapidly-exploring Random Tree*
    PRM,             // Probabilistic Roadmap
    TrapezoidalVel,  // Trapezoidal velocity profile
    CubicSpline,     // Cubic spline interpolation
    QuinticPoly,     // Quintic polynomial (smooth jerk)
}

// ── Sensor Fusion ────────────────────────────────────────────────────────────
pub const MAX_SENSORS: usize = 32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SensorType {
    IMU,        // Inertial Measurement Unit (accel + gyro + mag)
    Lidar2D,
    Lidar3D,
    Camera,
    DepthCamera,
    Encoder,    // joint encoder
    ForceTorque,
    GPS,
    Ultrasonic,
    Infrared,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SensorDescriptor {
    pub sensor_id:   u8,
    pub sensor_type: SensorType,
    pub name:        [u8; 32],
    pub rate_hz:     u16,
    pub frame_id:    [u8; 16],   // TF frame name
    pub enabled:     AtomicBool,
    pub sample_count: AtomicU64,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ImuReading {
    pub accel:     Vec3f,    // m/s²
    pub gyro:      Vec3f,    // rad/s
    pub mag:       Vec3f,    // µT
    pub timestamp_ns: u64,
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct FusedPose {
    pub position:    Vec3f,
    pub orientation: [f32; 4],  // quaternion (w, x, y, z)
    pub linear_vel:  Vec3f,
    pub angular_vel: Vec3f,
    pub timestamp_ns: u64,
    pub confidence:  f32,       // 0.0–1.0
}

// ── Digital Twin (sigma-twin) ────────────────────────────────────────────────
pub const MAX_TWINS: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TwinSyncMode {
    RealTimeStream,   // continuous joint state streaming
    SnapshotOnChange, // sync only when state changes
    Periodic(u32),    // sync every N ms
    Manual,           // sync on explicit command
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DigitalTwin {
    pub twin_id:      u32,
    pub robot_name:   [u8; 32],
    pub sync_mode:    TwinSyncMode,
    pub last_sync_ns: AtomicU64,
    pub divergence:   AtomicU32,  // max joint error ×1000
    pub active:       AtomicBool,
}

// ── Global State ─────────────────────────────────────────────────────────────
static mut TOPIC_TABLE:  [Option<DdsTopic>; MAX_DDS_TOPICS]     = [None; MAX_DDS_TOPICS];
static mut PARTICIPANT_TABLE: [Option<DdsParticipant>; MAX_DDS_PARTICIPANTS] = [None; MAX_DDS_PARTICIPANTS];
static mut SENSOR_TABLE: [Option<SensorDescriptor>; MAX_SENSORS] = [None; MAX_SENSORS];
static mut TWIN_TABLE:   [Option<DigitalTwin>; MAX_TWINS]       = [None; MAX_TWINS];
static mut FUSED_POSE:   FusedPose = FusedPose {
    position: Vec3f { x: 0.0, y: 0.0, z: 0.0 },
    orientation: [1.0, 0.0, 0.0, 0.0],
    linear_vel: Vec3f { x: 0.0, y: 0.0, z: 0.0 },
    angular_vel: Vec3f { x: 0.0, y: 0.0, z: 0.0 },
    timestamp_ns: 0,
    confidence: 0.0,
};

static ROBOTICS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static TOPIC_COUNT:     AtomicU32 = AtomicU32::new(0);
static PARTICIPANT_CNT: AtomicU32 = AtomicU32::new(0);

// ── Initialization ───────────────────────────────────────────────────────────
pub fn robotics_init() -> i32 {
    if ROBOTICS_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    unsafe {
        for slot in TOPIC_TABLE.iter_mut()       { *slot = None; }
        for slot in PARTICIPANT_TABLE.iter_mut()  { *slot = None; }
        for slot in SENSOR_TABLE.iter_mut()       { *slot = None; }
        for slot in TWIN_TABLE.iter_mut()         { *slot = None; }
    }
    TOPIC_COUNT.store(0, Ordering::SeqCst);
    PARTICIPANT_CNT.store(0, Ordering::SeqCst);
    0
}

// ── DDS Topic Management ─────────────────────────────────────────────────────
pub fn dds_create_topic(name: &[u8], type_hash: u64, qos: DdsQos,
                        reliability: DdsReliability, history: u16) -> i32 {
    let idx = TOPIC_COUNT.fetch_add(1, Ordering::SeqCst);
    if idx as usize >= MAX_DDS_TOPICS {
        TOPIC_COUNT.fetch_sub(1, Ordering::SeqCst);
        return -1;
    }
    let mut name_buf = [0u8; TOPIC_NAME_LEN];
    let copy_len = name.len().min(TOPIC_NAME_LEN);
    name_buf[..copy_len].copy_from_slice(&name[..copy_len]);

    unsafe {
        TOPIC_TABLE[idx as usize] = Some(DdsTopic {
            name: name_buf,
            type_hash,
            qos,
            reliability,
            history_depth: history,
            publishers: 0,
            subscribers: 0,
            active: AtomicBool::new(true),
            msg_count: AtomicU64::new(0),
        });
    }
    idx as i32
}

pub fn dds_publish(topic_idx: u32, _payload: &[u8]) -> i32 {
    if topic_idx as usize >= MAX_DDS_TOPICS { return -1; }
    unsafe {
        if let Some(ref topic) = TOPIC_TABLE[topic_idx as usize] {
            if !topic.active.load(Ordering::Relaxed) { return -2; }
            topic.msg_count.fetch_add(1, Ordering::Relaxed);
            // In a real implementation, payload would be dispatched to all
            // subscribers via sigma-bus IPC channels.
            return 0;
        }
    }
    -1
}

// ── Sensor Fusion (Complementary Filter) ─────────────────────────────────────
pub fn sensor_register(sensor_id: u8, sensor_type: SensorType,
                       name: &[u8], rate_hz: u16, frame_id: &[u8]) -> i32 {
    if sensor_id as usize >= MAX_SENSORS { return -1; }
    let mut name_buf = [0u8; 32];
    let mut frame_buf = [0u8; 16];
    let nc = name.len().min(32);
    let fc = frame_id.len().min(16);
    name_buf[..nc].copy_from_slice(&name[..nc]);
    frame_buf[..fc].copy_from_slice(&frame_id[..fc]);

    unsafe {
        SENSOR_TABLE[sensor_id as usize] = Some(SensorDescriptor {
            sensor_id, sensor_type, name: name_buf,
            rate_hz, frame_id: frame_buf,
            enabled: AtomicBool::new(true),
            sample_count: AtomicU64::new(0),
        });
    }
    0
}

/// Fuse IMU reading into the global pose estimate using a complementary filter.
/// alpha: weighting factor for gyro (0.0–1.0, typically 0.98)
pub fn sensor_fuse_imu(reading: &ImuReading, alpha: f32, dt_s: f32) -> i32 {
    unsafe {
        // Simple complementary filter for pitch/roll from accel + gyro
        // pitch_acc = atan2(ay, az)
        // roll_acc  = atan2(ax, az)
        // This is a no_std approximation
        let accel_pitch = approx_atan2(reading.accel.y, reading.accel.z);
        let accel_roll  = approx_atan2(reading.accel.x, reading.accel.z);

        // Integrate gyro
        let gyro_pitch = FUSED_POSE.orientation[1] + reading.gyro.x * dt_s;
        let gyro_roll  = FUSED_POSE.orientation[2] + reading.gyro.y * dt_s;
        let gyro_yaw   = FUSED_POSE.orientation[3] + reading.gyro.z * dt_s;

        // Complementary filter blend
        FUSED_POSE.orientation[1] = alpha * gyro_pitch + (1.0 - alpha) * accel_pitch;
        FUSED_POSE.orientation[2] = alpha * gyro_roll  + (1.0 - alpha) * accel_roll;
        FUSED_POSE.orientation[3] = gyro_yaw; // yaw from gyro only (no mag correction here)
        FUSED_POSE.orientation[0] = 1.0; // w component (simplified)

        // Integrate position from acceleration (double integration)
        FUSED_POSE.linear_vel.x += reading.accel.x * dt_s;
        FUSED_POSE.linear_vel.y += reading.accel.y * dt_s;
        FUSED_POSE.linear_vel.z += (reading.accel.z - 9.81) * dt_s; // subtract gravity

        FUSED_POSE.position.x += FUSED_POSE.linear_vel.x * dt_s;
        FUSED_POSE.position.y += FUSED_POSE.linear_vel.y * dt_s;
        FUSED_POSE.position.z += FUSED_POSE.linear_vel.z * dt_s;

        FUSED_POSE.angular_vel = reading.gyro;
        FUSED_POSE.timestamp_ns = reading.timestamp_ns;
        FUSED_POSE.confidence = alpha;
    }
    0
}

/// Approximate atan2 for no_std (Remez minimax polynomial)
fn approx_atan2(y: f32, x: f32) -> f32 {
    if x == 0.0 && y == 0.0 { return 0.0; }
    let abs_y = if y < 0.0 { -y } else { y };
    let abs_x = if x < 0.0 { -x } else { x };
    let (a, b) = if abs_x > abs_y { (abs_x, abs_y) } else { (abs_y, abs_x) };
    let r = b / a;
    let angle = r * (0.9998660 + r * r * (-0.3302995 + r * r * 0.1801410));
    let angle = if abs_x < abs_y { 1.5707963 - angle } else { angle };
    let angle = if x < 0.0 { 3.1415927 - angle } else { angle };
    if y < 0.0 { -angle } else { angle }
}

// ── Trajectory Planning (Trapezoidal Velocity Profile) ───────────────────────
pub fn trajectory_plan_trapezoidal(
    start: &[f32], goal: &[f32], joint_count: u8,
    max_vel: f32, max_accel: f32,
    output: &mut [TrajectoryPoint], max_points: usize
) -> i32 {
    let jc = joint_count as usize;
    if jc > MAX_JOINTS || max_points == 0 { return -1; }

    // Find the joint requiring the longest travel
    let mut max_dist: f32 = 0.0;
    for i in 0..jc {
        let d = if goal[i] > start[i] { goal[i] - start[i] } else { start[i] - goal[i] };
        if d > max_dist { max_dist = d; }
    }
    if max_dist < 0.001 { return 0; } // already at goal

    // Trapezoidal profile timing
    let t_accel = max_vel / max_accel;
    let d_accel = 0.5 * max_accel * t_accel * t_accel;

    let (t_total, t_cruise) = if 2.0 * d_accel >= max_dist {
        // Triangular profile (no cruise phase)
        let t = (max_dist / max_accel).sqrt() * 1.4142; // sqrt(2)
        (t, 0.0f32)
    } else {
        let cruise_dist = max_dist - 2.0 * d_accel;
        let tc = cruise_dist / max_vel;
        (2.0 * t_accel + tc, tc)
    };

    // Generate waypoints
    let dt = t_total / (max_points as f32 - 1.0).max(1.0);
    let mut count = 0usize;

    for idx in 0..max_points {
        let t = dt * idx as f32;
        if t > t_total { break; }

        let frac = if t < t_accel {
            // Acceleration phase
            0.5 * max_accel * t * t / max_dist
        } else if t < t_accel + t_cruise {
            // Cruise phase
            let dt2 = t - t_accel;
            (d_accel + max_vel * dt2) / max_dist
        } else {
            // Deceleration phase
            let dt3 = t - t_accel - t_cruise;
            (d_accel + max_vel * t_cruise + max_vel * dt3
             - 0.5 * max_accel * dt3 * dt3) / max_dist
        };
        let frac = if frac > 1.0 { 1.0 } else { frac };

        let mut pt = TrajectoryPoint {
            positions: [0.0; MAX_JOINTS],
            velocities: [0.0; MAX_JOINTS],
            accelerations: [0.0; MAX_JOINTS],
            joint_count,
            time_from_start_ms: (t * 1000.0) as u32,
        };
        for j in 0..jc {
            pt.positions[j] = start[j] + frac * (goal[j] - start[j]);
        }
        output[count] = pt;
        count += 1;
    }

    count as i32
}

// ── Digital Twin Sync ────────────────────────────────────────────────────────
pub fn twin_create(twin_id: u32, robot_name: &[u8], sync_mode: TwinSyncMode) -> i32 {
    if twin_id as usize >= MAX_TWINS { return -1; }
    let mut name_buf = [0u8; 32];
    let nc = robot_name.len().min(32);
    name_buf[..nc].copy_from_slice(&robot_name[..nc]);

    unsafe {
        TWIN_TABLE[twin_id as usize] = Some(DigitalTwin {
            twin_id,
            robot_name: name_buf,
            sync_mode,
            last_sync_ns: AtomicU64::new(0),
            divergence: AtomicU32::new(0),
            active: AtomicBool::new(true),
        });
    }
    0
}

pub fn twin_sync_state(twin_id: u32, _joint_state: &JointState, now_ns: u64) -> i32 {
    if twin_id as usize >= MAX_TWINS { return -1; }
    unsafe {
        if let Some(ref twin) = TWIN_TABLE[twin_id as usize] {
            if !twin.active.load(Ordering::Relaxed) { return -2; }
            twin.last_sync_ns.store(now_ns, Ordering::SeqCst);
            // In a real system, this would serialize joint_state and
            // send it to the digital twin visualization via sigma-bus IPC
            return 0;
        }
    }
    -1
}

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_robotics_init() -> i32 { robotics_init() }

#[no_mangle]
pub extern "C" fn sigma_dds_create_topic(
    name: *const u8, name_len: usize, type_hash: u64,
    qos: u8, reliability: u8, history: u16
) -> i32 {
    let q = match qos {
        0 => DdsQos::BestEffort,
        1 => DdsQos::Reliable,
        2 => DdsQos::Transient,
        _ => DdsQos::Volatile,
    };
    let r = if reliability == 0 { DdsReliability::BestEffort } else { DdsReliability::Reliable };
    let name_slice = unsafe { core::slice::from_raw_parts(name, name_len) };
    dds_create_topic(name_slice, type_hash, q, r, history)
}

#[no_mangle]
pub extern "C" fn sigma_dds_publish(topic_idx: u32, payload: *const u8, len: usize) -> i32 {
    let data = unsafe { core::slice::from_raw_parts(payload, len) };
    dds_publish(topic_idx, data)
}

#[no_mangle]
pub extern "C" fn sigma_robotics_fuse_imu(
    ax: f32, ay: f32, az: f32,
    gx: f32, gy: f32, gz: f32,
    mx: f32, my: f32, mz: f32,
    ts: u64, alpha: f32, dt: f32
) -> i32 {
    let reading = ImuReading {
        accel: Vec3f { x: ax, y: ay, z: az },
        gyro:  Vec3f { x: gx, y: gy, z: gz },
        mag:   Vec3f { x: mx, y: my, z: mz },
        timestamp_ns: ts,
    };
    sensor_fuse_imu(&reading, alpha, dt)
}

#[no_mangle]
pub extern "C" fn sigma_twin_create(twin_id: u32, name: *const u8, name_len: usize, sync: u8) -> i32 {
    let mode = match sync {
        0 => TwinSyncMode::RealTimeStream,
        1 => TwinSyncMode::SnapshotOnChange,
        3 => TwinSyncMode::Manual,
        n => TwinSyncMode::Periodic(n as u32 * 100),
    };
    let n = unsafe { core::slice::from_raw_parts(name, name_len) };
    twin_create(twin_id, n, mode)
}
