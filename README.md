# TT
![build](https://github.com/angelocarly/tt/actions/workflows/rust.yml/badge.svg)  
Terminal timetracking software.

## Features
- Display total logged hours per month.
- Log new entries at the current time, or specify the time by hand.
- Open and edit scratchpad notes in your favorite $EDITOR.

## Installation
```shell
git clone https://github.com/angelocarly/tt.git
cd tt
cargo install --path .
```
You can now use the `tt` command to log your time. Data is stored under `$HOME/.tt/`.
Use `tt help` to get started.
