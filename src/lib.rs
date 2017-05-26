#![warn(missing_docs)]

//! The [ok_tup!](macro.ok_tup.html) macro allows to unwrap multiple Option or Result values
//! at once, guarded by a single Option wrapper.
//!
//! From time to time we have to handle e.g. a lot of Optional
//! values (containing different types), but procceding makes only sense when
//! all variables contain a value, [ok_tup!](macro.ok_tup.html)
//! provides some syntax-sugar here.
//!
//! #Example
//! ```
//! #[macro_use]
//! extern crate ok_tup;
//!
//! let a: Option<i32>    = Some(1);
//! let b: Result<String> = Ok("jay".to_owned());
//!
//! if let Some((number, name)) = ok_tup!(a, b) {
//!     println!("num: {}  name: {}", number, name);
//! }
//! ```
//!
//! By implementing the [ok_tup::Optionaler](trait.Optionaler.html) trait,
//! it is possible to use any type with the [ok_tup!](macro.ok_tup.html) macro.
//!
//! ```
//! # #[macro_use]
//! # extern crate ok_tup;
//! use ok_tup::Optionaler;
//!
//! #[derive(Debug)]
//! struct Foo {
//!     x: i32
//! }
//! 
//! impl Optionaler<Foo> for Foo {
//!     fn okay(self) -> Option<Foo> {
//!         if self.x == 42 { Some(self) } else { None }
//!     }
//! }
//!
//! // The Foo struct can now be used with ok_tup! 
//! let a = Some(1);
//! let b = Some("jay".to_owned());
//! let c = Foo{x: 42};
//!
//! if let Some((num, name, foo)) = ok_tup!(a, b, c) {
//!     println!("num: {}  name: {}  foo: {:?}", num, name, foo);
//! }
//! ```


/// Convert a type which implements [ok_tup::Optionaler](trait.Optionaler.html) into a single Option guarded tuple.
///
/// ```
/// # #[macro_use]
/// # extern crate ok_tup;
/// let a: Option<i32>    = Some(1);
/// let b: Result<String> = Ok("jay".to_owned());
///
/// if let Some((number, name)) = ok_tup!(a, b) {
///     println!("num: {}  name: {}", number, name);
/// }
/// ```
#[macro_export]
macro_rules! ok_tup {
    ($a:expr) => {
        if let (Some(a),)
        = ($crate::Optionaler::okay($a),) {
            Some((a,))
        } else { None }
    };
    ($a:expr, $b:expr) => {
        if let (Some(a), Some(b))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
        ) {
            Some((a, b))
        } else { None }
    };
    ($a:expr, $b:expr, $c:expr) => {
        if let (Some(a), Some(b), Some(c))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
        ) {
            Some((a, b, c))
        } else { None }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        if let (Some(a), Some(b), Some(c), Some(d))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
        ) {
            Some((a, b, c, d))
        } else { None }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        if let (Some(a), Some(b), Some(c), Some(d), Some(e))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
            $crate::Optionaler::okay($e),
        ) {
            Some((a, b, c, d, e))
        } else { None }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
            $crate::Optionaler::okay($e),
            $crate::Optionaler::okay($f),
        ) {
            Some((a, b, c, d, e, f))
        } else { None }
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
            $crate::Optionaler::okay($e),
            $crate::Optionaler::okay($f),
            $crate::Optionaler::okay($g),
        ) {
            Some((a, b, c, d, e, f, g))
        } else { None }
    };
    (
        $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr
    ) => {
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g),
            Some(h))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
            $crate::Optionaler::okay($e),
            $crate::Optionaler::okay($f),
            $crate::Optionaler::okay($g),
            $crate::Optionaler::okay($h),
        ) {
            Some((a, b, c, d, e, f, g, h))
        } else { None }
    };
    (
        $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr,
        $i:expr
    ) => {
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g),
            Some(h), Some(i))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
            $crate::Optionaler::okay($e),
            $crate::Optionaler::okay($f),
            $crate::Optionaler::okay($g),
            $crate::Optionaler::okay($h),
            $crate::Optionaler::okay($i),
        ) {
            Some((a, b, c, d, e, f, g, h, i))
        } else { None }
    };
    (
        $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr,
        $i:expr, $j:expr
    ) => {
        if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g),
            Some(h), Some(i), Some(j))
        = (
            $crate::Optionaler::okay($a),
            $crate::Optionaler::okay($b),
            $crate::Optionaler::okay($c),
            $crate::Optionaler::okay($d),
            $crate::Optionaler::okay($e),
            $crate::Optionaler::okay($f),
            $crate::Optionaler::okay($g),
            $crate::Optionaler::okay($h),
            $crate::Optionaler::okay($i),
            $crate::Optionaler::okay($j),
        ) {
            Some((a, b, c, d, e, f, g, h, i, j))
        } else { None }
    };
    // TODO wirte a generic if-let general matcher
    ($($x:expr),*) => {
        if $($crate::Optionaler::okay($x).is_some()) &&* {
            Some((
                $($crate::Optionaler::okay($x).unwrap()),*
                ,))
        } else { None }
    };
}

/// A type that can be converted into an Option.
pub trait Optionaler<T> {
    /// Convert type T into an Option.
    fn okay(self) -> Option<T>;
}

impl<T, E> Optionaler<T> for Result<T, E> {
    #[inline]
    fn okay(self) -> Option<T> {
        Result::ok(self)
    }
}

impl<T> Optionaler<T> for Option<T> {
    #[inline]
    fn okay(self) -> Option<T> {
        self
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
        let a: Option<(i32, f32, u8, f64)>
            = ok_tup!(Some(1_i32), Some(2_f32), Some(3_u8), Some(4_f64));
        let a: Option<(_, _, _, _, _)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5));
        let a: Option<(_, _, _, _, _, _)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6));
        let a: Option<(_, _, _, _, _, _, _)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
                Some(7));
        let a: Option<(_, _, _, _, _, _, _, _)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
                Some(7), Some(8));
        let a: Option<(_, _, _, _, _, _, _, _, _)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
                Some(7), Some(8), Some(9));
        let a: Option<(_, _, _, _, _, _, _, _, _, _)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
                Some(7), Some(8), Some(9), Some(10));
        let a: Option<(_, _, _, _, _, _, _, _, _, _, String)>
            = ok_tup!(Some(1), Some(2), Some(3), Some(4), Some(5), Some(6),
                Some(7), Some(8), Some(9), Some(10), Some("a".to_owned()));
    }
}