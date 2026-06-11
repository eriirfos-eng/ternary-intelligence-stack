import fs from 'fs';
import init, { run_tern } from './pkg/ternlang_wasm.js';

const source = fs.readFileSync('./pkg/ternlang_wasm_bg.wasm');

async function test() {
  await init(source);
  console.log(run_tern("fn main() -> trit { return 1; }"));
}
test();
