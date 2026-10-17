# Knack Pipe

**Rust** · **v0.1.0**

_Line, word and character counter reading stdin_

---

## Features

- wc-style counts from stdin
- Handles arbitrary Unicode text
- Zero external dependencies

## Installation

```bash
git clone https://github.com/DevCode01/knack-pipe.git
cd knack-pipe
```

## Usage

```bash
cat file.txt | cargo run
```

## How it works

Reads all of stdin into memory and reports line, word and character counts (`wc`-style). Words are whitespace-delimited, characters count Unicode code points.

## Options

- no arguments - reads stdin, prints lines/words/characters

## Example

```bash
$ printf 'hello world
foo bar baz
' | cargo run
Lines:      2
Words:      5
Characters: 24
```

## Testing

```bash
bash tests/run.sh
```

Includes a functional check against real captured output, plus the build/syntax check.

## License

MIT
