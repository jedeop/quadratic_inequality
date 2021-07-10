mod parser;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Number(i32);
impl Number {
    fn new(input: &str) -> Self {
        Number(input.parse::<i32>().unwrap())
    }
    fn new_with_default(input: &str, default: i32) -> Self {
        Number(input.parse::<i32>().unwrap_or(default))
    }
}

#[derive(Debug, PartialEq)]
struct Monomial<'a> {
    sign: Sign,
    coefficient: Number,
    character: Option<&'a str>,
    degree: Option<Number>,
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}
impl Sign {
    fn new(input: &str) -> Self {
        match input {
            "+" | "" => Self::Plus,
            "-" => Self::Minus,
            _ => panic!("wrong operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {}
}
