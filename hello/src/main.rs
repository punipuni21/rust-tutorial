mod box_sample;
fn main() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    box_sample::print(Box::new(byte_array));
}
