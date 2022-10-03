# cube

A 3x3x3 cube solver

## Usage

```bash
# Install `cube` command
$ cargo install --path .

# Input Samples
$ cat sample.input
$ cat sample2.input

# Solving only with U, D and F operations
$ cube -UDF < sample.input
[2022-09-06T06:24:39Z INFO ] Solution: DDU'
{"ok":true,"solutions":[{"algorithm":"DDU'","length":3}]}

# Solving with CFOP Method
$ cube --cfop < sample2.input

# Solving with Roux Method
$ cube --roux < sample2.input
```

## Format

### Example

![](https://user-images.githubusercontent.com/2749629/188441799-0f08adb8-709a-47ee-8c97-4039821ebeb1.png)

Solve left (**initial** state) into right (**goal** state).

```dot
Init {
  YYY
  YYY
  YYY
  GGG OOO BBB RRR
  RRR GGG OOO BBB
  OOO BBB RRR GGG
  WWW
  WWW
  WWW
}

Goal {
  YYY
  YYY
  YYY
  RRR GGG OOO BBB
  RRR GGG OOO BBB
  RRR GGG OOO BBB
  WWW
  WWW
  WWW
}
```

Or you can give **scramble**.

```dot
Scramble {
  U D2
}
```

`Goal { ... }` can be omitted.
The default is standard form, up-face is yellow and front-face is red.

![](https://user-images.githubusercontent.com/2749629/188440065-7c9c71d1-5b34-4899-8968-ecabee745863.png)

Standard form.

### Colors

- `W`: White
- `Y`: Yellow
- `B`: Blue
- `G`: Green
- `R`: Red
- `O`: Orange
- `.`: Other Color (`.` matches to `.`)
- `*`: Wildcard (Any colors matching)

### Operations

- `U D F B L R`, clockwise
- `U' D' F' B' L' R'`, counter-clockwise
- `U2 D2 F2 B2 L2 R2`, 180 rotate
- `u d f b l r`, double-layers rotation
  - `u' d' f' b' l' r'`
  - `u2 d2 f2 b2 l2 r2`
  - `Uw Dw Fw Bw Lw Rw`, also ok
- `M S E` `M' S' E'`
- `x y z`, (r) (u) (f)

### BNF Spec

```prolog
<Input> ::= <Entry> | <Entry> <Input>

<Entry> ::= Init { <Cube> }
          | Goal { <Cube> }
          | Scramble { <Operations> }

<Cube> ::= <Color> * 54

<Operations> ::= <Op> | <Op> <Operations>
```

White-spaces and new-lines are all ignored.
And comments can put anywhere.

```prolog
<Comment> ::= <CommentMarker> <LineString> <Newline>

<CommentMarker> ::= "#" | "//" | ";"

<LineString> ::= <any-char> | <any-char> <LineString>

<Newline> ::= "\r" | "\n"
```
