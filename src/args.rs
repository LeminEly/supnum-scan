use std::env;

pub fn get_target() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: supnum <IP>");
        std::process::exit(1);
    }

    args[1].clone()
}
