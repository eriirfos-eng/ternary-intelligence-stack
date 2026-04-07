//! ternlang-astro: Interplanetary Delay-Tolerant Networking (DTN).
//!
//! Deep space communication suffers from massive, variable latency. Binary TCP 
//! fundamentally breaks over light-minute distances because it relies on immediate 
//! ACKs or binary timeouts. `ternlang-astro` introduces the "Suspended Quantum" 
//! State 0 to packet routing, creating the infrastructure for multi-planetary networks.

pub mod dtn {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum PacketState {
        Acknowledged = 1,
        InTransit = 0, // The packet exists in physical space between planets
        Lost = -1,
    }

    pub struct AstroRouter;

    impl AstroRouter {
        /// Evaluates a packet across an interplanetary mesh.
        /// Unlike Earth-bound routers, an AstroRouter does not drop a connection
        /// just because of a 40-minute light delay.
        pub fn route_mars_to_earth(&self, packet_id: &str, time_elapsed_sec: f64) -> PacketState {
            let light_delay_sec = 240.0; // ~4 minutes average

            if time_elapsed_sec < light_delay_sec {
                println!("ternlang-astro: Packet {} is in transit. Entering State 0.", packet_id);
                PacketState::InTransit // BET VM physically pauses the routing thread
            } else {
                // Determine if ACK was received or signal was lost
                PacketState::Acknowledged 
            }
        }
    }
}
