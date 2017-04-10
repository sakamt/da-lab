#!/bin/bash -eu

setting=$1
da=$2
typ=$3
count=$4
nproc=${5:-$(nproc --all)}

echo "nproc=$nproc"
cargo run --release --bin=l63_init $setting
cargo run --release --bin=l63_truth $setting init.msg
output="${DATA_DIR}/${typ}_mean/${da}/$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg truth.msg $output
parallel -j $nproc "echo Start: {}/${count};\
  cargo run --release --bin=l63_obs -- $setting truth.msg --output=obs{}.msg > /dev/null 2>&1 && \
  cargo run --release --bin=l63_${typ} -- $da $setting truth.msg obs{}.msg > $output/{}.csv 2> /dev/null \
  " ::: $(seq 1 $count)
mv obs*.msg $output/
