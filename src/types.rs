use std::{cmp::Ordering, ops::Add};

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Number(i32);
impl Number {
    pub(crate) fn new(input: i32) -> Self {
        Number(input)
    }
    pub(crate) fn new_with_default(input: Option<&str>, default: i32) -> Self {
        let num = match input {
            Some(degree) => match degree.parse::<i32>() {
                Ok(degree) => degree,
                Err(_) => match degree.chars().next() {
                    Some('-') => -default,
                    _ => default,
                },
            },
            None => default,
        };
        Number(num)
    }
    pub(crate) fn set_sign(&mut self, sign: &str) {
        let val = self.0.abs();
        self.0 = match sign {
            "-" => -val,
            _ => val,
        };
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Monomial<'a> {
    coefficient: Number,
    character: Option<&'a str>,
    degree: Number,
}
impl<'a> Monomial<'a> {
    pub(crate) fn new(coefficient: Number, character: Option<&'a str>, degree: Number) -> Self {
        Self {
            coefficient,
            character,
            degree,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Quadratic {
    character: String,
    a: i32,
    b: i32,
    c: i32,
}
impl Quadratic {
    #[cfg(test)]
    pub(crate) fn new(character: String, a: i32, b: i32, c: i32) -> Self {
        Self { character, a, b, c }
    }
    pub(crate) fn from_monomials(monomials: Vec<Monomial>) -> Result<Self> {
        let character: Option<&str> =
            monomials
                .iter()
                .fold(Ok(None), |old, Monomial { character: new, .. }| match new {
                    None => old,
                    Some(new) => match old {
                        Err(_) => old,
                        Ok(None) => Ok(Some(*new)),
                        Ok(Some(old)) => {
                            if *new != old {
                                Err(Error::InvalidCharacter {
                                    expected: old.to_string(),
                                    found: new.to_string(),
                                })
                            } else {
                                Ok(Some(old))
                            }
                        }
                    },
                })?;
        let (a, b, c) = monomials
            .iter()
            .try_fold((0, 0, 0), |(a, b, c), monomial| {
                let val = monomial.coefficient.0;
                match (monomial.character, monomial.degree) {
                    (Some(_), Number(2)) => Ok((a + val, b, c)),
                    (Some(_), Number(1)) => Ok((a, b + val, c)),
                    (None, Number(1)) => Ok((a, b, c + val)),
                    _ => Err(Error::InvalidQuadratic),
                }
            })?;
        Ok(Self {
            character: character.unwrap_or("").to_string(),
            a,
            b,
            c,
        })
    }
    fn reverse(&mut self) {
        self.a *= -1;
        self.b *= -1;
        self.c *= -1;
    }
    fn get_d(&self) -> i32 {
        self.b.pow(2) - 4 * &self.a * self.c
    }
    fn get_solution(&self) -> (f32, f32) {
        let solution1 = ((-self.b) as f32 - ((self.b.pow(2) - 4 * self.a * self.c) as f32).sqrt())
            / (2 * &self.a) as f32;
        let solution2 = ((-self.b) as f32 + ((self.b.pow(2) - 4 * self.a * self.c) as f32).sqrt())
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
pub(crate) enum Sign {
    Lt,
    Lte,
    Gt,
    Gte,
}
impl Sign {
    pub(crate) fn new(s: &str) -> Result<Self> {
        match s {
            "<" => Ok(Self::Lt),
            "<=" | "≤" => Ok(Self::Lte),
            ">" => Ok(Self::Gt),
            ">=" | "≥" => Ok(Self::Gte),
            k => Err(Error::InvalidIneqSign(k.to_string())),
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
pub(crate) struct QuadraticInequality {
    quadratic: Quadratic,
    sign: Sign,
}
impl QuadraticInequality {
    #[cfg(test)]
    pub(crate) fn new(quadratic: Quadratic, sign: Sign) -> Self {
        Self { quadratic, sign }
    }
    pub(crate) fn from_expr(input: (Quadratic, Sign, Quadratic)) -> Self {
        let (left, sign, mut right) = input;
        right.reverse();
        Self {
            quadratic: left + right,
            sign,
        }
    }
    pub(crate) fn get_solution(&self) -> String {
        let d = self.quadratic.get_d();
        let (s1, s2) = &self.quadratic.get_solution();
        let sign = if self.quadratic.a > 0 {
            self.sign.clone()
        } else {
            self.sign.reverse()
        };
        let character = &self.quadratic.character;
        match d.cmp(&0) {
            Ordering::Less => match sign {
                Sign::Lt | Sign::Lte => "no solution".to_string(),
                Sign::Gt | Sign::Gte => "all real number".to_string(),
            },
            Ordering::Greater => match sign {
                Sign::Lt => format!("{} < {} < {}", s1, character, s2),
                Sign::Lte => format!("{} ≤ {} ≤ {}", s1, character, s2),
                Sign::Gt => format!("{} < {} OR {} > {}", character, s1, character, s2),
                Sign::Gte => format!("{} ≤ {} OR {} ≥ {}", character, s1, character, s2),
            },
            Ordering::Equal => match sign {
                Sign::Lt => "no solution".to_string(),
                Sign::Lte => format!("{} = {}", character, s1),
                Sign::Gt => format!("all real number with {} ≠ {}", character, s1),
                Sign::Gte => "all real number".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_quadratic() {
        assert_eq!(
            Quadratic::from_monomials(vec![
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
            Ok(Quadratic {
                character: "x".to_string(),
                a: 1,
                b: 8,
                c: 4,
            })
        );
    }
    #[test]
    fn new_quadratic_2() {
        assert_eq!(
            Quadratic::from_monomials(vec![
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
            Ok(Quadratic {
                character: "x".to_string(),
                a: 1,
                b: 5,
                c: 4,
            })
        );
    }

    #[test]
    fn new_quadratic_wrong_character() {
        assert_eq!(
            Quadratic::from_monomials(vec![
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
            ]),
            Err(Error::InvalidCharacter {
                expected: "x".to_string(),
                found: "y".to_string()
            })
        );
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
            QuadraticInequality::from_expr((
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
