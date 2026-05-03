#!/bin/bash
for i in {1..100}; do
  cargo run --bin run_experiment -- $i $i
done
