use std::ops::Add;

pub mod parser;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Number(i32);
impl Number {
    fn new_with_default(input: Option<&str>, default: i32) -> Self {
        let num = match input {
            Some(degree) => match degree.parse::<i32>() {
                Ok(degree) => degree,
                Err(_) => match degree.chars().next() {
                    Some('-') => -1 * default,
                    _ => default,
                },
            },
            None => default,
        };
        Number(num)
    }
    fn set_sign(&mut self, sign: &str) {
        let val = self.0.abs();
        self.0 = match sign {
            "-" => -1 * val,
            _ => val,
        };
    }
}

#[derive(Debug, PartialEq)]
struct Monomial<'a> {
    coefficient: Number,
    character: Option<&'a str>,
    degree: Number,
}

#[derive(Debug, PartialEq)]
struct Quadratic {
    character: String,
    a: i32,
    b: i32,
    c: i32,
}
impl Quadratic {
    fn new(monomials: Vec<Monomial>) -> Self {
        let mut character = None;
        monomials.iter().for_each(|m| {
            if m.character.is_none() {
                return;
            }
            if character.is_none() {
                character = m.character
            } else if m.character != character {
                panic!("quadratic character is confused")
            }
        });
        let (a, b, c) = monomials.iter().fold((0, 0, 0), |(a, b, c), monomial| {
            let val = monomial.coefficient.0;
            match (monomial.character, monomial.degree) {
                (Some(_), Number(2)) => (a + val, b, c),
                (Some(_), Number(1)) => (a, b + val, c),
                (None, Number(1)) => (a, b, c + val),
                _ => panic!("not quadratic"),
            }
        });
        Self {
            character: character.unwrap_or("").to_string(),
            a,
            b,
            c,
        }
    }
    fn reverse(&mut self) {
        self.a = -1 * self.a;
        self.b = -1 * self.b;
        self.c = -1 * self.c;
    }
    fn get_d(&self) -> i32 {
        &self.b.pow(2) - 4 * &self.a * self.c
    }
    fn get_solution(&self) -> (f32, f32) {
        let solution1 = ((-1 * &self.b) as f32
            - ((&self.b.pow(2) - 4 * &self.a * self.c) as f32).sqrt())
            / (2 * &self.a) as f32;
        let solution2 = ((-1 * &self.b) as f32
            + ((&self.b.pow(2) - 4 * &self.a * self.c) as f32).sqrt())
            / (2 * &self.a) as f32;

        if solution1 < solution2 {
            (solution1, solution2)
        } else {
            (solution2, solution1)
        }
    }
}

impl Add for Quadratic {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            character: self.character,
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
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
pub struct QuadraticInequality {
    quadratic: Quadratic,
    sign: Sign,
}
impl QuadraticInequality {
    fn new(input: (Quadratic, Sign, Quadratic)) -> Self {
        let (left, sign, mut right) = input;
        right.reverse();
        Self {
            quadratic: left + right,
            sign,
        }
    }
    pub fn get_solution(&self) -> String {
        let d = self.quadratic.get_d();
        let (s1, s2) = &self.quadratic.get_solution();
        let sign = if self.quadratic.a > 0 {
            self.sign.clone()
        } else {
            self.sign.reverse()
        };
        let character = &self.quadratic.character;
        if d < 0 {
            match sign {
                Sign::Lt | Sign::Lte => "no solution".to_string(),
                Sign::Gt | Sign::Gte => "all real number".to_string(),
            }
        } else if d > 0 {
            match sign {
                Sign::Lt => format!("{} < {} < {}", s1, character, s2),
                Sign::Lte => format!("{} ≤ {} ≤ {}", s1, character, s2),
                Sign::Gt => format!("{} < {} OR {} > {}", character, s1, character, s2),
                Sign::Gte => format!("{} ≤ {} OR {} ≥ {}", character, s1, character, s2),
            }
        } else {
            match sign {
                Sign::Lt => "no solution".to_string(),
                Sign::Lte => format!("{} = {}", character, s1),
                Sign::Gt => format!("all real number with {} ≠ {}", character, s1),
                Sign::Gte => "all real number".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_quadratic() {
        assert_eq!(
            Quadratic::new(vec![
                Monomial {
                    coefficient: Number(1),
                    character: Some("x"),
                    degree: Number(2)
                },
                Monomial {
                    coefficient: Number(5),
                    character: Some("x"),
                    degree: Number(1)
                },
                Monomial {
                    coefficient: Number(4),
                    character: None,
                    degree: Number(1)
                },
                Monomial {
                    coefficient: Number(3),
                    character: Some("x"),
                    degree: Number(1)
                },
            ]),
            Quadratic {
                character: "x".to_string(),
                a: 1,
                b: 8,
                c: 4,
            }
        );
    }
    #[test]
    fn new_quadratic_2() {
        assert_eq!(
            Quadratic::new(vec![
                Monomial {
                    coefficient: Number(1),
                    character: Some("x"),
                    degree: Number(2)
                },
                Monomial {
                    coefficient: Number(5),
                    character: Some("x"),
                    degree: Number(1)
                },
                Monomial {
                    coefficient: Number(4),
                    character: None,
                    degree: Number(1)
                },
            ]),
            Quadratic {
                character: "x".to_string(),
                a: 1,
                b: 5,
                c: 4,
            }
        );
    }

    #[test]
    #[should_panic]
    fn new_quadratic_wrong_character() {
        Quadratic::new(vec![
            Monomial {
                coefficient: Number(1),
                character: Some("x"),
                degree: Number(2),
            },
            Monomial {
                coefficient: Number(5),
                character: Some("y"),
                degree: Number(1),
            },
            Monomial {
                coefficient: Number(4),
                character: None,
                degree: Number(0),
            },
        ]);
    }

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
    fn new_quadratic_inequality() {
        assert_eq!(
            QuadraticInequality::new((
                Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 5,
                    c: 2,
                },
                Sign::Lt,
                Quadratic {
                    character: "x".to_string(),
                    a: -1,
                    b: 0,
                    c: -2,
                }
            )),
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 2,
                    b: 5,
                    c: 4,
                },
                sign: Sign::Lt,
            }
        );
    }

    #[test]
    fn add_quadratic() {
        let left = Quadratic {
            character: "x".to_string(),
            a: 1,
            b: 3,
            c: 2,
        };

        let right = Quadratic {
            character: "x".to_string(),
            a: 3,
            b: 1,
            c: -3,
        };
        let result = Quadratic {
            character: "x".to_string(),
            a: 4,
            b: 4,
            c: -1,
        };

        assert_eq!(left + right, result);
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

    #[test]
    fn get_solution_of_quadratic_inequality_a_lt_0() {
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: -1,
                    b: 5,
                    c: -4,
                },
                sign: Sign::Lt,
            }
            .get_solution(),
            "x < 1 OR x > 4".to_string()
        );
    }

    #[test]
    fn get_special_solutions() {
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 4,
                },
                sign: Sign::Lte,
            }
            .get_solution(),
            "x = -2".to_string()
        );
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 4,
                },
                sign: Sign::Gte,
            }
            .get_solution(),
            "all real number".to_string()
        );
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 5,
                },
                sign: Sign::Lt,
            }
            .get_solution(),
            "no solution".to_string()
        );
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 5,
                },
                sign: Sign::Gt,
            }
            .get_solution(),
            "all real number".to_string()
        );
        assert_eq!(
            QuadraticInequality {
                quadratic: Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 5,
                },
                sign: Sign::Lte,
            }
            .get_solution(),
            "no solution".to_string()
        );
    }
}
