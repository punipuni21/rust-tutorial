pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn say_name(&self) {
        println!("I am {}", self.name);
    }

    pub fn say_age(&self) {
        println!("I am {} years old", self.age);
    }
}
