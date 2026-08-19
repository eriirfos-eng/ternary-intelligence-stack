// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// Commercial tier. See LICENSE-COMMERCIAL in the repository root.
// Unauthorized use, copying, or distribution is prohibited.
// Reference Patent Pending A50296/2026.

use ternlang_core::Trit;

/// Raw trit allocator mapping ternary states to physical byte offsets.
/// 
/// Mapping Logic:
/// - State -1 (Reject): Offset 0x00
/// - State  0 (Tend):   Offset 0x01
/// - State +1 (Affirm): Offset 0x02
pub struct TritAllocator {
    memory_base: *mut u8,
    size_trits: usize,
}

impl TritAllocator {
    /// Initialise a raw memory block for trit allocation.
    pub fn new(size_trits: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align(size_trits, 1).unwrap();
        // SAFETY: `layout` has non-zero size (`size_trits` is caller-provided) and valid
        // alignment (1). The returned pointer is used exclusively by this struct.
        let ptr = unsafe { std::alloc::alloc(layout) };
        
        Self {
            memory_base: ptr,
            size_trits,
        }
    }

    /// Maps a trit state to its physical byte representation.
    #[inline(always)]
    fn trit_to_offset(t: Trit) -> u8 {
        match t {
            Trit::Reject => 0x00,
            Trit::Tend   => 0x01,
            Trit::Affirm => 0x02,
        }
    }

    /// Stores a trit in physical memory.
    pub fn store(&mut self, index: usize, val: Trit) {
        if index >= self.size_trits {
            panic!("[ALLOC] OOB Access: Index {} exceeds size {}", index, self.size_trits);
        }
        // SAFETY: Bounds checked above. `memory_base.add(index)` is within the allocation
        // made in `new()` which covers `size_trits` bytes. No aliasing: `&mut self` is exclusive.
        unsafe {
            *self.memory_base.add(index) = Self::trit_to_offset(val);
        }
    }

    /// Loads a trit from physical memory.
    pub fn load(&self, index: usize) -> Trit {
        if index >= self.size_trits {
            panic!("[ALLOC] OOB Access: Index {} exceeds size {}", index, self.size_trits);
        }
        // SAFETY: Bounds checked above. Pointer is valid for the lifetime of `self`.
        let raw = unsafe { *self.memory_base.add(index) };
        match raw {
            0x00 => Trit::Reject,
            0x01 => Trit::Tend,
            0x02 => Trit::Affirm,
            _    => panic!("[ALLOC] Invalid memory state: {}", raw),
        }
    }
}

impl Drop for TritAllocator {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.size_trits, 1).unwrap();
        // SAFETY: `memory_base` was allocated with this exact layout in `new()`. `Drop`
        // is called at most once; no other code deallocates this pointer.
        unsafe {
            std::alloc::dealloc(self.memory_base, layout);
        }
    }
}
