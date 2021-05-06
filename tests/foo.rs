#[macro_use] extern crate afmt;

/// A simple struct format
mod simple {
    #[fmt("value: " v)]
    struct Foo {
        v: u32,
    }

    #[test]
    fn parse_correctly() {
        let f: Foo = "value: 65".parse().unwrap();
        assert_eq!(f.v, 65);
    }

    #[test]
    fn literal_bad_match() {
        let f: Result<Foo, _> = "val: 35".parse();
        assert!(f.is_err());
    }

    #[test]
    fn capture_bad_parse() {
        let f: Result<Foo, _> = "value: 5x6".parse();
        assert!(f.is_err());
    }
}

/// Slightly more complex struc format
mod point {
    #[fmt("x[" x "] y[" y "]")]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn works() {
        let p: Point = "x[-34] y[79]".parse().unwrap();
        assert_eq!(p.x, -34);
        assert_eq!(p.y, 79);
    }
}
