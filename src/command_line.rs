

pub fn command_line() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }

    let args: Vec<String> = std::env::args().skip(0).collect();
    if args.len() > 0 { // we have args!
        println!("{:?}", args);
    }
}