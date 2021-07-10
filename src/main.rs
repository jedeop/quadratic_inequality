use std::{env, process};

use quadratic_inequality::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("only one argument needed: {}", args.join(", "));
        process::exit(1);
    }
    let input = &args[1];
    println!("{}", input);
    println!("{}", parse(input).get_solution());
}
