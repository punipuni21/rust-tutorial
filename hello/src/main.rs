mod impl_sample;
fn main() {
    let p = impl_sample::Person {
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name();
    p.say_age();
}
