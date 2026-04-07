# ternlang-astro

Interplanetary Delay-Tolerant Networking (DTN) for the BET VM.

TCP/IP was designed for Earth. It relies on low-latency acknowledgments and immediate timeouts. When building a data link between Earth, Mars, and the orbital mining colonies, binary protocols collapse. You cannot drop a connection just because the light-speed delay is 14 minutes.

## The State 0 Vacuum
`ternlang-astro` solves the deep-space routing problem natively. When a packet is beamed across the solar system, the transmitting node does not enter a "Failure/Timeout" state. It transitions to `State 0 (InTransit)`. 

The thread yields on the hardware level, consuming zero power until the returning light wave breaks the equilibrium. **Ternlang is the protocol of the interplanetary economy.**
