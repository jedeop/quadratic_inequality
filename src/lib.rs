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

#[derive(Debug, PartialEq)]
struct Quadratic {
    character: String,
    a: Number,
    b: Number,
    c: Number,
}
impl Quadratic {
    fn new((a, b, c): (Monomial, Monomial, Monomial)) -> Self {
        assert_eq!(a.character, b.character);

        assert_eq!(a.degree, Some(Number(2)));
        assert_eq!(b.degree, None);

        let character = match a.character {
            Some(character) => character,
            None => panic!("wrong quadratic"),
        };

        Self {
            character: character.to_string(),
            a: a.coefficient,
            b: b.coefficient,
            c: c.coefficient,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
