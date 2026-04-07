//! ternlang-swarm: Triadic Kinematics for Autonomous Robotics.
//!
//! Binary robots operate on true/false hazard detection: "Path is clear" (1) or 
//! "Obstacle detected" (0). This results in erratic, jerky movements when faced 
//! with dynamic environments. `ternlang-swarm` gives physical machines the ability 
//! to "hesitate" (State 0) and collaboratively negotiate space.

pub mod kinematics {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(i8)]
    pub enum KinematicIntent {
        Advance = 1,
        Hesitate = 0, // State 0: Slow down, read environment, maintain momentum
        Reverse = -1, // Evade
    }

    pub struct SwarmDrone {
        pub id: usize,
        pub proximity_sensor: f32,
    }

    impl SwarmDrone {
        pub fn new(id: usize) -> Self {
            SwarmDrone { id, proximity_sensor: 10.0 }
        }

        /// Triadic Collision Avoidance.
        /// Replaces binary "STOP/GO" with biological hesitation.
        pub fn compute_vector(&self, human_distance: f32) -> KinematicIntent {
            if human_distance > 5.0 {
                KinematicIntent::Advance
            } else if human_distance < 1.0 {
                KinematicIntent::Reverse
            } else {
                // The drone enters a state of physical hesitation.
                // It does not freeze (binary 0); it yields authority to the swarm MoE.
                println!("ternlang-swarm: Drone {} entering State 0 (Hesitation).", self.id);
                KinematicIntent::Hesitate
            }
        }
    }
}
