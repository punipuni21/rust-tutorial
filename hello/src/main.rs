use async_sample::find_by_user_id;
use futures::executor;
mod async_sample;

fn main() {
    executor::block_on(find_by_user_id(Db {}, UserId(u1)));
}
