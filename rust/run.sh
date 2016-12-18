#!/usr/bin/env zsh
# TODO check args
time cargo run --release --bin=l63_init $1
time cargo run --release --bin=l63_genobs $1 init.msg
output=$(date +%Y%m%d-%H%M%S)
time cargo run --release --bin=l63_enkf $1 obs.msg init.msg $output
cp $1 init.msg obs.msg truth.msg $output
