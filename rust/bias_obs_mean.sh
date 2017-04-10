#!/bin/bash -eu

setting=$1
da=$2
count=$3
nproc=${4:-$(nproc --all)}
echo "nproc=$nproc"

cargo run --release --bin=l63_init $setting
cargo run --release --bin=l63_truth $setting init.msg
output="bias_obs_mean_${da}_$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg truth.msg $output
parallel -j $nproc "echo {}/${count};\
  cargo run --release --bin=l63_obs -- $setting truth.msg --output=obs{}.msg > /dev/null && \
  cargo run --release --bin=l63_bias -- $da $setting truth.msg obs{}.msg > $output/{}.csv 2>/dev/null \
  " ::: $(seq 1 $count)
mv obs*.msg $output/
