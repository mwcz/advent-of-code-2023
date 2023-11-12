set dotenv-load
set positional-arguments

FORCE_DEFAULT := ""

# scaffold a new day and open its files in vim
@day DAY:
  echo "starting day {{DAY}} year $AOC_YEAR"
  just input {{DAY}}
  just example {{DAY}}
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
    touch examples/d{{DAY}}; \
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

# open DAY's files in vim or neovim (src, input, & example)
@vim DAY:
  commands=(${NVIM_BIN:-"nvim"} nvim vim); \
  for cmd in "${commands[@]}"; do \
    if command -v "$cmd" > /dev/null; then \
      echo "opening day {{DAY}}'s files with $cmd"; \
      just files {{DAY}} | xargs "$cmd"; \
      break; \
    fi \
  done \

# update src/lib.rs with the new day
@update_lib DAY:
  echo "pub mod d{{DAY}};" >> src/lib.rs
  sort -nu src/lib.rs -o src/lib.rs

# shorthand for cargo run
@run DAY *ARGS:
  cargo r -- -d {{DAY}} {{ARGS}}

# shorthand for cargo run -r
@r DAY *ARGS:
  cargo r -r -- -d {{DAY}} {{ARGS}}

# run with console visualization (not all days have this)
@viz DAY *ARGS:
  cargo r -r -F visualize -- -d {{DAY}} {{ARGS}}

# run CMD when DAY's files change (src, input, & example)
@watch CMD DAY *ARGS:
  just files {{DAY}} | entr -c just {{CMD}} {{DAY}} {{ARGS}}

# retrieve the files used for a given day
@files DAY:
  { \
    find src -name "d{{DAY}}.rs"; \
    echo "examples/d{{DAY}}"; \
    find examples -name "d{{DAY}}-*"; \
    find input -name "d{{DAY}}"; \
  }
  # the echo adds the example file to the ouput even if it doesn't exist yet

# run tests (using cargo-nextest)
@test DAY *ARGS:
  cargo nextest run -E "test(/d{{DAY}}p/)" {{ARGS}}

# run ALL tests (using cargo-nextest)
@test_all *ARGS:
  cargo nextest run {{ARGS}}
