# cube
cube solver

## Usage

```bash
$ cargo run -- --help

# Solving with U, D and F
$ cargo run -- -UDF < sample.input

Init:
YYY
YYY
YYY
GGGOOOBBBRRR
RRRGGGOOOBBB
OOOBBBRRRGGG
WWW
WWW
WWW

Goal:
YYY
YYY
YYY
RRRGGGOOOBBB
RRRGGGOOOBBB
RRRGGGOOOBBB
WWW
WWW
WWW

Solved: DDU'
```

## Format

### Colors

- `W`: White
- `Y`: Yellow
- `B`: Blue
- `G`: Green
- `R`: Red
- `O`: Orange
- `.`: Other Color (`.` matches to `.`)
- `*`: Wildcard (Any colors matching)

### A Face

A face is a matrix of 3x3 colors.

Example:

```
WBW
WYB
ORG
```

### A Cube

A cube is written as a cube net.

```
(U-Face)
(F-Face)(R-Face)(B-Face)(L-Face)
(D-Face)
```

For example, the following is the solved state.

```
YYY
YYY
YYY
RRRGGGOOOBBB
RRRGGGOOOBBB
RRRGGGOOOBBB
WWW
WWW
WWW
```

### Input Format

Solver `cube` read an initial state and  a goal state.
The 2 states are separated with an empty line.


```
YYY
YYY
YYY
GGGOOOBBBRRR
RRRGGGOOOBBB
OOOBBBRRRGGG
WWW
WWW
WWW

YYY
YYY
YYY
RRRGGGOOOBBB
RRRGGGOOOBBB
RRRGGGOOOBBB
WWW
WWW
WWW
```

