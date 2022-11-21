struct Droppable;

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Resource will be released!");
    }
}

pub fn droppable() -> () {
    {
        let d = Droppable;
    }
    println!("The Droppale should be released at the end of block.");
}
