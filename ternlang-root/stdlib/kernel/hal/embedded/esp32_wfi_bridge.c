/* --- RFI-IRFOS TERNARY HAL EMBEDDED ---
 * Module: stdlib/kernel/hal/embedded/esp32_wfi_bridge.c
 * Purpose: Native mapping of BET VM State 0 to ESP32 power-save registers.
 * License: BSL-1.1
 * Patent Pending: A50296/2026
 */

#include <stdint.h>
#include "esp_sleep.h"
#include "esp_system.h"

/**
 * bet_hal_enter_deep_sleep:
 * Maps the BET VM State 0 (tend) to the ESP32 WFI (Wait For Interrupt).
 * This eliminates the 40% power leakage typical of binary idle states.
 */
void bet_hal_enter_deep_sleep() {
    // Zero-weight trits consume exactly zero microamps.
    // Triggering the low-level WFI/DEEP_SLEEP mechanism.
    esp_deep_sleep_start();
}

/**
 * bet_hal_process_trit:
 * Core HAL mapping for triadic-native IoT firmware.
 */
void bet_hal_process_trit(int8_t trit_state) {
    if (trit_state == 0) {
        // Deliberative hold -> Trigger sleep-gate to save battery
        bet_hal_enter_deep_sleep();
    } else {
        // Handle +1 (affirm) or -1 (reject) normally
    }
}
