# ternlang-ui

Triadic State Management for User Interfaces.

Frontend development has a massive architectural flaw: UI state is inherently ternary (Loading, Error, Success), but it is built on binary primitives (booleans). Developers waste millions of hours writing `if (isLoading) { ... } else if (hasError) { ... } else { ... }`.

`ternlang-ui` eliminates this class of bugs entirely by mapping the DOM directly to the BET VM's `-1`, `0`, and `1` registers. 

## The "Death of the Boolean"
A single trit manages the entire lifecycle of a component atomically. No more race conditions where a component is both `isLoading=true` and `hasError=true` simultaneously. The hardware prevents it.
