use std::env;

fn main() -> Result<(), client::MainError> {
    let args: Vec<String> = env::args().collect();
    client::Client::new_and_run(&args[1..])
}
