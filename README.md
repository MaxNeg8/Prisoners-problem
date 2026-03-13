# Introduction
This is a small and simple program written in Rust which approximates the solution to the [100 prisoners problem](https://en.wikipedia.org/wiki/100_prisoners_problem) numerically.
An analytical solution to the problem exists, which yields

$$p_{\mathrm{success}}(2n) = 1 - (H_{2n} - H_n)$$

for $2n$ prisoners, where

$$H_n = \sum_{k = 1}^n \frac{1}{k}$$

is the $n$-th Harmonic number. Therefore, this program is meant to be more of a coding exercise rather than a useful tool.

# Building
If you are on Linux x86_64, you may download and run the binary included in the [latest release](https://github.com/MaxNeg8/Prisoners-problem/releases/latest). Otherwise, make sure you [have installed Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and run the following commands:

```bash
$ git clone https://github.com/MaxNeg8/Prisoners-problem prisoners
$ cd prisoners
$ cargo build --release
```

The, run the program using

```bash
$ ./target/release/prisoners
```

# Usage
Please refer to the output of `prisoners --help` for usage instructions. The output of the program will be $p_{\mathrm{success}}(2n)$ as defined above, where $2n$ is the `--prisoners` argument.
