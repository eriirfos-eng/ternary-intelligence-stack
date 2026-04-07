//! ternlang-cad: Triadic Topology Optimization (T-CAD).
//!
//! Replaces binary Solid/Void voxel geometry with Triadic Metamaterials.
//! State 0 represents procedurally resolved internal lattice structures.

pub mod geometry {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum VoxelState {
        Solid = 1, // 100% density
        Void = -1, // 0% density
        Lattice = 0, // Metamaterial (State 0) - Computed at slicing time
    }

    /// Calculates the effective mass of a structural volume.
    /// The Lattice state allows the BET-VM to dynamically infer density.
    pub fn calculate_mass(voxels: &[VoxelState], lattice_density: f32) -> f32 {
        voxels.iter().map(|v| match v {
            VoxelState::Solid => 1.0,
            VoxelState::Void => 0.0,
            VoxelState::Lattice => lattice_density, // Dynamically resolved
        }).sum()
    }
}
