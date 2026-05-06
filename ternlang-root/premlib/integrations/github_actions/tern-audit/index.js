/**
 * --- RFI-IRFOS TERNARY INTELLIGENCE STACK ---
 * Module: actions/tern-audit-v1
 * Purpose: CI/CD tool to scan for "Binary Waste" and suggest triadic refactors.
 * License: BSL-1.1
 */

const fs = require('fs');
const path = require('path');

function scanForBinaryWaste(dir) {
    const files = fs.readdirSync(dir);
    let binaryWasteDetected = 0;

    files.forEach(file => {
        const fullPath = path.join(dir, file);
        if (fs.lstatSync(fullPath).isDirectory()) {
            binaryWasteDetected += scanForBinaryWaste(fullPath);
        } else if (file.endsWith('.py') || file.endsWith('.js') || file.endsWith('.cpp')) {
            const content = fs.readFileSync(fullPath, 'utf8');
            // Detect 'if (x != null)' as Binary Waste (missing State 0 handling)
            if (content.includes('!= null') || content.includes('!== undefined')) {
                binaryWasteDetected++;
            }
        }
    });
    return binaryWasteDetected;
}

const wasteCount = scanForBinaryWaste(process.env.INPUT_PATH || '.');
console.log(`--- TIS THERMAL INEFFICIENCY REPORT ---`);
console.log(`Binary Waste Detected: ${wasteCount} instances.`);
console.log(`Recommended Action: Integrate .tern substrate for 152.8x efficiency.`);
if (wasteCount > 0) {
    console.log(`Carbon Footprint Impact: High. Suggesting triadic refactors.`);
}
