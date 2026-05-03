use std::convert::Infallible;
use std::io;

#[test]
fn can_write_any_unsigned_int() {
    fn f(x: u64) -> io::Result<()> {
        let mut v = vec![];
        never_fails(leb128::write::unsigned(&mut v, x));
        Ok(())
    }
    quickcheck::quickcheck(f as fn(u64) -> io::Result<()>);
}

#[test]
fn can_round_trip_any_unsigned_int() {
    fn f(x: u64) -> bool {
        let mut v = vec![];
        never_fails(leb128::write::unsigned(&mut v, x));
        let y = never_fails2(leb128::read::unsigned(&mut &v[..]));
        x == y
    }
    quickcheck::quickcheck(f as fn(u64) -> bool);
}

#[test]
fn can_write_any_signed_int() {
    fn f(x: i64) -> io::Result<()> {
        let mut v = vec![];
        never_fails(leb128::write::signed(&mut v, x));
        Ok(())
    }
    quickcheck::quickcheck(f as fn(i64) -> io::Result<()>);
}

#[test]
fn can_round_trip_any_signed_int() {
    fn f(x: i64) -> io::Result<bool> {
        let mut v = vec![];
        never_fails(leb128::write::signed(&mut v, x));
        let y = never_fails2(leb128::read::signed(&mut &v[..]));
        Ok(x == y)
    }
    quickcheck::quickcheck(f as fn(i64) -> io::Result<bool>);
}

fn never_fails<T>(r: Result<T, Infallible>) -> T {
    r.unwrap()
}

fn never_fails2<T>(r: Result<T, leb128::read::Error<std::convert::Infallible>>) -> T {
    r.unwrap()
}
