# Advent of Code: 2022

This repo holds my work on Advent of Code in December 2022. My participation in AoC this year is mostly a pursuit in continuing to learn Rust.

## Day 1: Calorie Counting

### Parsing with Pear

I took Day 1 as an opportunity to play around with [`pear`](https://crates.io/crates/pear), an ominous parsing library that piqued my curiosity when I noticed it in `figment`'s dependency. It's lacking a README and some amount of documentation, but it appears to be used in `rocket` to provide some HTTP-related parsing functionality (and is used in `figment`) as well.

The grammar of the input data is so simple that it would be simpler to just loop over the lines in the file and parse numbers vs empty lines, but I went with trying to write a parser using `pear` for the fun factor.

The parser is not fully correct because it assumes some newlines at the end of the file which don't exist in the input (since newlines are only needed to delimit between calorie lines and blocks of elves' calories, the extra whitespace is not included at the end of the file). I initially started looking at fixing that, but decided to just tweak the input and move on.

### Debug Optimizations and --release

Solving Day 1 increased my appreciation for the performance distinctions of building with debug optimizations vs for `--release`. I would not be surprised if my code is just sloppy & non-performant, but it runs markedly slowly without the `--release` flag.

With debug optimizations the program runs in 5.36 seconds.

Built in release mode, the program runs in 0.04 seconds.

```
big-donut in advent-of-code on  main [?] is 📦 v0.1.0 via 🦀 v1.65.0 
➜ cargo test
   Compiling day_1_calorie_counting v0.1.0 (/home/travis/projects/advent-of-code)
    Finished test [unoptimized + debuginfo] target(s) in 0.34s
     Running unittests src/main.rs (target/debug/deps/day_1_calorie_counting-d8e7cea1497a1c83)

running 1 test
test test::solve_day_1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.36s


big-donut in advent-of-code on  main [?] is 📦 v0.1.0 via 🦀 v1.65.0 took 5s 
➜ cargo test --release
    Finished release [optimized] target(s) in 0.00s
     Running unittests src/main.rs (target/release/deps/day_1_calorie_counting-0353544f908997b4)

running 1 test
test test::solve_day_1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

big-donut in advent-of-code on  main [?] is 📦 v0.1.0 via 🦀 v1.65.0 
```
