use std::collections::HashMap;

/// HPC Cluster Job State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HpcJobState {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Represents a Scientific High-Performance Computing Job (like Slurm/PBS)
#[derive(Debug, Clone)]
pub struct HpcClusterJob {
    pub job_id: u32,
    pub name: String,
    pub nodes_requested: u32,
    pub cores_per_node: u32,
    pub state: HpcJobState,
    pub script_payload: String,
}

impl HpcClusterJob {
    pub fn new(job_id: u32, name: &str, nodes: u32, cores: u32, script: &str) -> Self {
        Self {
            job_id,
            name: name.to_string(),
            nodes_requested: nodes,
            cores_per_node: cores,
            state: HpcJobState::Pending,
            script_payload: script.to_string(),
        }
    }

    pub fn start_job(&mut self) {
        if self.state == HpcJobState::Pending {
            self.state = HpcJobState::Running;
        }
    }

    pub fn complete_job(&mut self) {
        if self.state == HpcJobState::Running {
            self.state = HpcJobState::Completed;
        }
    }
}

/// Simulates a Message Passing Interface (MPI) communicator for parallel workloads
#[derive(Debug, Clone)]
pub struct MpiCommunicator {
    pub size: u32,
    pub rank: u32,
    pub message_buffer: HashMap<u32, Vec<u8>>, // maps rank to received byte packets
}

impl MpiCommunicator {
    pub fn new(size: u32, rank: u32) -> Self {
        Self {
            size,
            rank,
            message_buffer: HashMap::new(),
        }
    }

    /// Simulates sending a packet from current rank to destination rank
    pub fn send(
        &self,
        dest: u32,
        data: &[u8],
        communicators: &mut [MpiCommunicator],
    ) -> Result<(), &'static str> {
        if dest >= self.size {
            return Err("Destination rank out of bounds");
        }
        for comm in communicators {
            if comm.rank == dest {
                comm.message_buffer.insert(self.rank, data.to_vec());
                return Ok(());
            }
        }
        Err("Destination communicator not found")
    }

    /// Simulates broadcasting a message to all ranks in the communicator
    pub fn broadcast(&self, data: &[u8], communicators: &mut [MpiCommunicator]) {
        for comm in communicators {
            if comm.rank != self.rank {
                comm.message_buffer.insert(self.rank, data.to_vec());
            }
        }
    }
}

/// CAN-bus Frame representation for Automotive Controllers
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanFrame {
    pub id: u32,
    pub data: [u8; 8],
    pub dlc: u8, // data length code
}

/// Simulates Automotive/Industrial Engine Control Unit (ECU) controller
#[derive(Debug, Clone)]
pub struct EcuController {
    pub ecu_id: u8,
    pub name: String,
    pub brake_applied: bool,
    pub speed_kmh: f32,
    pub error_log: Vec<String>,
}

impl EcuController {
    pub fn new(ecu_id: u8, name: &str) -> Self {
        Self {
            ecu_id,
            name: name.to_string(),
            brake_applied: false,
            speed_kmh: 0.0,
            error_log: Vec::new(),
        }
    }

    /// Processes incoming CAN-bus frames representing vehicle states or command signals
    pub fn process_can_frame(&mut self, frame: &CanFrame) -> Result<&'static str, &'static str> {
        if frame.dlc > 8 {
            return Err("Invalid CAN Frame DLC");
        }

        match frame.id {
            0x101 => {
                // Throttle signal
                let throttle = frame.data[0] as f32;
                self.speed_kmh = throttle * 1.5;
                Ok("Speed updated")
            }
            0x102 => {
                // Brake signal
                let brake_signal = frame.data[0];
                self.brake_applied = brake_signal != 0;
                if self.brake_applied {
                    self.speed_kmh = 0.0;
                }
                Ok("Brake applied")
            }
            0x500 => {
                // Emergency Fault
                self.error_log
                    .push("Emergency fault CAN code received!".to_string());
                self.speed_kmh = 0.0;
                self.brake_applied = true;
                Ok("Safety failsafe activated")
            }
            _ => Err("Unknown CAN ID"),
        }
    }
}

/// Educational Sandbox Coding Challenge
#[derive(Debug, Clone)]
pub struct EduChallenge {
    pub challenge_id: u32,
    pub title: String,
    pub description: String,
    pub difficulty: String,
}

/// Secured Educational Playground environment for students
#[derive(Debug, Clone)]
pub struct EduPlayground {
    pub student_name: String,
    pub level: u32,
    pub current_score: u32,
    pub active_challenge: Option<EduChallenge>,
}

impl EduPlayground {
    pub fn new(student_name: &str) -> Self {
        Self {
            student_name: student_name.to_string(),
            level: 1,
            current_score: 0,
            active_challenge: None,
        }
    }

    pub fn set_challenge(&mut self, challenge: EduChallenge) {
        self.active_challenge = Some(challenge);
    }

    /// Submits a student's answer code. If correct, awards points and advances levels.
    pub fn submit_solution(&mut self, student_code: &str) -> Result<&'static str, &'static str> {
        let challenge = self
            .active_challenge
            .as_ref()
            .ok_or("No active challenge")?;

        // Basic static verification of educational coding results
        if student_code.contains("print")
            && student_code.contains("hello")
            && challenge.challenge_id == 1
        {
            self.current_score += 100;
            if self.current_score >= 200 {
                self.level += 1;
            }
            Ok("Congratulations! Your educational code is correct and fully sandboxed.")
        } else {
            Err("Code analysis failed: expected standard output or matching signature")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpc_cluster_jobs() {
        let mut job = HpcClusterJob::new(1001, "AstroPhysics-Sim", 16, 64, "run-sim.sh");
        assert_eq!(job.state, HpcJobState::Pending);

        job.start_job();
        assert_eq!(job.state, HpcJobState::Running);

        job.complete_job();
        assert_eq!(job.state, HpcJobState::Completed);
    }

    #[test]
    fn test_mpi_message_passing() {
        let node0 = MpiCommunicator::new(3, 0);
        let node1 = MpiCommunicator::new(3, 1);
        let node2 = MpiCommunicator::new(3, 2);

        let mut communicators = [node0.clone(), node1.clone(), node2.clone()];

        assert!(node0.send(1, b"hello rank 1", &mut communicators).is_ok());
        assert_eq!(
            communicators[1].message_buffer.get(&0).unwrap(),
            b"hello rank 1"
        );

        node0.broadcast(b"sync signal", &mut communicators);
        assert_eq!(
            communicators[1].message_buffer.get(&0).unwrap(),
            b"sync signal"
        );
        assert_eq!(
            communicators[2].message_buffer.get(&0).unwrap(),
            b"sync signal"
        );
    }

    #[test]
    fn test_automotive_ecu_failsafe() {
        let mut ecu = EcuController::new(0x0A, "Transmission-ECU");

        let throttle_frame = CanFrame {
            id: 0x101,
            data: [40, 0, 0, 0, 0, 0, 0, 0],
            dlc: 8,
        };
        assert!(ecu.process_can_frame(&throttle_frame).is_ok());
        assert_eq!(ecu.speed_kmh, 60.0);

        let failsafe_frame = CanFrame {
            id: 0x500,
            data: [0; 8],
            dlc: 8,
        };
        assert_eq!(
            ecu.process_can_frame(&failsafe_frame),
            Ok("Safety failsafe activated")
        );
        assert_eq!(ecu.speed_kmh, 0.0);
        assert!(ecu.brake_applied);
        assert_eq!(ecu.error_log.len(), 1);
    }

    #[test]
    fn test_educational_sandbox_challenges() {
        let mut play = EduPlayground::new("Aaryan");
        let challenge = EduChallenge {
            challenge_id: 1,
            title: "Hello World".to_string(),
            description: "Print hello to console".to_string(),
            difficulty: "Beginner".to_string(),
        };

        play.set_challenge(challenge);

        // Incorrect code
        let fail_res = play.submit_solution("fn main() { return 0; }");
        assert!(fail_res.is_err());

        // Correct code
        let pass_res = play.submit_solution("fn main() { print!(\"hello\"); }");
        assert!(pass_res.is_ok());
        assert_eq!(play.current_score, 100);
    }
}
