#!/bin/bash -eu

setting=$1
da=$2
count=$3

cargo run --release --bin=l63_init $setting
cargo run --release --bin=l63_truth $setting init.msg
output="bias_${da}_$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg truth.msg $output
for i in $(seq 1 $count); do
  cargo run --release --bin=l63_obs $setting truth.msg > /dev/null 2>&1
  cargo run --release --bin=l63_${da}_bias $setting truth.msg obs.msg > $output/$i.csv 2>/dev/null
  echo "$i/$count"
done

