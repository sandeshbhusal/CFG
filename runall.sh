#!/bin/bash
TC=$1
echo "Testcase $1"

echo "Expected"
cat ./TC4/tests/a$1.txt

echo ""

echo "Got"
for input in {1..5}; do
    ./target/release/CFGDeriver 1 ./TC4/tests/tc$1.txt ./TC4/tests/in$1_$input.txt
done
