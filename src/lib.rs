#![warn(missing_docs)]

/*!
Create to convert a type which implements the[Optionaler](trait.Optionaler.html)
trait into a single Option guarded tuple, using a macro.

## Examples

From time to time we have to handle a lot of Optional
values (containing different types), but procceding makes only sense when
all variables contain a value, [`ok_tup!`](macro.ok_tup.html) provides some syntax-sugar here.

```rust
use ok_tup::ok_tup;

let a = Some(1);
let b = Ok::<_, ()>("jay".to_owned());

if let Some((number, name)) = ok_tup!(a, b) {
    println!("num: {}  name: {}", number, name);
}
```

By implementing the [Optionaler](trait.Optionaler.html) trait,
it is possible to use any type with [`ok_tup!`](macro.ok_tup.html).

```rust
use ok_tup::{ok_tup, Optionaler};

#[derive(Debug)]
struct Foo {
    x: i32
}

impl Optionaler<Foo> for Foo {
    fn okay(self) -> Option<Foo> {
        if self.x == 42 { Some(self) } else { None }
    }
}

// The Foo struct can now be used with ok_tup!
let a = Some(1);
let b = Some("jay".to_owned());
let c = Foo{x: 42};

if let Some((num, name, foo)) = ok_tup!(a, b, c) {
    println!("num: {}  name: {}  foo: {:?}", num, name, foo);
}
```
*/

#[macro_export]
macro_rules! ok_tup {
    ($a:expr) => ({
        use $crate::Optionaler;
        if let Some(a)
        = Optionaler::okay($a) {
            Some((a,))
        } else { None }
    });
    ($a:expr, $b:expr) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
        ) {
            Some((a, b))
        } else { None }
    });
    ($a:expr, $b:expr, $c:expr) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
        ) {
            Some((a, b, c))
        } else { None }
    });
    ($a:expr, $b:expr, $c:expr, $d:expr) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
        ) {
            Some((a, b, c, d))
        } else { None }
    });
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d), Some(e))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
            Optionaler::okay($e),
        ) {
            Some((a, b, c, d, e))
        } else { None }
    });
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
            Optionaler::okay($e),
            Optionaler::okay($f),
        ) {
            Some((a, b, c, d, e, f))
        } else { None }
    });
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
            Optionaler::okay($e),
            Optionaler::okay($f),
            Optionaler::okay($g),
        ) {
            Some((a, b, c, d, e, f, g))
        } else { None }
    });
    (
        $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr
    ) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g),
            Some(h))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
            Optionaler::okay($e),
            Optionaler::okay($f),
            Optionaler::okay($g),
            Optionaler::okay($h),
        ) {
            Some((a, b, c, d, e, f, g, h))
        } else { None }
    });
    (
        $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr,
        $i:expr
    ) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g),
            Some(h), Some(i))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
            Optionaler::okay($e),
            Optionaler::okay($f),
            Optionaler::okay($g),
            Optionaler::okay($h),
            Optionaler::okay($i),
        ) {
            Some((a, b, c, d, e, f, g, h, i))
        } else { None }
    });
    (
        $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr,
        $i:expr, $j:expr
    ) => ({
        use $crate::Optionaler;
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g),
            Some(h), Some(i), Some(j))
        = (
            Optionaler::okay($a),
            Optionaler::okay($b),
            Optionaler::okay($c),
            Optionaler::okay($d),
            Optionaler::okay($e),
            Optionaler::okay($f),
            Optionaler::okay($g),
            Optionaler::okay($h),
            Optionaler::okay($i),
            Optionaler::okay($j),
        ) {
            Some((a, b, c, d, e, f, g, h, i, j))
        } else { None }
    });
    // TODO wirte a generic if-let general matcher
    ($($x:expr),+) => ({
        use $crate::Optionaler;
        if $(Optionaler::okay($x).is_some()) &&* {
            Some((
                $(Optionaler::okay($x).unwrap()),+
                ,))
        } else { None }
    });
}

/// A type that can be converted into an Option.
pub trait Optionaler<T> {
    /// Convert type T into an Option.
    fn okay(self) -> Option<T>;
}

impl<T> Optionaler<T> for Option<T> {
    #[inline]
    fn okay(self) -> Option<T> {
        self
    }
}

impl<'a, T: Clone> Optionaler<T> for &'a Option<T> {
    #[inline]
    fn okay(self) -> Option<T> {
        self.clone()
    }
}

impl<T, E> Optionaler<T> for Result<T, E> {
    #[inline]
    fn okay(self) -> Option<T> {
        Result::ok(self)
    }
}

impl<'a, T: Clone, E> Optionaler<T> for &'a Result<T, E> {
    #[inline]
    fn okay(self) -> Option<T> {
        match self {
            Ok(x) => Some(x.clone()),
            Err(_) => None,
        }
    }
}

impl Optionaler<std::ffi::OsString> for std::path::PathBuf {
    #[inline]
    fn okay(self) -> Option<std::ffi::OsString> {
        Some(self.into())
    }
}

impl<'a> Optionaler<std::ffi::OsString> for &'a std::path::Path {
    #[inline]
    fn okay(self) -> Option<std::ffi::OsString> {
        Some(self.to_path_buf().into())
    }
}

impl<'a> Optionaler<std::path::PathBuf> for &'a std::path::Path {
    #[inline]
    fn okay(self) -> Option<std::path::PathBuf> {
        Some(self.into())
    }
}

impl<'a> Optionaler<String> for &'a std::path::Path {
    #[inline]
    fn okay(self) -> Option<String> {
        self.to_str().map(str::to_owned)
    }
}

impl Optionaler<std::path::PathBuf> for std::ffi::OsString {
    #[inline]
    fn okay(self) -> Option<std::path::PathBuf> {
        Some(self.into())
    }
}

impl<'a> Optionaler<std::ffi::OsString> for &'a std::ffi::OsStr {
    #[inline]
    fn okay(self) -> Option<std::ffi::OsString> {
        Some(self.to_os_string())
    }
}

impl<'a> Optionaler<std::path::PathBuf> for &'a std::ffi::OsStr {
    #[inline]
    fn okay(self) -> Option<std::path::PathBuf> {
        Some(self.into())
    }
}

impl<'a> Optionaler<String> for &'a std::ffi::OsStr {
    #[inline]
    fn okay(self) -> Option<String> {
        self.to_str().map(str::to_owned)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[allow(unused_variables)]
    fn test_ok_tup() {
        let a: Option<(_,)> = ok_tup!(Ok::<_, String>(1));
        let a: Option<(_, _)> = ok_tup!(Ok::<_, String>(1), Some(2));
        let a: Option<(_, _, _)> = ok_tup!(Some(1), Some(2), Some(3));
        let a: Option<(i32, f32, u8, f64)> =
            ok_tup!(Some(1_i32), Some(2_f32), Some(3_u8), Some(4_f64));
        let a: Option<(_, _, _, _, _)> =
            ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5));
        let a: Option<(_, _, _, _, _, _)> =
            ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6));
        let a: Option<(_, _, _, _, _, _, _)> = ok_tup!(
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7)
        );
        let a: Option<(_, _, _, _, _, _, _, _)> = ok_tup!(
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8)
        );
        let a: Option<(_, _, _, _, _, _, _, _, _)> = ok_tup!(
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9)
        );
        let a: Option<(_, _, _, _, _, _, _, _, _, _)> = ok_tup!(
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10)
        );
        let a: Option<(_, _, _, _, _, _, _, _, _, _, String)> = ok_tup!(
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some("a".to_owned())
        );

        let s: &str = "hello";
        ok_tup!(Some(s));
        let s: Option<i32> = Some(1);
        ok_tup!(&s);

        let s: &str = "hello";
        ok_tup!(Ok::<_, i32>(s));
        let s: Result<_, i32> = Ok(1);
        ok_tup!(&s);
    }

    #[test]
    fn test_from_pathbuf() {
        use std::ffi::OsStr;
        use std::path::PathBuf;

        let pb = PathBuf::from("hi");
        let _foo: Option<(&OsStr,)> = ok_tup!(pb.file_stem());
    }

    #[test]
    fn test_from_path() {
        use std::ffi::OsString;
        use std::path::Path;
        use std::path::PathBuf;

        let p = Path::new("hi");
        let _foo: Option<(OsString,)> = ok_tup!(p);

        let p = Path::new("hi");
        let _foo: Option<(PathBuf,)> = ok_tup!(p);
    }

    #[test]
    fn test_from_osstring() {
        use std::ffi::OsString;
        use std::path::PathBuf;

        let s: OsString = "hi".into();
        let _foo: Option<(PathBuf,)> = ok_tup!(s);
    }

    #[test]
    fn test_from_osstr() {
        use std::ffi::OsStr;
        use std::ffi::OsString;
        use std::path::PathBuf;

        let p = OsStr::new("hi");
        let _foo: Option<(OsString,)> = ok_tup!(p);

        let p = OsStr::new("hi");
        let _foo: Option<(PathBuf,)> = ok_tup!(p);
    }
}
