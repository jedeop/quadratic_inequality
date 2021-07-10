mod parser;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Number(i32);
impl Number {
    fn new(input: &str) -> Self {
        println!("number: {}", input);
        Number(input.parse::<i32>().unwrap())
    }
    fn new_with_default(input: &str, default: i32) -> Self {
        println!("number: {}", input);
        Number(input.parse::<i32>().unwrap_or_else(|_| {
            if input.chars().next() == Some('-') {
                -1 * default
            } else {
                default
            }
        }))
    }
}

#[derive(Debug, PartialEq)]
struct Monomial<'a> {
    coefficient: Number,
    character: Option<&'a str>,
    degree: Option<Number>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {}
}
