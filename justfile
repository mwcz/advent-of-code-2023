set dotenv-load
set positional-arguments

@day DAY:
  echo "starting day {{DAY}} year $AOC_YEAR"
  just input {{DAY}}
  just template {{DAY}}
  just vim {{DAY}}

@input DAY:
  if [ ! -f inputs/d{{DAY}} ]; then \
    mkdir inputs -p 2> /dev/null; \
    curl -s https://adventofcode.com/$AOC_YEAR/day/{{DAY}}/input -H "Cookie: session=$AOC_SESSION" > inputs/d{{DAY}}; \
    echo "input saved: inputs/d{{DAY}}"; \
  else \
    echo "input for day {{DAY}} already exists in inputs/d{{DAY}}"; \
  fi

@example DAY:
  if [ ! -f examples/d{{DAY}} ]; then \
    mkdir examples -p 2> /dev/null; \
    touch examples/{{DAY}}; \
    echo "empty example created: examples/d{{DAY}}"; \
  else \
    echo "example for day {{DAY}} already exists in examples/d{{DAY}}"; \
  fi

@template DAY:
  if [ ! -f src/d{{DAY}}.rs ]; then \
    cp ./templates/d.rs /tmp/_aoc_{{DAY}}.rs; \
    export AOC_DAY={{DAY}}; envsubst </tmp/_aoc_{{DAY}}.rs > src/d{{DAY}}.rs; \
    rm /tmp/_aoc_{{DAY}}.rs; \
    echo "source created: src/d{{DAY}}.rs"; \
  else \
    echo "source for {{DAY}} already exists in src/d{{DAY}}.rs"; \
  fi

@vim DAY:
  ~/neovim/bin/nvim src/d{{DAY}}.rs examples/d{{DAY}} input/d{{DAY}} 

@run *ARGS:
  cargo r -- {{ARGS}}
