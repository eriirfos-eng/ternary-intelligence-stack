import subprocess
import csv
import re
import os

base_dir = os.path.dirname(os.path.abspath(__file__))

# Find binary
bin_path = "./target/release/hardened_bench"
if not os.path.exists(bin_path):
    bin_path = "../target/release/hardened_bench"

points = [0.0, 0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.75, 0.9]
results = []

# Events to collect
events = [
    "instructions", "cycles",
    "branches", "branch-misses",
    "L1-dcache-load-misses", "L1-dcache-loads",
    "LLC-load-misses", "LLC-loads",
    "stalled-cycles-frontend", "stalled-cycles-backend"
]

print(f"Sparsity | IPC | Branch Miss % | L1 Miss % | LLC Miss % | Frontend Stall % | Backend Stall %")
print("-" * 100)

for p in points:
    cmd = [
        "perf", "stat", "-x", ",", "-e", ",".join(events),
        bin_path, str(p)
    ]
    
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    stdout, stderr = proc.communicate()
    
    # Parse perf output from stderr (perf stat -x , outputs to stderr by default or if redirected)
    # Actually -x , often goes to stderr.
    perf_data = {}
    for line in stderr.splitlines():
        parts = line.split(",")
        if len(parts) >= 3:
            val = parts[0]
            event = parts[2]
            if val and val != "<not counted>":
                perf_data[event] = float(val)

    if not perf_data:
        # Try stdout if stderr is empty
        for line in stdout.splitlines():
             parts = line.split(",")
             if len(parts) >= 3:
                val = parts[0]
                event = parts[2]
                if val and val != "<not counted>":
                    perf_data[event] = float(val)

    # Calculate metrics
    try:
        ins = perf_data.get("instructions", 0)
        cyc = perf_data.get("cycles", 1)
        br = perf_data.get("branches", 1)
        br_m = perf_data.get("branch-misses", 0)
        l1_m = perf_data.get("L1-dcache-load-misses", 0)
        l1_l = perf_data.get("L1-dcache-loads", 1)
        llc_m = perf_data.get("LLC-load-misses", 0)
        llc_l = perf_data.get("LLC-loads", 1)
        f_stall = perf_data.get("stalled-cycles-frontend", 0)
        b_stall = perf_data.get("stalled-cycles-backend", 0)

        ipc = ins / cyc
        br_rate = (br_m / br) * 100
        l1_rate = (l1_m / l1_l) * 100
        llc_rate = (llc_m / llc_l) * 100
        f_rate = (f_stall / cyc) * 100
        b_rate = (b_stall / cyc) * 100

        print(f"{p:8.2f} | {ipc:3.2f} | {br_rate:12.1f}% | {l1_rate:8.1f}% | {llc_rate:9.1f}% | {f_rate:15.1f}% | {b_rate:14.1f}%")
        
        results.append({
            "sparsity": p,
            "ipc": ipc,
            "branch_miss_pct": br_rate,
            "l1_miss_pct": l1_rate,
            "llc_miss_pct": llc_rate,
            "frontend_stall_pct": f_rate,
            "backend_stall_pct": b_rate,
            "instructions": ins,
            "cycles": cyc
        })
    except Exception as e:
        print(f"Error parsing point {p}: {e}")

# Save to CSV
output_file = os.path.join(base_dir, "perf_counters.csv")
with open(output_file, "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=results[0].keys())
    writer.writeheader()
    writer.writerows(results)
