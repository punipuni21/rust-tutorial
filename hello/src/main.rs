use tweet::Tweet;

mod animal;
mod tweet;

fn main() {
    let dove = animal::Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = animal::Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }
}
