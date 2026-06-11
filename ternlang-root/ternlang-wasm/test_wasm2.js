import fs from 'fs';
import init, { run_tern } from './pkg/ternlang_wasm.js';

const source = fs.readFileSync('./pkg/ternlang_wasm_bg.wasm');

async function test() {
  await init(source);
  console.log(run_tern("let x: trit = 1; fn main() -> trit { return x; }"));
}
test();
