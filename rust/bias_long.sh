#!/bin/bash -eu

setting=$1
da=$2

cargo run --release --bin=l63_init $setting
cargo run --release --bin=l63_truth $setting init.msg
cargo run --release --bin=l63_obs $setting truth.msg
output="bias_long_${da}_$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg obs.msg truth.msg $output
cargo run --release --bin=l63_${da}_bias $setting truth.msg obs.msg > $output/bias.csv
