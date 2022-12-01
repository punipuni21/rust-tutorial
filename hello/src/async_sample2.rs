use futures::future::Future;
use std::process::Output;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn something_greet_async_function() -> impl Future<Output = i32> {
    async {
        let ans = async_add(2, 3).await;
        println!("{}", ans);
        ans
    }
}
