import re

with open("crates/albert-cli/src/main.rs", "r") as f:
    lines = f.readlines()

with open("crates/albert-cli/src/main.rs", "w") as f:
    skip = False
    for i, line in enumerate(lines):
        if "struct Guard(Option<Arc<AtomicBool>>);" in line:
            # If we see Guard twice, skip the second one
            pass
            
