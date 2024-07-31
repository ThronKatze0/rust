mod mhm;
mod test;

use test::CoolStruct;

fn main() {
    println!("Hello, world!");
    let var: CoolStruct = CoolStruct { test: 45 };
    var.print_test();
}
