#!/bin/sh -eu

setting=$1
da=$2

time cargo run --release --bin=l63_init $setting
time cargo run --release --bin=l63_genobs $setting init.msg
output="${da}_$(date +%Y%m%d-%H%M%S)"
time cargo run --release --bin=l63_$da $setting obs.msg init.msg $output
time cargo run --release --bin=l63_rmse truth.msg $output
cp $setting init.msg obs.msg truth.msg rmse.msg $output
