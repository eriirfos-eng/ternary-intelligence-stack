import fs from 'fs';
import init, { run_tern, check_tern } from './pkg/ternlang_wasm.js';

const source = fs.readFileSync('./pkg/ternlang_wasm_bg.wasm');
const tern_code = fs.readFileSync('../examples/02_decision_gate.tern', 'utf-8');

async function test() {
  await init(source);
  console.log("Check:", check_tern(tern_code));
  console.log("Run:", run_tern(tern_code));
}
test();
