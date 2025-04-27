This crate defines ToBoxed Trait to its derive

``` rust

use to_boxed::ToBoxed;

#[derive(ToBoxed)]
struct Hello {
    a: i32,
}

fn say_hello() {
    let hello = Hello { a: 1 };
    let boxed_hello = hello.to_boxed();
}
```

