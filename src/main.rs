use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("only one argument needed: {}", args.join(", "));
        process::exit(1);
    }
    let input = &args[1];
    println!("{}", input);
    println!("{}", quadratic_inequality::solve(input));
}
