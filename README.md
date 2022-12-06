# Advent of Code: 2022

This repo holds my work on Advent of Code in December 2022. My participation in AoC this year is mostly a pursuit in continuing to learn Rust.

Below are not really writeups, they're more notes to self, musings, complaints, etc.

## Day 1: Calorie Counting

### Day 1, Part 1

#### Parsing with Pear

I took Day 1 as an opportunity to play around with [`pear`](https://crates.io/crates/pear), an ominous parsing library that piqued my curiosity when I noticed it in `figment`'s dependency. It's lacking a README and some amount of documentation, but it appears to be used in `rocket` to provide some HTTP-related parsing functionality (and is used in `figment`) as well.

The grammar of the input data is so simple that it would be simpler to just loop over the lines in the file and parse numbers vs empty lines, but I went with trying to write a parser using `pear` for the fun factor.

The parser is not fully correct because it assumes some newlines at the end of the file which don't exist in the input (since newlines are only needed to delimit between calorie lines and blocks of elves' calories, the extra whitespace is not included at the end of the file). I initially started looking at fixing that, but decided to just tweak the input and move on.

#### Debug Optimizations and --release

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

### Day 1, Part 2

Part 2 was frustrating because I had the correct answer from the beginning, but the value I submitted was reported as incorrect.

After writing more tests and trying to think through where I was going wrong (to no avail), I tried submitting the answer a third time, and it was correct.

I have no idea what's up there-- I wonder if some browser autofill or extensions could be fucking with my forms...

I took a peek at the HTML and I see that Bitwarden is instrumenting forms to some degree, even when locked:

```html
<form method="post" action="1/answer">
  <input type="hidden" name="level" value="2">
  <p>
    Answer:
    <input type="text" name="answer" autocomplete="off" data-com.bitwarden.browser.user-edited="yes">
    <input type="submit" value="[Submit]">
  </p>
</form>
```

I'm not prepared to assign any blame at the moment, just an observation for now... For all I know I typo'd the answer multiple times.

## Day 2: Rock Paper Scissors

### Day 2, Part 1

It was straightforward to implement the Rock Paper Scissors game logic (yay enum variants). I again implemented a parser using `pear`, and this time around I'm pretty happy with it.

### Day 2, Part 2

Part 2 changed the interpretation of the second column of the strategy guide to refer to the desired outcome instead of the move to choose.

Achieve this was just a matter of extending the parsing functionality to interpret the strategy guide both ways and adding a few methods around the game logic.

I also threw in a `PartialOrd` implementation for `Move` because the meaning of the comparison traits often escapes me; it was good for learning, but didn't add much value I think.

## Day 3: Rucksack Reorganization

### Day 3, Part 1

The input format for this challenge was so simple that I opted to just read in the lines instead of writing a parser (like in previous challenges), but this came with the drawback that I was not motivated to do any validation on that. Part of me feels that kind of validation isn't really necessary for quick puzzles, but part of me likes having it.

### Day 3, Part 2

TODO
