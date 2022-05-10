use astd::task;
use procks::Procks;
use std::env;
use std::process;

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    return task::block_on(
        async {
            if let Ok(mut procks) = Procks::from_iter(env::args()) {
                if let Ok(_) = procks.run().await {
                    return 0;
                }
            }
            return 1;
        }
    );
}
