# btr-diff: a tool for comparing files
This project aims to achieve the same as GNU Diffutils but present the output in a better way ~~(and is written in Rust)~~

## Running
Before you try to run make sure you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system

1. `git clone https://github.com/antste0/btr-diff.git`
2. `cd btr-diff`
3. `cargo b`
4. `cargo r [file1] [file2]` where [file1] and [file2] are files you're trying to compare

The differences from [file1] are in green and from [file2] - in red.

The output should look something like this:
```
--------------------
Comparing [file1] and [file2]:
--------------------
4       diff1
4       diff2

6       diff1
7       diff1
6       diff_2
7       diff_2
--------------------
```
