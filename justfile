set dotenv-load
set positional-arguments

FORCE_DEFAULT := ""

# scaffold a new day and open its files in vim
@day DAY:
  echo "starting day {{DAY}} year $AOC_YEAR"
  just input {{DAY}}
  just template {{DAY}}
  just update_lib {{DAY}}
  just vim {{DAY}}

# download DAY's input file (unless it exists already) (requires AOC_SESSION env)
@input DAY:
  if [ ! -f input/d{{DAY}} ]; then \
    mkdir input -p 2> /dev/null; \
    curl -s https://adventofcode.com/$AOC_YEAR/day/{{DAY}}/input -H "Cookie: session=${AOC_SESSION:?'please set AOC_SESSION'}" > input/d{{DAY}}; \
    echo "input saved: input/d{{DAY}}"; \
  else \
    echo "input for day {{DAY}} already exists in input/d{{DAY}}"; \
  fi

# create an empty file to hold DAY's example input (unless it exists already)
@example DAY:
  if [ ! -f examples/d{{DAY}} ]; then \
    mkdir examples -p 2> /dev/null; \
    touch examples/{{DAY}}; \
    echo "empty example created: examples/d{{DAY}}"; \
  else \
    echo "example for day {{DAY}} already exists in examples/d{{DAY}}"; \
  fi

# copy the solution template into src/ (unless it exists already)
@template DAY FORCE=FORCE_DEFAULT:
  if [ "{{FORCE}}" == "force" ] || [ ! -f src/d{{DAY}}.rs ]; then \
    cp ./templates/d.rs /tmp/_aoc_{{DAY}}.rs; \
    export AOC_DAY={{DAY}}; envsubst </tmp/_aoc_{{DAY}}.rs > src/d{{DAY}}.rs; \
    rm /tmp/_aoc_{{DAY}}.rs; \
    echo "source created: src/d{{DAY}}.rs"; \
  else \
    echo "source for {{DAY}} already exists in src/d{{DAY}}.rs"; \
  fi

# open DAY's files in vim (sol'n file, example file, and input file)
@vim DAY:
  ~/neovim/bin/nvim src/d{{DAY}}.rs examples/d{{DAY}} input/d{{DAY}} 

# update src/lib.rs with the new day
@update_lib DAY:
  echo "pub mod d{{DAY}};" >> src/lib.rs
  sort -u src/lib.rs -o src/lib.rs

# shorthand for cargo run
@run *ARGS:
  cargo r -- {{ARGS}}

# shorthand for cargo run -r
@rrun *ARGS:
  cargo r -r -- {{ARGS}}

# run with console visualization (not all days have this)
@viz *ARGS:
  cargo r -r -F visualize -- {{ARGS}}
