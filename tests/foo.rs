extern crate afmt;

use afmt::fmt;
use std::str::FromStr;

#[fmt("v: " v " f: " f)]
struct Foo {
    v: u32,
    f: f64,
}

#[fmt("<" level ">" name ": " msg)]
struct Log {
    level: u32,
    name: String,
    msg: String,
}

#[fmt()]
struct Empty;

#[fmt(v)]
struct OneValue {
    v: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let f: Foo = "v: 65 f: 3.1415".parse().unwrap();
        assert_eq!(f.v, 65);
        assert_eq!(f.f, 3.1415);
    }

    #[test]
    fn log() {
        let l: Log = "<65>my_func: hello this is the msg".parse().unwrap();
        assert_eq!(l.level, 65);
        assert_eq!(l.name, "my_func".to_owned());
        assert_eq!(l.msg, "hello this is the msg".to_owned());
    }

    #[test]
    fn complex_log() {
        let l: Log = "<43>func<>name: this<>is the msg".parse().unwrap();
        assert_eq!(l.level, 43);
        assert_eq!(l.name, "func<>name".to_owned());
        assert_eq!(l.msg, "this<>is the msg".to_owned());
    }

    #[test]
    fn empty() {
        let _e: Empty = "rand".parse().unwrap();
    }

    #[test]
    fn one_value() {
        let o: OneValue = "356".parse().unwrap();
        assert_eq!(o.v, 356);
    }

    #[test]
    fn with_radix() {
        let u = u32::from_str_radix("deadbeef", 16).unwrap();
        assert_eq!(u, 0xdeadbeef);
    }
}