<div align="center">

<h1>
  <span style="font-size: 80px;">Axe</span>
<picture>
  <img height="80" src="icon.svg"/>
</picture>
</h1>

[![Build status](https://github.com/jacek-kurlit/axe/actions/workflows/on_merge.yml/badge.svg)](https://github.com/jacek-kurlit/axe/actions)
[![GitHub Release](https://img.shields.io/github/v/release/jacek-kurlit/axe)](https://github.com/jacek-kurlit/axe/releases/latest)

</div>

Argument execute is xargs alternative that focus on arguments processing and ordering.

This tool is still under heavy development

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [How does Axe work](#how-does-axe-work)
- [Motivation](#motivation)
- [Development](#development)

## Features

### Placing arguments in defined order
```sh
echo "foo bar baz" | axe echo {2} {1} {0}
```
output: `baz bar foo`

### Processing input as data series
```sh
echo "a b c\nd e f" | axe echo {}
```
output:
```text
a b c
d e f
```
Explanation:
Each new line is treated as a data entry.
Echo command will be run for each line.
This is equal to calling
```sh
echo "a b c"
echo "d e f"
```

### Arguments splitting for individual arguments

```sh
echo "lord_of_the_rings.txt" | axe echo "this is file name '{0.0}' and this is file extension '{0.1}'"
```
output: `this is file name 'lord_of_the_rings' and this is file extension 'txt'`

### Arguments splitting for array arguments

```sh
echo "a_b c_d e_f" | axe echo {_0}
```
output: `a c e`

## Installation

**[Archives of precompiled binaries for axe are available for Linux and macOS.](https://github.com/jacek-kurlit/axe/releases)**

With **[dra](https://github.com/devmatteini/dra)** 

```sh
dra download -i -a -o ~/.local/bin jacek-kurlit/axe
```

With **[eget](https://github.com/zyedidia/eget)** 

```sh
eget jacek-kurlit/axe --to=~/.local/bin
```

If you're a **Rust programmer**, axe can be installed with `cargo`.

```sh
cargo install axe-cli
```

## How does axe work?

### Entries and command execution
Lets consider the following example stdin input:
```text
a b c
d e f
h i j
```
If we pass it to `axe echo {}` axe will call echo 3 times for each line.
On each line space will be treated as an arguments separator.
As result axe will resolve and call echo like this:
```text
echo a b c
echo d e f
echo h i j
```
### Arguments resolution

Each entry line will be split by arguments separator that can be chosen by the user (by default space).
You may then refer to each argument by its index.
For example:
```sh
echo "a b c" | axe echo {0}
```
There is only one entry with 3 arguments.
Echo will print first argument which is `a`.

### Arguments splitting
Each argument can be splitted into multiple arguments.
For example:
```sh
echo "a.b c.d e.d_f.g" | axe echo {1.1} {0.0} {2_0}
```
Axe will read this as follows:
- {1.1} split second argument ('c.d') by '.' and choose second part (`d`)
- {0.0} split first argument ('a.b') by '.' and choose first part (`a`)
- {2_0} split third argument ('e.d_f.g') by '_' and choose first part (`e.d`)

### Arguments resolving into arrays
Arguments can be resolved into arrays.
For example:
```sh
echo "f1.txt f2.txt f3.txt" | axe echo {.0}
```
Axe will read this as follows:
{.0} split all arguments by '.' and for each item choose first part (`f1, f2, f3`)

You may split each argument into multiple items

```sh
echo "f1.txt f2.txt f3.txt" | axe echo {0.} {1}
```
Axe will read this as follows:
{0.} split first argument by '.' and choose all parts (`f1, txt`)
{1} take second argument(`f2.txt`)

Output `f1 txt f2.txt`

## Motivation
Every time I was using xargs command I was frustrated that I cannot tell where I want to place arguments.
This missing feature made me decide to create my own tool.

## Development

### Setup

- Rust 1.81+
- Cargo make [link](https://github.com/sagiegurari/cargo-make)
- Cargo nextest [link](https://github.com/nextest-rs/nextest)

### Building

```sh
git clone https://github.com/jacek-kurlit/axe
cd axe
cargo build --release
./target/release/axe --version
```
## Special thanks to
- [icon creator](https://freeicons.io/profile/5790)
