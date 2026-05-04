#include <stdio.h>
#include <stdint.h>

/* Ternary Primitive Definitions */
typedef int8_t trit;
#define TRIT_REJECT -1
#define TRIT_HOLD    0
#define TRIT_AFFIRM  1

trit consensus(trit a, trit b) {
    if (a == TRIT_REJECT || b == TRIT_REJECT) return TRIT_REJECT;
    if (a == TRIT_HOLD   || b == TRIT_HOLD)   return TRIT_HOLD;
    return TRIT_AFFIRM;
}

trit check_system(trit status, trit threshold) {
    trit vote = consensus(status, threshold);
    switch (vote) {
        case -1: return TRIT_REJECT;
        case  0: return TRIT_HOLD;
        case  1: return TRIT_AFFIRM;
        default: return TRIT_HOLD;
    }
}

trit launch_poll(trit propulsion, trit avionics, trit weather, trit range_safety) {
    switch (range_safety) {
        case -1: return TRIT_REJECT;
        case  0: return TRIT_HOLD;
        case  1: {
            trit systems_12 = consensus(propulsion, avionics);
            trit systems_34 = consensus(weather, range_safety);
            trit all_systems = consensus(systems_12, systems_34);
            return all_systems;
        }
        default: return TRIT_HOLD;
    }
}

int main() {
    trit propulsion = TRIT_AFFIRM;
    trit avionics = TRIT_AFFIRM;
    trit weather = TRIT_HOLD;
    trit range_safety = TRIT_AFFIRM;

    trit launch_decision = launch_poll(propulsion, avionics, weather, range_safety);

    printf("Launch Decision Result: %d\n", launch_decision);
    return 0;
}
