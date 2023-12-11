# CFGDeriver - check if a string can be derived by a CFG or not 

CFGDeriver is a project done as a requirement for Course CS-561 at Boise State Univeristy (Fall 2023 - Theory of Computation). The program takes in a CFL description in a CFG file and a string file and traces the string file on the CFG using a PDA to see if the  string is in the CFL or not. Implementation of a CFG Parser and a PDA are provided that can be run separately. This project is developed entirely in the Rust programming language. Benchmarks are done using bash and python3.

## Compiling.
The rust version for this project is Rust 1.72. You will need to install Rust 1.72 on your machine (or higher) to run this project. In order to install rust, follow the instructions at [the rustup website](https://rustup.rs/). To check if you have rust already installed on your machine, run `which rust` to verify the installation location. After you have installed rust, run the following command to compile the program:

```bash
cargo build --release
```

This will pull in the required dependencies for the program and compile it. The compiled binary will be `target/release/CFGDeriver`. 

## Running:
Running the compiled binary is pretty straigtforward. Some test cases and evaluation benchmark suite have been provided with this code inside the TC4 folder. In order to run the CFGDeriver, run the following command:

```bash
target/release/CFGDeriver <bound type> <CFG definition file> <string file>
```

Help can be obtained for each argument type by runinng:

```bash
target/release/CFGDeriver --help
```

Environment variable `RUST_LOG` can be used to check the trace of the string. For e.g.

```bash
RUST_LOG=debug ./target/release/CFGDeriver 1 TC4/evals/L2Gb.txt TC4/evals/eval2_1.txt
```

The variable can take in `debug` and `trace` parameters, which show operations on stack and variable expansion as well as machine terminations.

## Testing:
To test a particular testsuite, the `runall.sh` file can be used. For an e.g. to run testsuite 1, you can run:

```bash
./runall.sh 1
```

## Benchmarks:
There is a very trivial benchmark script provided with the code that runs the compiled binary. _Do not run the script without compiling the binary, it will fail._ All benchmarks are done on the release mode. In order to run benchmarks and obtain results, run:

```bash
python3 bench.py <id of benchmark suite>
```

This produces answers for each 3 eval strings for the given eval id.

# Addendum (files):
There are some files inside the project source tree `src`.:
- cfg.rs:
This file contains definition of a CFG.

- pda.rs:
This file contains the conversion from CFG to a PDA and tracing a string. It contains methods for BFS and DFS methods, the DFS search is enabled by default.

- parser.rs:
This file contains the parsing logic.

- main.rs:
This is the entrypoint of the program.

Other files contain the pretty-print instructions.

- Sandesh Bhusal
Dec, 2023