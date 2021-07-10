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

#[derive(Debug, PartialEq, Clone)]
enum Sign {
    Lt,
    Lte,
    Gt,
    Gte,
}
impl Sign {
    fn new(s: &str) -> Self {
        match s {
            "<" => Self::Lt,
            "<=" | "≤" => Self::Lte,
            ">" => Self::Gt,
            ">=" | "≥" => Self::Gte,
            _ => panic!("bad inequality sign"),
        }
    }
    fn reverse(&self) -> Self {
        match self {
            Self::Lt => Self::Gt,
            Self::Lte => Self::Gte,
            Self::Gt => Self::Lt,
            Self::Gte => Self::Lte,
        }
    }
}

#[derive(Debug, PartialEq)]
struct QuadraticInequality {
    quadratic: Quadratic,
    sign: Sign,
}
impl QuadraticInequality {
    fn new(input: (Quadratic, Sign)) -> Self {
        let (quadratic, sign) = input;
        Self { quadratic, sign }
    }
    fn get_solution(&self) -> String {
        let (s1, s2) = &self.quadratic.get_solution();
        let sign = if &self.quadratic.a > &0 {
            self.sign.clone()
        } else {
            self.sign.reverse()
        };
        let character = &self.quadratic.character;
        match sign {
            Sign::Lt => format!("{} < {} < {}", s1, character, s2),
            Sign::Lte => format!("{} ≤ {} ≤ {}", s1, character, s2),
            Sign::Gt => format!("{} < {} OR {} > {}", character, s1, character, s2),
            Sign::Gte => format!("{} ≤ {} OR {} ≥ {}", character, s1, character, s2),
        }
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

    #[test]
    fn get_solution_of_quadratic_inequality() {
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 5,
                    c: 4,
                },
                sign: Sign::Lt,
            }
            .get_solution(),
            "-4 < x < -1".to_string()
        );
    }
}
