// use std::vec;

enum RealCat {
    Alive(CatStats),
    Dead,
}

// struct CatStats {
//     hungry: bool,
// }

// impl CatStats {
//     fn drop(self) {}
// }

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().map(|x| x * 5).sum();
    println!("{sum}");
    println!("fefwef");
}
