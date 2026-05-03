import numpy as np
import csv
from scipy.optimize import curve_fit

def piecewise_linear(x, x0, y0, k1, k2):
    return np.piecewise(x, [x < x0], [lambda x: k1*x + y0-k1*x0, lambda x: k2*x + y0-k2*x0])

def analyze():
    x = []
    y = []
    cycles = []
    with open('hardened_results.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            x.append(float(row['sparsity']))
            y.append(float(row['latency_ms']))
            cycles.append(float(row['cycles']))
    
    x = np.array(x)
    y = np.array(y)
    cycles = np.array(cycles)
    
    # 1. Linear Fit
    p_lin = np.polyfit(x, y, 1)
    y_lin = np.polyval(p_lin, x)
    res_lin = np.sum((y - y_lin)**2)
    
    # 2. Piecewise Linear Fit (Detecting 1 Breakpoint)
    # Initial guess: x0=0.2, y0=mean(y), k1=0, k2=-10
    try:
        popt, _ = curve_fit(piecewise_linear, x, y, p0=[0.2, 15, 0, -20])
        y_pw = piecewise_linear(x, *popt)
        res_pw = np.sum((y - y_pw)**2)
        breakpoint = popt[0]
    except:
        res_pw = float('inf')
        breakpoint = None

    print(f"Linear Residual: {res_lin:.4f}")
    print(f"Piecewise Residual: {res_pw:.4f}")
    
    if res_pw < res_lin * 0.5:
        print(f"Regime Hypothesis: SUPPORTED. Detected breakpoint at {breakpoint:.2f}")
    else:
        print("Regime Hypothesis: INCONCLUSIVE or NOT SUPPORTED (Linear scaling dominant).")

    # Bottleneck Analysis based on Slope
    # Slope at high sparsity vs low sparsity
    if breakpoint:
        k1 = popt[2]
        k2 = popt[3]
        print(f"Slope 1 (Low Sparsity): {k1:.2f}")
        print(f"Slope 2 (High Sparsity): {k2:.2f}")
    
    # cycles vs latency stability
    freqs = cycles / (y / 1000)
    avg_freq = freqs.mean()
    print(f"Average Frequency (Proxy): {avg_freq/1e9:.2f} GHz")
    print(f"Frequency Variation: {freqs.std()/avg_freq*100:.2f}%")

if __name__ == "__main__":
    analyze()
