data-macro-rs
=============

Ever wanted to define a simple struct, without worrying about the whole struct/impl/new() rigamarole?

No? 

Well, now you can anyway.

```rust
#![feature(phase)]
#[phase(plugin)]
extern crate data_macro;

data!(
    SomeData {
        a: uint = 1             // fields can be set as normal
        b: uint = a + 2         // fields can reference other fields
        c: uint = mul_by_four(b)// fields can be set via functions
    }                           // commas and semicolons are unnecessary
);

fn mul_by_four(num: uint) -> uint {
    println!("Multiplying {} by 4", num);
    num * 4
}

fn main() {
    let data: SomeData = SomeData::new();
    println!("data.c is {}", data.c);
}
```
