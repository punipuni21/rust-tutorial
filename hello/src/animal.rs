use crate::tweet::Tweet;
pub struct Dove;
pub struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("QUack!");
    }
}
