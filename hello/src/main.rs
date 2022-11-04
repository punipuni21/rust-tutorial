fn main() {
    // Resultについて
    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }

    // unwqp_or_elseも使用できる
    let result2: Result<i32, String> = Ok(200);
    println!("code2: {}", result2.unwrap_or(-1))
}
