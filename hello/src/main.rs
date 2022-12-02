mod async_move;
use futures::executor;

fn main() {
    executor::block_on(async_move::move_to_async_block());
}
