# ok_tup

Create to convert a type which implements the `Optionaler` trait
into a single Option guarded tuple, using a macro.

Version: 0.2.1


## Installation

Add to `Cargo.toml`:
```toml
[dependencies.ok_tup]
git = "https://github.com/thibran/ok_tup.git"
tag = "v0.2.1"
```


## Examples

From time to time we have to handle a lot of Optional
values (containing different types), but procceding makes only sense when
all variables contain a value, `ok_tup!` provides some syntax-sugar here.

```rust
use ok_tup::ok_tup;

let a = Some(1);
let b = Ok::<_, ()>("jay".to_owned());

if let Some((number, name)) = ok_tup!(a, b) {
    println!("num: {}  name: {}", number, name);
}
```

By implementing the `Optionaler` trait,
it is possible to use any type with `ok_tup!`.

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