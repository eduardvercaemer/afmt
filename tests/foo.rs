#[macro_use] extern crate afmt;

/// A simple struct format
mod simple {
    use std::error::Error;

    #[fmt("value: " v)]
    struct Foo {
        v: i32,
    }

    #[test]
    fn parse_positive() -> Result<(), Box<dyn Error>> {
        let f: Foo = "value: 65".parse()?;
        assert_eq!(f.v, 65);
        Ok(())
    }

    #[test]
    fn parse_negative() -> Result<(), Box<dyn Error>> {
        let f: Foo = "value: -45".parse()?;
        assert_eq!(f.v, -45);
        Ok(())
    }

    #[test]
    fn parse_zero() -> Result<(), Box<dyn Error>> {
        let f: Foo = "value: 000".parse()?;
        assert_eq!(f.v, 0);
        Ok(())
    }

    #[test]
    fn trailing_space_fails() -> Result<(), Box<dyn Error>> {
        let f: Result<Foo, _> = "value: 35  ".parse();
        assert!(f.is_err());
        Ok(())
    }

    #[test]
    fn empty_field_capture() -> Result<(), Box<dyn Error>> {
        let f: Result<Foo, _> = "value: ".parse();
        assert!(f.is_err());
        Ok(())
    }

    #[test]
    fn strlit_non_match() -> Result<(), Box<dyn Error>> {
        let f: Result<Foo, _> = "val: 35".parse();
        assert!(f.is_err());
        Ok(())
    }

    #[test]
    fn capture_non_parse() -> Result<(), Box<dyn Error>> {
        let f: Result<Foo, _> = "value: 5x6".parse();
        assert!(f.is_err());
        Ok(())
    }
}

/// Slightly more complex struct format
mod point {
    use std::error::Error;

    #[fmt("x[" x "] y[" y "]")]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn parse_correctly() -> Result<(), Box<dyn Error>> {
        let p: Point = "x[5] y[7]".parse()?;
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 7);
        Ok(())
    }
    
    #[test]
    fn capture_lookahead_non_match() -> Result<(), Box<dyn Error>> {
        let p: Result<Point, _> = "x[5] y[7}".parse();
        assert!(p.is_err());
        Ok(())
    }
}
