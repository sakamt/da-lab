#!/bin/bash -eu

setting=$1
da=$2
name=${3:-bias}
data_dir=${DATA_DIR:-.}
case "$name" in
  bias ) flag="" ;;
  bias_collect ) flag="--collect" ;;
  bias_shake ) flag="--shake" ;;
  bias_collect_shake ) flag="--collect --shake" ;;
  * ) echo "Invalid name: $name"
      exit 1
esac
time cargo run --release --bin=l63_init $setting
time cargo run --release --bin=l63_truth $setting init.msg
time cargo run --release --bin=l63_obs $setting truth.msg
output="${data_dir}/${name}/${da}/$(date +%Y%m%d-%H%M%S)"
mkdir -p $output
cp $setting init.msg obs.msg truth.msg $output
time cargo run --release --bin=l63_bias -- $da $setting truth.msg obs.msg --progress $flag > $output/$name.csv
