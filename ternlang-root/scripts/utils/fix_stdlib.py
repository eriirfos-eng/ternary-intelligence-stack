import re

file_path = 'ternlang-api/src/main.rs'
with open(file_path, 'r') as f:
    content = f.read()

# Fix stdlib_read to allow tier 0 (unauthenticated) for base tier files
new_content = re.sub(
    r'let user_tier = if let Some\(entry\) = state\.keys\.peek\(raw\)\.await \{ entry\.tier \} else \{ 0 \};',
    r'let user_tier = if let Some(entry) = state.keys.peek(raw).await { entry.tier } else { 1 };',
    content
)

with open(file_path, 'w') as f:
    f.write(new_content)
