mod async_sample2;
use futures::executor;
mod future_sample;

fn main() {
    executor::block_on(async_sample2::something_greet_async_function());
}
