use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char, digit1},
    combinator::{map, opt},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{Monomial, Number, Quadratic, QuadraticInequality, Sign};

fn coefficient(input: &str) -> IResult<&str, Number> {
    map(
        take_while(|c| matches!(c, '0'..='9' | '+' | '-')),
        |s: &str| Number::new_with_default(s, 1),
    )(input)
}
fn character(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
fn degree(input: &str) -> IResult<&str, Number> {
    map(preceded(char('^'), digit1), Number::new)(input)
}

fn monomial(input: &str) -> IResult<&str, Monomial> {
    map(
        tuple((coefficient, opt(character), opt(degree))),
        |(coefficient, character, degree)| Monomial {
            coefficient,
            character,
            degree,
        },
    )(input)
}

fn quadratic(input: &str) -> IResult<&str, Quadratic> {
    map(tuple((monomial, monomial, monomial)), Quadratic::new)(input)
}

fn sign(input: &str) -> IResult<&str, Sign> {
    map(
        alt((tag("<"), tag("<="), tag("≤"), tag(">"), tag(">="), tag("≥"))),
        Sign::new,
    )(input)
}

fn quadratic_inequality(input: &str) -> IResult<&str, QuadraticInequality> {
    map(tuple((quadratic, sign)), QuadraticInequality::new)(input)
}

pub fn parse(input: &str) -> QuadraticInequality {
    quadratic_inequality(input).expect("can't parse input").1
}

#[cfg(test)]
mod tests {
    use crate::QuadraticInequality;

    use super::*;

    #[test]
    fn parse_degree() {
        assert_eq!(degree("^3"), Ok(("", Number(3))))
    }

    #[test]
    fn parse_monomial() {
        assert_eq!(
            monomial("+x^2"),
            Ok((
                "",
                Monomial {
                    coefficient: Number(1),
                    character: Some("x"),
                    degree: Some(Number(2))
                }
            ))
        );
        assert_eq!(
            monomial("-x^4"),
            Ok((
                "",
                Monomial {
                    coefficient: Number(-1),
                    character: Some("x"),
                    degree: Some(Number(4))
                }
            ))
        );
    }

    #[test]
    fn parse_monomial_only_variable() {
        assert_eq!(
            monomial("x"),
            Ok((
                "",
                (Monomial {
                    coefficient: Number(1),
                    character: Some("x"),
                    degree: None,
                })
            ))
        );
    }
    #[test]
    fn parse_monomial_const() {
        assert_eq!(
            monomial("2"),
            Ok((
                "",
                Monomial {
                    coefficient: Number(2),
                    character: None,
                    degree: None,
                }
            ))
        );
    }

    #[test]
    fn parse_quadratic() {
        assert_eq!(
            quadratic("x^2+4x+5"),
            Ok((
                "",
                Quadratic {
                    character: "x".to_string(),
                    a: 1,
                    b: 4,
                    c: 5,
                }
            ))
        );
    }

    #[test]
    fn parse_quadratic_inequality() {
        assert_eq!(
            quadratic_inequality("x^2+3x-10<0"),
            Ok((
                "0",
                QuadraticInequality {
                    quadratic: Quadratic {
                        character: "x".to_string(),
                        a: 1,
                        b: 3,
                        c: -10,
                    },
                    sign: Sign::Lt,
                }
            ))
        );
    }

    #[test]
    fn parse_and_get_solution_of_quadratic_inequality() {
        assert_eq!(
            parse("x^2+3x-10≥0").get_solution(),
            "x ≤ -5 OR x ≥ 2".to_string(),
        );
    }
}
