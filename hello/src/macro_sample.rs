pub fn macro_sample() -> () {
    //プログラム外のリソースへのアクセス
    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    //アサーションマクロ
    assert!(true);
    assert_eq!(1, 1);
    assert_ne!(1, 0);
    debug_assert!(false); //debugビルドのみで有効
    debug_assert_eq!(1, 1);
    debug_assert_ne!(1, 0);
}
