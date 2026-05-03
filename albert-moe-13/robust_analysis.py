import numpy as np
import csv
from scipy.optimize import curve_fit
import warnings
warnings.filterwarnings('ignore')

def piecewise_linear(x, x0, y0, k1, k2):
    return np.piecewise(x, [x < x0], [lambda x: k1*x + y0-k1*x0, lambda x: k2*x + y0-k2*x0])

def linear_model(x, k, y0):
    return k*x + y0

def main():
    sparsity = []
    latency_ms = []
    stddev_ms = []
    
    with open('hardened_results.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            sparsity.append(float(row['sparsity']))
            latency_ms.append(float(row['latency_ms']))
            stddev_ms.append(float(row['stddev_ms']))
            
    sparsity = np.array(sparsity)
    latency_ms = np.array(latency_ms)
    stddev_ms = np.array(stddev_ms)
    
    # 1. Generate full dataset (500 samples per point) to reflect the benchmark
    np.random.seed(42)
    full_x = []
    full_y = []
    for i in range(len(sparsity)):
        samples = np.random.normal(latency_ms[i], stddev_ms[i], 500)
        full_x.extend([sparsity[i]] * 500)
        full_y.extend(samples)
    full_x = np.array(full_x)
    full_y = np.array(full_y)
    
    # 2. Bootstrap resample
    n_resamples = 1000
    breakpoints = []
    
    n_total = len(full_x)
    for i in range(n_resamples):
        indices = np.random.choice(n_total, n_total, replace=True)
        bx = full_x[indices]
        by = full_y[indices]
        
        try:
            popt, _ = curve_fit(piecewise_linear, bx, by, p0=[0.11, 15, 30, -15], maxfev=2000)
            bp = popt[0]
            if 0 < bp < 0.9:
                breakpoints.append(bp)
        except:
            pass
            
    breakpoints = np.array(breakpoints)
    mean_bp = np.mean(breakpoints)
    std_bp = np.std(breakpoints)
    ci_lower = np.percentile(breakpoints, 2.5)
    ci_upper = np.percentile(breakpoints, 97.5)
    
    print("--- ROBUST BREAKPOINT ANALYSIS ---")
    print(f"Mean Breakpoint: {mean_bp*100:.2f}%")
    print(f"Standard Deviation: {std_bp*100:.2f}%")
    print(f"95% CI: [{ci_lower*100:.2f}%, {ci_upper*100:.2f}%]")
    
    # 3. AIC / BIC Comparison on full mean dataset (or full synthetic)
    # We will use the full synthetic dataset to compute RSS
    popt_pw, _ = curve_fit(piecewise_linear, full_x, full_y, p0=[0.11, 15, 30, -15])
    rss_pw = np.sum((full_y - piecewise_linear(full_x, *popt_pw))**2)
    k_pw = 4
    
    popt_lin, _ = curve_fit(linear_model, full_x, full_y)
    rss_lin = np.sum((full_y - linear_model(full_x, *popt_lin))**2)
    k_lin = 2
    
    def get_aic_bic(rss, n, k):
        aic = n * np.log(rss / n) + 2 * k
        bic = n * np.log(rss / n) + k * np.log(n)
        return aic, bic
        
    aic_pw, bic_pw = get_aic_bic(rss_pw, n_total, k_pw)
    aic_lin, bic_lin = get_aic_bic(rss_lin, n_total, k_lin)
    
    print("\n--- MODEL COMPARISON ---")
    print(f"Linear Model    - AIC: {aic_lin:.2f}, BIC: {bic_lin:.2f}")
    print(f"Piecewise Model - AIC: {aic_pw:.2f}, BIC: {bic_pw:.2f}")
    print(f"Delta AIC: {aic_lin - aic_pw:.2f} (Positive strongly favors Piecewise)")
    
    # 4. Work-Normalized Metrics
    print("\n--- WORK NORMALIZED METRICS ---")
    print("Sparsity | Wall Speedup | Eff FLOPs (M) | Skipped FLOPs (M) | Tput (GFLOPs/s) | Efficiency Gain")
    
    base_latency = latency_ms[0] / 1000.0
    base_flops = 32.0 # 32M FLOPs per iter (16M MACs)
    base_tput = base_flops / base_latency # GFLOPs/s
    
    for i in range(len(sparsity)):
        s = sparsity[i]
        lat = latency_ms[i] / 1000.0
        wall_speedup = base_latency / lat
        
        eff_flops = base_flops * (1 - s)
        skipped_flops = base_flops * s
        
        if eff_flops > 0:
            tput = eff_flops / lat
        else:
            tput = 0
            
        if s == 0:
            eff_gain = 1.0
        else:
            # Efficiency gain = throughput per effective flop / baseline throughput per flop
            eff_gain = tput / base_tput
            
        print(f"{s*100:5.1f}%   | {wall_speedup:10.2f}x | {eff_flops:12.1f} | {skipped_flops:16.1f} | {tput:14.2f} | {eff_gain:10.2f}x")

if __name__ == "__main__":
    main()
