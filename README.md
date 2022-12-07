# Advent of code 2022

https://adventofcode.com/

## Requirements

```
cargo install cargo-watch
cargo install hyperfine
```

## Commandes

add day :

```bash
cargo new dayXX --vcs none
```

run :

```bash
cargo watch -x "run -p dayXX"
```

test :

```bash
cargo watch -x "test -p dayXX"
```

benchmark :

```bash
cargo build --release -p dayXX 
hyperfine --warmup 5 -N .\target\release\dayXX.exe
hyperfine --warmup 5 -N ./target/release/dayXX
```
