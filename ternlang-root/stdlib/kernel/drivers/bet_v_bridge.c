/* --- RFI-IRFOS T-DRIVER ALPHA ---
 * Module: stdlib/kernel/drivers/bet_v_bridge.c
 * Purpose: High-performance software-to-silicon interrupt bridge for BET VM.
 * License: BSL-1.1
 * Patent Pending: A50296/2026
 */

#include <stdint.h>
#include <stdbool.h>

/* Simulated Hardware Register Addresses */
#define BET_ISA_INTERRUPT_REG  0x4000
#define TRIT_HOLD_SIGNAL       0x00    /* Map State 0 (tend) to Trit-Hold */
#define TRIT_AFFIRM_SIGNAL     0x01    /* Map State +1 (affirm) to Truth-Gate */
#define TRIT_REJECT_SIGNAL     0xFF    /* Map State -1 (reject) to Conflict-Gate */

/* 
 * bet_signal_trit_hold:
 * Dispatches a zero-latency interrupt to a simulated CPU register.
 * Avoids context switch overhead by using a direct memory-mapped I/O (MMIO) signal.
 */
void bet_signal_trit_hold(volatile uint8_t *mmio_reg) {
    // Map State 0 (tend) directly to the TSKIP primitive on BET-compliant hardware.
    *mmio_reg = TRIT_HOLD_SIGNAL;
}

/* 
 * bet_dispatch_signal:
 * Dispatches a triadic signal based on the internal VM state.
 */
int bet_dispatch_signal(int8_t trit_state) {
    volatile uint8_t *reg = (uint8_t *)BET_ISA_INTERRUPT_REG;
    
    switch (trit_state) {
        case 1:  *reg = TRIT_AFFIRM_SIGNAL; return 0;
        case -1: *reg = TRIT_REJECT_SIGNAL; return 0;
        case 0:  bet_signal_trit_hold(reg); return 0; // Hardware-level skip triggered
        default: return -1;
    }
}
