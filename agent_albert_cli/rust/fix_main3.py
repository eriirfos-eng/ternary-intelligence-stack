import os
import re

path = "crates/rusty-ternlang-cli/src/main.rs"
with open(path, "r") as f:
    content = f.read()

# Undo the global _config rename
content = content.replace("let _config = ConfigLoader::default_for(&cwd).load()?;", "let config = ConfigLoader::default_for(&cwd).load()?;")

# Apply unused config only to build_runtime
build_runtime_start = content.find("fn build_runtime")
if build_runtime_start != -1:
    config_load_idx = content.find("let config = ConfigLoader::default_for(&cwd).load()?;", build_runtime_start)
    if config_load_idx != -1:
        content = content[:config_load_idx] + "let _config = ConfigLoader::default_for(&cwd).load()?;" + content[config_load_idx + len("let config = ConfigLoader::default_for(&cwd).load()?;"):]

# Fix payload.message.usage across newlines
content = re.sub(r"map_usage\(\s*payload\.message\.usage,\s*\)", "map_usage(payload.message.usage.clone())", content)
content = re.sub(r"map_usage\(\s*payload\.message\.usage\s*\)", "map_usage(payload.message.usage.clone())", content)

with open(path, "w") as f:
    f.write(content)
