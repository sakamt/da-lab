#!/bin/bash -eu

setting=$1
da=$2
typ=${3:-bias}
data_dir=${DATA_DIR:-.}

time cargo run --release --bin=l63_init $setting
time cargo run --release --bin=l63_truth $setting init.msg
time cargo run --release --bin=l63_obs $setting truth.msg
output="${data_dir}/${typ}/${da}/$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg obs.msg truth.msg $output
time cargo run --release --bin=l63_$typ -- $da $setting truth.msg obs.msg --progress > $output/$typ.csv
