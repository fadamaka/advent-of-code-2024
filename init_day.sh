#!/bin/bash
cp -r src/day## src/day${1}
sed -i "s/day##/day${1}/" src/day${1}/a.rs
sed -i "s/day##/day${1}/" src/day${1}/b.rs
sed -i "s/fn main() {/mod day${1};\nfn main() {/" src/main.rs