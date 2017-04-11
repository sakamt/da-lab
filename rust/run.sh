#!/bin/bash -eu

setting=$1
da=$2
data_dir=${DATA_DIR:-.}

time cargo run --release --bin=l63_init $setting
time cargo run --release --bin=l63_truth $setting init.msg
time cargo run --release --bin=l63_obs $setting truth.msg
output="${data_dir}/${da}/$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
time cargo run --release --bin=l63_run -- $da $setting obs.msg init.msg $output --progress
time cargo run --release --bin=l63_rmse $setting truth.msg $output
cp $setting init.msg obs.msg truth.msg rmse.msg $output
