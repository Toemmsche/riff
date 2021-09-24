# riff
Implementation of the GNU diff utility in Rust, mainly for practice purposes.

## Usage

`cargo run file_A.txt file_B.txt`

The output will prefix lines that should be deleted (inserted) with `-` (`+`). The diff uses an LCS algorithm that uses O(n²) time and space.