# Description

Learning Rust from the rust lang book.

# Chapters

Here try to encode information as much as possible to have a quick review of the
topics such that you can review everything in like 1 or 2 hours. If needed, you
can always refer to the source for a detailed explanation on the topic. Keep the
detailed notes in the source code itself as comments.

## Ch 1: Getting started

* rustup: version manager for rust
* rustc: compiler
* cargo: rust's build system and package manager
    * cargo ; new, build (--release), run, check, update, doc --open

## Ch 2: First program, a guessing game

* *prelude*: items in std library in scope for every program

## Ch 3: Concepts

* mut, const
* i8, u8, i16, u16, ..., u128, isize, usize, f64(default), f32, bool,
char(4 Bytes)
* compound types; tuple(cannot be resized) x: (f64, u8) = (6.1, 2); unit (shown
with ()), when expressions don't return anything; array(fixed length, same type)
`let a: (i32; 5) = [1, 2, 3, 4, 5]`;
* define return type of a function with `->`. Functions implecitly return the
last expression, but one can also use return.

### Statement and Expression
*Statement*: instruction that does not return a value. e.g. `let x = 1;`

**Expression**: Evaluates to a resultant value. e.g.
```Rust
let x = {
    let x = 3;
    x + 1 // note there is no semicolon here, because it is an Expression.
};
```

* in Rust, loops can return values.
```Rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

* Can also label loops (with single quotes).
```rust
'counting_up': loop {
    let mut remaining = 10;
    if remaining == 9 {
        break;
    }
    if remaining == 2 {
        break 'counting_up';
    }
    remaining -= 1
}
```