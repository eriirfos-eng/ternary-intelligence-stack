import re
import sys

def convert(match):
    px_val = float(match.group(1))
    rem_val = px_val / 13.0
    return f"font-size: {rem_val:.4f}rem"

with open(sys.argv[1], 'r') as f:
    content = f.read()

# Pattern for font-size: Xpx but NOT for the root html selector
# We'll skip the "html { font-size: 13px; }" line which we set manually
lines = content.split('\n')
new_lines = []
for line in lines:
    if 'html {' in line and 'font-size:' in line:
        new_lines.append(line)
    else:
        new_lines.append(re.sub(r"font-size:\s*([0-9.]+)px", convert, line))

with open(sys.argv[1], 'w') as f:
    f.write('\n'.join(new_lines))
