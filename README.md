#### How to trigger /rust-lang/cargo/issues/6430

``` bash
$ git clone <repo>

# IMPORTANT! to trigger the dead_code warning
$ cargo clean

$ cargo check --tests
```

#### environment
``` bash
$ rustc --version (nightly)
rustc 1.31.0 (abe02cefd 2018-12-04)

$ cargo --version
cargo 1.31.0 (339d9f9c8 2018-11-16)
```

#### Output

``` bash
$ cargo clean ; cargo_check --tests
    Checking open-taffeta v0.1.0 (/home/jman/tmp/cargo_issue_6430)
warning: struct is never constructed: `MyStruct`
 --> tests/submod.rs:1:1
  |
1 | pub struct MyStruct {
  | ^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: method is never used: `new`
 --> tests/submod.rs:7:5
  |
7 |     pub fn new() -> MyStruct {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: method is never used: `add_num`
  --> tests/submod.rs:11:5
   |
11 |     pub fn add_num(&self) {
   |     ^^^^^^^^^^^^^^^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
```



See file `tests/test_2.rs` on how to make the false positive disappear
