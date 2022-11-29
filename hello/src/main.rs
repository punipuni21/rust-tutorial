use future_sample::CountDown;
use futures::{executor, future::join_all};

mod future_sample;

fn main() {
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}:{}", i, s);
    }
}
