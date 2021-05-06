# afmt

Simple rust library for parsing basic data structures from
strings.

## Usage

You can specify string formats to any strucute, via the use of the `fmt` macro,
in which you can specify a chain of string literals and struct member names, to
represent the format of a structure.

```rust
#[fmt("value: " v "--" f)]
struct Foo {
    v: u32,
    r: f64,
}

let f: Foo = "value: 65--3.14".parse()?;
```

## Limitations

Consider we want to parse strings similar to
```
some text here 364
```
into a struct with a `String` first part and a `u32` second part.

If we tried parsing it as the following format attribute
```rust
#[fmt(msg v)]
struct Foo {
    msg: String,
    v: u32,
}
```
the result would be ambiguous, multiple interpretations could be
```rust
Foo {
    msg: "some text here ",
    v: 364,
}
Foo {
    msg: "some text here 3",
    v: 64,
}
Foo {
    msg: "some text here 36",
    v: 4,
}
            ...
```

this means that we _must_ specify some literal delimiter between every pair
of capture variables. Literal delimiters are parsed in a way that the first
match splits the string, so a delimiter of `" "` would result in the two parts
beign `"some"` and `"text here 364"`, which is not ideal.

For this reason, you should consider which strings you can easily parse, by
considering special delimiters, i.e. delimiters should not appear in data you
want to capture.

## Examples

```rust
#[macro_use] extern crate afmt;
```

```rust
#[fmt("value :" v)]
struct Foo {
    v: u32,
}

#[test]
fn it_works() {
    let f: Foo = "value: 65".parse().unwrap();
    assert_eq!(f.v, 65);
}
```

```rust
#[fmt("x[" x "] y[" y "]")]
struct Point {
    x: u32,
    y: u32,
}

#[test]
fn it_works() {
    let p: Point = "x[65] y[39]".parse().unwrap();
    assert_eq!(p.x, 65);
    assert_eq!(p.y, 39);
}
```

```rust
#[fmt("INFO: " msg)]
struct Bar {
    msg: String,
}

#[test]
fn it_works() {
    let b: Result<Bar,_> = "WARN: this is a warning".parse();
    assert!(b.is_err());
}
```