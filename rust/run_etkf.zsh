#!/usr/bin/env zsh
# TODO check args
time cargo run --release --bin=l63_init $1
time cargo run --release --bin=l63_genobs $1 init.msg
output="ETKF_$(date +%Y%m%d-%H%M%S)"
time cargo run --bin=l63_etkf $1 obs.msg init.msg $output
time cargo run --release --bin=l63_rmse truth.msg $output
cp $1 init.msg obs.msg truth.msg rmse.msg $output
