#!/bin/bash
TC=$1
CASE=$2
BOUND=$2
echo "eval $1"

echo ""

echo "Got"
for input in {1..3}; do
    ./target/debug/CFGDeriver --bound-type $BOUND --cfg-file ./TC4/evals/L${1}Gb.txt --string-file ./TC4/evals/eval$1_$CASE.txt
done
