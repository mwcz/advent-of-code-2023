# Advent of Code 2023 🦀

These are my Rust solutions for [Advent of Code 2023](https://adventofcode.com/2023).

[My AoC solutions for other years](https://github.com/mwcz?tab=repositories&q=advent&type=source&language=&sort=name)

## Running solutions

Install [justfile](https://just.systems/man/en/) and run `just --list` to get started.

## Example commands

| Purpose                       | Command                                    |
| ---                           | ---                                        |
| Start day 1                   | `just day 1`                               |
| Run day 3 on file change      | `just watch 3`                             |
| Run day 16 with example input | `just run 16 -e`                           |
| Run day 1 with custom input   | `just run 1 -i examples/custom_input_file` |
| Run day 21 in release mode    | `just r 21`                                |
| Run tests | `just test`                                |
| Run tests for day 13 | `just test 13`                                |

`cargo run` can be used directly, but 

## Start a new day

The command `just day N` (where `N` is the day number, from 1 to 25), will scaffold and open all[^1] the files needed for that day.

```
just day 1
```

This will do the following actions, only as needed:

 1. Download input for day 1 and save it to `input/d1`
 2. Create a blank example file `examples/d1`
 3. Create `src/d1.rs` (and add the module to `src/lib.rs`)
 4. Open the source file, input file, and example file in vim

## Watch for changes

```
just watch 1
```

## Days with multiple examples

[^1]: Most days contain only one example input, but some contain more.  For multi-example days, create more example files in the `examples` directory with names of your choosing, and use `-i/--input` to use them.  Here's [2021 day 12](https://adventofcode.com/2021/day/12) as an example, which contains three examples.  Let's say you save the first example in the default location `examples/d12`, the second to `examples/d12-2` and the third to `examples/d12-3`.

```
# run with the example input in the default example file: examples/d12
just run 12 -e

# run with example inputs you saved to examples/d12-2 and examples/d12-3
just run 12 -i examples/d12-2
just run 12 -i examples/d12-3
```

## Adding tests

If you like to tweak solutions after finding the answer, it can be helpful to write a simple test.

## Want to use this?

I set this up in the offseason, so I haven't used it yet during AoC.  I'm not sure I'd recommend it, but don't let that stop you.

## Pros

 - Fast compilation ([pico-args](https://crates.io/crates/pico-args) is the only dependency)
 - Fast LSP startup
 - No magical-feeling macros
 - All the code (other than pico-args) is here in the repo, so it can be changed on a whim

## Cons

 - No benchmarks
