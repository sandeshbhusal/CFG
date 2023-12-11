#!/usr/bin/python3

import sys
from subprocess import STDOUT, check_output

if len(sys.argv) < 2:
    print("usage: python3 bench.py <number of eval suite>")
    sys.exit(0)

langnum = sys.argv[1]

languages = [
    f"L{langnum}Gb",
    f"L{langnum}cnf"
]

NUM_EVALS = 3

print("Starting benchmarks...")
print("-" * 40)
print("Lang\tevalcase\tbound\toutput")
print("-" * 40)
language_pair = languages

for e in range(1, 4):
    stringfile = f"./TC4/evals/eval{langnum}_{e}.txt"
    # Do the actual run with timeout.
    for bound in range(1, 4):
        for language in language_pair:
            if "Gb" in language and bound == 3:
                continue
            if "cnf" in language and bound < 3:
                continue
            
            cmd = [
                "./target/release/CFGDeriver",
                str(bound),
                f"./TC4/evals/{language}.txt",
                str(stringfile)
            ]
            # print(' '.join(cmd))
            try:
                output = check_output(cmd, stderr=STDOUT, timeout = 30)
                print(language, "\t", f"eval{langnum}_{e}", "\t", bound, "\t", output.decode("utf-8").strip())
            except:
                print(language, "\t", f"eval{langnum}_{e}", "\t", bound, "\t", "timeout")