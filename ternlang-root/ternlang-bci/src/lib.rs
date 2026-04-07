//! ternlang-bci: Native Brain-Computer Interface (BCI) decoding for the BET VM.
//!
//! Binary BCIs (like Neuralink) reduce complex neural activity to a binary spike (0 or 1).
//! This is biologically inaccurate. Neurons exhibit excitation (+1), resting potential (0), 
//! and active inhibition (-1). `ternlang-bci` decodes EEG/ECoG arrays directly into 
//! native hardware trits, preserving the brain's natural inhibitory pathways.

pub mod neural {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(i8)]
    pub enum NeuralSignal {
        Excitation = 1,
        Resting = 0,
        Inhibition = -1,
    }

    pub struct BCIArray {
        pub channels: usize,
    }

    impl BCIArray {
        pub fn new(channels: usize) -> Self {
            BCIArray { channels }
        }

        /// Decodes a raw voltage delta directly into a hardware trit.
        /// No lossy sigmoid functions. No arbitrary binary thresholds.
        pub fn decode_voltage_delta(&self, voltage_mv: f32, threshold: f32) -> NeuralSignal {
            if voltage_mv > threshold {
                NeuralSignal::Excitation // Action potential
            } else if voltage_mv < -threshold {
                NeuralSignal::Inhibition // Hyperpolarization (Active suppression)
            } else {
                NeuralSignal::Resting // State 0 (TEND) - Baseline
            }
        }
    }
}
