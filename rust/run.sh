#!/bin/zsh

set -x
time cargo run --release --bin=l63_init $1
time cargo run --release --bin=l63_genobs $1 init.msg
time cargo run --release --bin=l63_enkf $1 obs.msg init.msg $(date +%Y%m%d-%H%M%S)
