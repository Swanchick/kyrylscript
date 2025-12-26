use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let path = args.get(1);

    if let Some(path) = path {}
}
