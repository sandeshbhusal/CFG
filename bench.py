from subprocess import STDOUT, check_output

languages = [
    ["L1Gb",
    "L1cnf"],
    ["L2Gb",
    "L2cnf"],
    ["L3Gb",
    "L3cnf"]
]

NUM_EVALS = 3

print("Starting benchmarks...")
print("-" * 40)
print("Lang\tevalcase\tbound\toutput")
print("-" * 40)
for i in range(1, 4):
    language_pair = languages[i-1]
    for e in range(1, 4):
        stringfile = f"./TC4/evals/eval{i}_{e}.txt"
        # Do the actual run with timeout.
        for bound in range(1, 4):
            for language in language_pair:
                if "Gb" in language and bound == 3:
                    continue
                if "cnf" in language and bound < 3:
                    continue
                
                cmd = [
                    "./target/release/CFGDeriver",
                    "--bound-type",
                    str(bound),
                    "--cfg-file",
                    f"./TC4/evals/{language}.txt",
                    "--string-file",
                    str(stringfile)
                ]
                # print(' '.join(cmd))
                try:
                    output = check_output(cmd, stderr=STDOUT, timeout = 30)
                    print(language, "\t", f"eval{i}_{e}", "\t", bound, "\t", output.decode("utf-8").strip())
                except:
                    print(language, "\t", f"eval{i}_{e}", "\t", bound, "\t", "timeout")