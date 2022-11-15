use std::vec;

mod generics;
use generics::make_tuple;

fn main() {
    let t1 = make_tuple(1, 1);
    let t2 = make_tuple("Hello", "World");
    let t3 = make_tuple(vec![1, 2, 3], vec![4, 5]);
    let t4 = make_tuple(3, "years old");
}
