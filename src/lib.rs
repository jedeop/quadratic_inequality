mod parser;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Number(i32);
impl Number {
    fn new(input: &str) -> Self {
        Number(input.parse::<i32>().unwrap())
    }
    fn new_with_default(input: &str, default: i32) -> Self {
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
    a: i32,
    b: i32,
    c: i32,
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
            a: a.coefficient.0,
            b: b.coefficient.0,
            c: c.coefficient.0,
        }
    }
    fn get_solution(&self) -> (f32, f32) {
        println!("{:#?}", &self);
        let solution1 = ((-1 * &self.b) as f32
            - ((&self.b.pow(2) - 4 * &self.a * self.c) as f32).sqrt())
            / (2 * &self.a) as f32;
        let solution2 = ((-1 * &self.b) as f32
            + ((&self.b.pow(2) - 4 * &self.a * self.c) as f32).sqrt())
            / (2 * &self.a) as f32;

        (solution1, solution2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_solution_of_quadratic() {
        assert_eq!(
            Quadratic {
                character: "x".to_string(),
                a: 1,
                b: 5,
                c: 4,
            }
            .get_solution(),
            (-4.0, -1.0)
        );
    }
}
