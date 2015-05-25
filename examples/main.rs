#[macro_use]
extern crate data_macro;

data!(
    SomeData {
        a: usize = 1             // fields can be set as normal
        b: usize = a + 2         // fields can reference other fields
        c: usize = mul_by_four(b)// fields can be set via functions
    }                           // commas and semicolons are unnecessary
);

fn mul_by_four(num: usize) -> usize {
    println!("Multiplying {} by 4", num);
    num * 4
}

fn main() {
    let data: SomeData = SomeData::new();
    println!("data.c is {}", data.c);
}
