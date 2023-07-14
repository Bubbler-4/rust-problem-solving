# cargo-oj: Solution packer for Online Judges

## What does it do?

All online judges (OJs) require you to write and submit a single file.
But keeping all the I/O and algorithm snippets and copy-pasting them into the main file is not only cumbersome,
but also against the spirit of modular code.

`cargo-oj` lets you keep your code organized in a module structure inside a library crate,
and it will happily pack your `lib.rs` and its children into a `main.rs`,
removing all the unnecessary bloat (which is dead code) along the way.

## How to use it?

* Create a library crate.
* Write your solution in `lib.rs`, which should contain a `pub fn main() {...}`. Optionally put

    ```rs
    #![allow(dead_code)]
    #![allow(unused_imports)]
    ```

    at the top of `lib.rs`. This will eliminate warnings about unused items (mostly data structures and algorithms) in your IDE.
    `cargo-oj` strips these declarations before testing for dead code.

* Write your own library as child modules of `lib.rs`. **Mark all items that you will directly use as `pub(crate)`.**
    This lets you use those items in different modules within the same crate (especially `lib.rs`),
    while letting `rustc` emit unused warnings when appropriate.
* When your solution is complete, run `cargo oj` at the crate root folder. `src/bin/main.rs` will be created.
    Then you can:
    * run it locally via `cargo run --bin main --release`, and/or
    * submit the file using `cargo-boj` or other tools.

## Installation

```
cargo install --path=.
```

## Usage

```
cargo oj
```
