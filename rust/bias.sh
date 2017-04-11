#!/bin/bash -eu

setting=$1
da=$2
name=${3:-bias}
special=${3:+"--special=$3"}
data_dir=${DATA_DIR:-.}

time cargo run --release --bin=l63_init $setting
time cargo run --release --bin=l63_truth $setting init.msg
time cargo run --release --bin=l63_obs $setting truth.msg
output="${data_dir}/${name}/${da}/$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg obs.msg truth.msg $output
time cargo run --release --bin=l63_bias -- $da $setting truth.msg obs.msg $special --progress > $output/$name.csv
